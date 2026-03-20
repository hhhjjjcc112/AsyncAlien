use core::{arch::asm, task};

use basic::task::TrapFrame;
use mem::PhysAddr;
use platform;

use crate::{plic_domain, task_domain, timer};

use super::{
    context::{fault_address, X86TrapFrame, X86TrapFrameExt}, gdt, syscall, vector,
};

unsafe extern "C" {
    fn x86_trampoline_return(user_cr3: usize, trap_cx_ptr: usize) -> !;
}

/// 内核态 trap 处理函数
/// 由 trampoline.asm 中的 .Ltrap_common 直接调用
/// 参数 rdi: 指向 TrapFrame 的指针
/// 返回后汇编自动 ret，继续执行恢复寄存器并 iretq
#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_user() {
        panic!("kernel_trap_handler: received user-mode trap");
    }
    assert!(
        !arch::is_interrupt_enable(),
        "Interrupts should be disabled in kernel trap handler"
    );
    handle_kernel_trap(frame);
    // 函数返回，汇编自动 ret
}

#[repr(C)]
pub struct UserTrapResult {
    pub user_cr3: usize,
    pub trap_cx_ptr: usize,
}

/// 用户态 trap 处理函数
/// 由 trampoline.asm 中的 .Lfrom_user 直接调用
/// 参数 rdi: 指向 TrapFrame 的指针
/// 返回后汇编自动 ret，继续执行在 trampoline 中的恢复代码
#[unsafe(no_mangle)]
pub extern "C" fn user_trap_vector() -> UserTrapResult {
    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().expect("user_trap_vector: no trap frame for current task");
    let frame = X86TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));

    handle_user_trap(frame);

    // 在返回前设置 TSS.rsp0，使下次 trap 时 CPU 能正确栈帧
    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();

    // 更新 TSS.rsp0
    gdt::write_tss_rsp0(trap_cx_ptr + basic::task::TrapFrame::USER_CONTEXT_SIZE);

    // 通过指定寄存器返回 user_cr3 和 trap_cx_ptr，供 trampoline 恢复用户态使用
    // 使用了rax和rdx寄存器
    UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    }
}

fn handle_user_trap(frame: &mut X86TrapFrame) {
    let vec = frame.vector as u8;

    match vec {
        vector::DIVIDE_ERROR => panic!("Divide error at RIP={:#x}", frame.rip),
        vector::DEBUG => log::debug!("Debug exception at RIP={:#x}", frame.rip),
        vector::BREAKPOINT => log::debug!("Breakpoint at RIP={:#x}", frame.rip),
        vector::INVALID_OPCODE => panic!("Invalid opcode at RIP={:#x}", frame.rip),
        vector::GENERAL_PROTECTION => {
            panic!(
                "General protection fault at RIP={:#x}, error_code={:#x}",
                frame.rip, frame.error_code
            );
        }
        vector::PAGE_FAULT => {
            let fault_addr = fault_address();
            task_domain!()
                .do_load_page_fault(fault_addr)
                .expect("do_load_page_fault failed");
            log::debug!(
                "Page fault handled: addr={:#x}, RIP={:#x}",
                fault_addr,
                frame.rip
            );
        }
        vector::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vector::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            crate::task::yield_now();
            send_apic_eoi();
        }

        vector::SYSCALL => {
            // 兼容入口：int 0x80 从用户态触发
            syscall::handle_legacy_syscall(frame);
        }

        v if (vector::IRQ_BASE..vector::APIC_TIMER).contains(&v) => {
            trace!("[{}] External interrupt: IRQ {}", arch::cpu_id(), v - vector::IRQ_BASE);
            plic_domain!().handle_irq().expect("handle_irq failed");
            send_apic_eoi();
        }

        vector::APIC_ERROR => {
            log::warn!("APIC error interrupt");
            send_apic_eoi();
        }

        vector::APIC_SPURIOUS => {
            log::warn!("Spurious APIC interrupt");
        }

        _ => {
            panic!(
                "Unhandled trap: vector={}, RIP={:#x}, error={:#x}",
                vec, frame.rip, frame.error_code
            );
        }
    }
}

fn handle_kernel_trap(frame: &mut X86TrapFrame) {
    let vec = frame.vector as u8;

    match vec {
        vector::DIVIDE_ERROR => panic!("Divide error at RIP={:#x}", frame.rip),
        vector::DEBUG => log::debug!("Debug exception at RIP={:#x}", frame.rip),
        vector::BREAKPOINT => log::debug!("Breakpoint at RIP={:#x}", frame.rip),
        vector::INVALID_OPCODE => panic!("Invalid opcode at RIP={:#x}", frame.rip),
        vector::GENERAL_PROTECTION => {
            panic!(
                "General protection fault at RIP={:#x}, error_code={:#x}",
                frame.rip, frame.error_code
            );
        }
        vector::PAGE_FAULT => {
            let fault_addr = fault_address();
            panic!(
                "Kernel page fault at RIP={:#x}, fault_addr={:#x}, error={:#x}",
                frame.rip, fault_addr, frame.error_code
            );
        }
        vector::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vector::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            send_apic_eoi();
        }

        vector::SYSCALL => panic!("syscall from kernel mode"),

        v if (vector::IRQ_BASE..vector::APIC_TIMER).contains(&v) => {
            trace!("[{}] External interrupt: IRQ {}", arch::cpu_id(), v - vector::IRQ_BASE);
            plic_domain!().handle_irq().expect("handle_irq failed");
            send_apic_eoi();
        }

        vector::APIC_ERROR => {
            log::warn!("APIC error interrupt");
            send_apic_eoi();
        }

        vector::APIC_SPURIOUS => {
            log::warn!("Spurious APIC interrupt");
        }

        _ => {
            panic!(
                "Unhandled trap: vector={}, RIP={:#x}, error={:#x}",
                vec, frame.rip, frame.error_code
            );
        }
    }
}

#[inline]
fn send_apic_eoi() {
    platform::apic::eoi();
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_return() -> ! {
    // 该入口用于任务上下文首次/再次返回用户态，不走常规函数返回。
    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();
    unsafe {
        asm!(
            "jmp {ret}",
            ret = sym x86_trampoline_return,
            in("rdi") user_cr3,
            in("rsi") trap_cx_ptr,
            options(noreturn)
        )
    }
}
