use core::arch::asm;
use core::sync::atomic::{AtomicUsize, Ordering};

use config::TRAMPOLINE;
use platform;

use crate::{task_domain, timer};
#[cfg(target_arch = "riscv64")]
use crate::plic_domain;
#[cfg(target_arch = "x86_64")]
use crate::apic_domain;

#[cfg(feature = "trap_self_test")]
use super::self_test;
use super::{
    context::{fault_address, X86TrapFrame, X86TrapFrameExt}, syscall,
    user_ctx::{current_trap_frame, prepare_user_return, UserTrapResult}, vectors,
};

unsafe extern "C" {
    fn strampoline();
    fn x86_trampoline_return(user_cr3: usize, trap_cx_ptr: usize) -> !;
}

static USER_TRAP_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);
static USER_RETURN_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// 内核态 trap 处理函数
/// 由 trampoline.asm 中的 .Ltrap_common 直接调用
/// 参数 rdi: 指向 TrapFrame 的指针
/// 返回后汇编自动 ret，继续执行恢复寄存器并 iretq
#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_user() {
        panic!("kernel_trap_handler: received user-mode trap");
    }
    handle_kernel_trap(frame);
    // 函数返回，汇编自动 ret
}

/// 用户态 trap 处理函数
/// 由 trampoline.asm 中的 .Lfrom_user 直接调用
/// 参数 rdi: 指向 TrapFrame 的指针
/// 返回后汇编自动 ret，继续执行在 trampoline 中的恢复代码
#[unsafe(no_mangle)]
pub extern "C" fn user_trap_vector() -> UserTrapResult {
    let frame = current_trap_frame();

    handle_user_trap(frame);

    // SysV ABI 下以 rax/rdx 返回 user_cr3 与 trap_cx_ptr。
    prepare_user_return()
}

fn handle_user_trap(frame: &mut X86TrapFrame) {
    let vec = frame.vector as u8;
    let trace_idx = USER_TRAP_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if trace_idx < 16 {
        log::warn!(
            "[x86 user trap] vec={}, rip={:#x}, err={:#x}",
            vec,
            frame.rip,
            frame.error_code
        );
    }

    match vec {
        vectors::DIVIDE_ERROR => panic!("Divide error at RIP={:#x}", frame.rip),
        vectors::DEBUG => log::debug!("Debug exception at RIP={:#x}", frame.rip),
        vectors::BREAKPOINT => {
            #[cfg(feature = "trap_self_test")]
            self_test::record_breakpoint(frame.rip);
            #[cfg(feature = "trap_self_test")]
            println!("[trap_self_test] breakpoint entered at RIP={:#x}", frame.rip);
            log::info!("Breakpoint at RIP={:#x}", frame.rip);
        }
        vectors::INVALID_OPCODE => panic!("Invalid opcode at RIP={:#x}", frame.rip),
        vectors::GENERAL_PROTECTION => {
            panic!(
                "General protection fault at RIP={:#x}, error_code={:#x}",
                frame.rip, frame.error_code
            );
        }
        vectors::PAGE_FAULT => {
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
        vectors::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vectors::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            crate::task::yield_now();
            send_apic_eoi();
        }

        vectors::SYSCALL => {
            // 兼容入口：int 0x80 从用户态触发
            syscall::handle_legacy_syscall(frame);
        }

        v if (vectors::IRQ_BASE..vectors::APIC_TIMER).contains(&v) => {
            trace!("[{}] External interrupt: IRQ {}", arch::cpu_id(), v - vectors::IRQ_BASE);
            let irq = (v - vectors::IRQ_BASE) as usize;
            apic_domain!().handle_irq(irq).expect("handle_irq failed");
            send_apic_eoi();
        }

        vectors::APIC_ERROR => {
            log::warn!("APIC error interrupt");
            send_apic_eoi();
        }

        vectors::APIC_SPURIOUS => {
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
        vectors::DIVIDE_ERROR => panic!("Divide error at RIP={:#x}", frame.rip),
        vectors::DEBUG => log::debug!("Debug exception at RIP={:#x}", frame.rip),
        vectors::BREAKPOINT => {
            #[cfg(feature = "trap_self_test")]
            self_test::record_breakpoint(frame.rip);
            #[cfg(feature = "trap_self_test")]
            println!("[trap_self_test] breakpoint entered at RIP={:#x}", frame.rip);
            log::info!("Breakpoint at RIP={:#x}", frame.rip);
        }
        vectors::INVALID_OPCODE => panic!("Invalid opcode at RIP={:#x}", frame.rip),
        vectors::GENERAL_PROTECTION => {
            panic!(
                "General protection fault at RIP={:#x}, error_code={:#x}",
                frame.rip, frame.error_code
            );
        }
        vectors::PAGE_FAULT => {
            let fault_addr = fault_address();
            panic!(
                "Kernel page fault at RIP={:#x}, fault_addr={:#x}, error={:#x}",
                frame.rip, fault_addr, frame.error_code
            );
        }
        vectors::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vectors::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            send_apic_eoi();
        }

        vectors::SYSCALL => panic!("syscall from kernel mode"),

        v if (vectors::IRQ_BASE..vectors::APIC_TIMER).contains(&v) => {
            trace!("[{}] External interrupt: IRQ {}", arch::cpu_id(), v - vectors::IRQ_BASE);
            #[cfg(target_arch = "x86_64")]
            {
                let irq = (v - vectors::IRQ_BASE) as usize;
                apic_domain!().handle_irq(irq).expect("handle_irq failed");
            }
            #[cfg(target_arch = "riscv64")]
            plic_domain!().handle_irq().expect("handle_irq failed");
            send_apic_eoi();
        }

        vectors::APIC_ERROR => {
            log::warn!("APIC error interrupt");
            send_apic_eoi();
        }

        vectors::APIC_SPURIOUS => {
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
    let (user_cr3, trap_cx_ptr) = task_domain
        .page_table_token_with_trap_frame_virt_addr()
        .unwrap();
    let trace_idx = USER_RETURN_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if trace_idx < 8 {
        log::warn!(
            "[x86 trap_return] user_cr3={:#x}, trap_cx={:#x}",
            user_cr3,
            trap_cx_ptr
        );
    }
    // 返回代码必须位于 trampoline 共享映射中，避免切到用户 CR3 后取指失败。
    let ret_va = x86_trampoline_return as *const () as usize - strampoline as *const () as usize
        + TRAMPOLINE;
    unsafe {
        asm!(
            "jmp {ret}",
            ret = in(reg) ret_va,
            in("rdi") user_cr3,
            in("rsi") trap_cx_ptr,
            options(noreturn)
        )
    }
}
