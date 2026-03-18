use basic::sync::OnceGet;
use mem::PhysAddr;
use platform;
use x86_64::registers::model_specific::Msr;

use crate::{plic_domain, task_domain, timer};

use super::{
    context::{fault_address, X86TrapFrame, X86TrapFrameExt}, gdt, idt, syscall, vector,
};

unsafe extern "C" {
    fn x86_trampoline_return(user_cr3: usize, trap_cx_ptr: usize) -> !;
}

const MSR_KERNEL_GS_BASE: u32 = 0xC000_0102;

#[unsafe(no_mangle)]
pub extern "C" fn user_trap_vector() {
    // 从用户入口回到内核分发前，先切回内核 IDT。
    idt::set_kernel_trap_entry();

    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap();
    let frame = basic::task::TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));
    user_trap_handler(frame);
}

#[unsafe(no_mangle)]
pub fn kernel_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_user() {
        panic!("kernel_trap_handler: received user-mode trap");
    }
    assert!(
        !arch::is_interrupt_enable(),
        "Interrupts should be disabled in kernel trap handler"
    );
    handle_trap(frame, false);
}

/// 汇编统一入口分发函数
#[unsafe(no_mangle)]
pub extern "C" fn x86_trap_dispatch(frame: &mut X86TrapFrame) {
    if frame.is_user() {
        user_trap_handler(frame);
    } else {
        kernel_trap_handler(frame);
    }
}

pub fn user_trap_handler(frame: &mut X86TrapFrame) {
    // 进入内核处理后，统一使用内核 IDT。
    idt::set_kernel_trap_entry();

    // 保存用户态 FPU/SSE 状态。
    frame.save_fx_state();

    handle_trap(frame, true);
    trap_return();
}

fn handle_trap(frame: &mut X86TrapFrame, from_user: bool) {
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
            if from_user {
                task_domain!()
                    .do_load_page_fault(fault_addr)
                    .expect("do_load_page_fault failed");
                log::debug!(
                    "Page fault handled: addr={:#x}, RIP={:#x}",
                    fault_addr,
                    frame.rip
                );
            } else {
                panic!(
                    "Kernel page fault at RIP={:#x}, fault_addr={:#x}, error={:#x}",
                    frame.rip, fault_addr, frame.error_code
                );
            }
        }
        vector::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vector::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            if from_user {
                crate::task::yield_now();
            }
            send_apic_eoi();
        }

        vector::SYSCALL => {
            if from_user {
                // 兼容入口：用户态若仍触发 int 0x80，复用同一套分发逻辑。
                syscall::handle_legacy_syscall();
            } else {
                panic!("syscall from kernel mode");
            }
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

#[inline]
fn send_apic_eoi() {
    platform::apic::eoi();
}

#[unsafe(no_mangle)]
pub fn trap_return() -> ! {
    // 回用户态前切换为用户入口 IDT，保证下次用户态 trap 先落 trampoline。
    idt::set_user_trap_entry();

    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap();
    let trap_frame = basic::task::TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));

    // 恢复用户态 FPU/SSE 状态（切换 CR3 之前，仍使用物理地址访问）。
    trap_frame.restore_fx_state();

    // TSS.rsp0 指向 TrapFrame 顶部，保证用户态 trap 先写入共享上下文页。
    gdt::write_tss_rsp0(trap_cx_ptr + basic::task::TrapFrame::USER_CONTEXT_SIZE);
    unsafe {
        Msr::new(MSR_KERNEL_GS_BASE).write(trap_cx_ptr as u64);
    }

    unsafe { x86_trampoline_return(user_cr3, trap_cx_ptr) }
}
