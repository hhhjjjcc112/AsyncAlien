use basic::sync::OnceGet;
use core::arch::asm;
use mem::PhysAddr;
use platform;

use crate::{plic_domain, task_domain, timer};

use super::{gdt, syscall, vector, context::X86TrapFrame};

#[inline]
pub fn set_kernel_trap_entry() {
    // x86_64 下 IDT 常驻，无需像 RISC-V 那样切换 stvec。
}

#[inline]
pub fn set_user_trap_entry() {
    // x86_64 下用户态与内核态共享同一张 IDT。
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn user_trap_vector() {
    // 保留统一接口地址，实际 trap 进入点由 IDT 决定。
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

#[unsafe(no_mangle)]
pub fn user_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_kernel() {
        panic!("user_trap_handler: received kernel-mode trap");
    }
    set_kernel_trap_entry();
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
            let fault_addr = X86TrapFrame::fault_address();
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
    set_user_trap_entry();
    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap();
    let trap_frame = basic::task::TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));

    // 关键步骤：为后续用户态 -> 内核态切换准备内核栈与 TrapFrame 指针。
    gdt::write_tss_rsp0(trap_frame.kernel_sp().as_usize());
    unsafe {
        arch::write_msr(arch::MSR_KERNEL_GS_BASE, trap_cx_ptr as u64);
    }

    unsafe {
        asm!(
            "mov cr3, {cr3}",
            "mov rsp, {frame}",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbp",
            "pop rbx",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rsi",
            "pop rdi",
            "pop rdx",
            "pop rcx",
            "pop rax",
            "add rsp, 16",
            "iretq",
            cr3 = in(reg) user_cr3,
            frame = in(reg) trap_cx_ptr,
            options(noreturn)
        )
    }
}
