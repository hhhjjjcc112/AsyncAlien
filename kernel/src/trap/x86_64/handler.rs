use core::{
    arch::asm,
};

use config::TRAMPOLINE;

#[cfg(feature = "trap_test")]
use super::test;
use super::{
    context::{fault_address, X86TrapFrame, X86TrapFrameExt},
    syscall,
    user_ctx::{current_trap_frame, prepare_user_return, UserTrapResult},
    vectors,
};
use crate::{
    task_domain, timer,
};

unsafe extern "C" {
    fn strampoline();
    fn x86_trampoline_return(user_cr3: usize, trap_cx_ptr: usize) -> !;
}

/// 统计用户态APIC timer中断触发次数
#[cfg(feature = "apic_timer_test")]
static APIC_TIMER_USER_TRAP_COUNT: core::sync::atomic::AtomicUsize =
    core::sync::atomic::AtomicUsize::new(0);
/// 统计内核态APIC timer中断触发次数
#[cfg(feature = "apic_timer_test")]
static APIC_TIMER_KERNEL_TRAP_COUNT: core::sync::atomic::AtomicUsize =
    core::sync::atomic::AtomicUsize::new(0);

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

    match vec {
        vectors::DIVIDE_ERROR => panic!("Divide error at RIP={:#x}", frame.rip),
        vectors::DEBUG => log::debug!("Debug exception at RIP={:#x}", frame.rip),
        vectors::BREAKPOINT => {
            #[cfg(feature = "trap_test")]
            test::record_breakpoint(frame.rip);
            #[cfg(feature = "trap_test")]
            println!("[trap_test] breakpoint entered at RIP={:#x}", frame.rip);
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
            match task_domain!().do_load_page_fault(fault_addr) {
                Ok(()) => {}
                Err(err) => {
                    panic!(
                        "do_load_page_fault failed: addr={:#x}, rip={:#x}, rsp={:#x}, err_code={:#x}, err={:?}",
                        fault_addr, frame.rip, frame.rsp, frame.error_code, err
                    );
                }
            }
        }
        vectors::DOUBLE_FAULT => panic!("Double fault! RIP={:#x}", frame.rip),

        vectors::APIC_TIMER => {
            #[cfg(feature = "apic_timer_test")]
            {
                let _ = APIC_TIMER_USER_TRAP_COUNT
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            }
            crate::vdso::refresh_time_snapshot();
            handle_local_apic_timer(timer::next_trigger_deadline());
            crate::task::yield_now();
        }

        vectors::SYSCALL => {
            // 兼容入口：int 0x80 从用户态触发
            syscall::handle_legacy_syscall(frame);
        }

        v if (vectors::IRQ_BASE..vectors::APIC_TIMER).contains(&v) => {
            let irq = (v - vectors::IRQ_BASE) as usize;
            if let Some(apic) = crate::trap::INTERRUPT_CONTROLLER_DOMAIN.get() {
                apic.handle_irq(irq).expect("handle_irq failed");
            }
            send_apic_eoi();
        }

        vectors::APIC_ERROR => {
            let local_apic = crate::trap::LOCAL_APIC_DOMAIN
                .get()
                .expect("local_apic domain not registered");
            let error_status = local_apic
                .get_error_status()
                .unwrap_or(0);
            panic!(
                "APIC Error (user mode): error_status={:#x} (send_checksum={} recv_checksum={} send_accept={} recv_accept={} illegal_vector={})",
                error_status,
                (error_status & 0x01) != 0,
                (error_status & 0x02) != 0,
                (error_status & 0x04) != 0,
                (error_status & 0x08) != 0,
                (error_status & 0x80) != 0
            );
        }

        vectors::APIC_SPURIOUS => {
            // Per Intel SDM, spurious interrupts should be ignored and NOT serviced.
            // No EOI needed; CPU will auto-deassert the interrupt signal.
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
            #[cfg(feature = "trap_test")]
            test::record_breakpoint(frame.rip);
            #[cfg(feature = "trap_test")]
            println!("[trap_test] breakpoint entered at RIP={:#x}", frame.rip);
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
            #[cfg(feature = "apic_timer_test")]
            {
                let _ = APIC_TIMER_KERNEL_TRAP_COUNT
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            }
            crate::vdso::refresh_time_snapshot();
            handle_local_apic_timer(timer::next_trigger_deadline());
        }

        vectors::SYSCALL => panic!("syscall from kernel mode"),

        v if (vectors::IRQ_BASE..vectors::APIC_TIMER).contains(&v) => {
            let irq = (v - vectors::IRQ_BASE) as usize;
            if let Some(apic) = crate::trap::INTERRUPT_CONTROLLER_DOMAIN.get() {
                apic.handle_irq(irq).expect("handle_irq failed");
            }
            send_apic_eoi();
        }

        vectors::APIC_ERROR => {
            let local_apic = crate::trap::LOCAL_APIC_DOMAIN
                .get()
                .expect("local_apic domain not registered");
            let error_status = local_apic
                .get_error_status()
                .unwrap_or(0);
            panic!(
                "APIC Error (kernel mode): error_status={:#x} (send_checksum={} recv_checksum={} send_accept={} recv_accept={} illegal_vector={})",
                error_status,
                (error_status & 0x01) != 0,
                (error_status & 0x02) != 0,
                (error_status & 0x04) != 0,
                (error_status & 0x08) != 0,
                (error_status & 0x80) != 0
            );
        }

        vectors::APIC_SPURIOUS => {
            // Per Intel SDM, spurious interrupts should be ignored and NOT serviced.
            // No EOI needed; CPU will auto-deassert the interrupt signal.
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
    let local_apic = crate::trap::LOCAL_APIC_DOMAIN
        .get()
        .expect("local_apic domain not registered");
    local_apic.eoi().expect("local_apic eoi failed");
}

fn handle_local_apic_timer(next_deadline: usize) {
    let local_apic = crate::trap::LOCAL_APIC_DOMAIN
        .get()
        .expect("local_apic domain not registered");
    // platform::println!(
    //     "[x86_64][apic_timer] local_apic set_timer enter next_deadline={:#x}",
    //     next_deadline
    // );
    local_apic
        .set_timer(next_deadline)
        .expect("local_apic set_timer failed");
    // platform::println!(
    //     "[x86_64][apic_timer] local_apic set_timer ok next_deadline={:#x}",
    //     next_deadline
    // );
    local_apic.eoi().expect("local_apic eoi failed");
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_return() -> ! {
    // 该入口用于任务上下文首次/再次返回用户态，不走常规函数返回。
    let UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    } = prepare_user_return();
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
