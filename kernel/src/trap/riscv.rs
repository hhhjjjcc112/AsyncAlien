//! RISC-V Trap handling
//!
//! This module implements trap handling for RISC-V architecture using native naming.

use core::arch::asm;

use basic::sync::OnceGet;
use config::TRAMPOLINE;
use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sepc, sscratch, sstatus,
    sstatus::SPP,
    stval, stvec,
    stvec::TrapMode,
};

use crate::{plic_domain, task_domain, timer};

// ============================================================================
// Assembly entry points (defined in .asm files)
// ============================================================================

unsafe extern "C" {
    fn kernel_v();
    fn user_v();
    fn user_r();
}

// ============================================================================
// Trap entry configuration
// ============================================================================

/// Set kernel trap entry point
/// 
/// Configures stvec to point to kernel exception vector and
/// stores kernel trap handler address in sscratch.
#[inline]
pub fn set_kernel_trap_entry() {
    unsafe {
        sscratch::write(kernel_trap_vector as *const () as usize);
        stvec::write(kernel_v as *const () as usize, TrapMode::Direct);
    }
}

/// Set user trap entry point
/// 
/// Configures stvec to point to trampoline page for user trap handling.
#[inline]
pub fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

// ============================================================================
// Trap handlers
// ============================================================================

/// Kernel trap handler
/// 
/// This function is called when a trap occurs in supervisor mode.
/// It should not re-enable interrupts to avoid nested traps.
#[unsafe(no_mangle)]
pub fn kernel_trap_vector(sp: usize) {
    let sstatus = sstatus::read();
    let spp = sstatus.spp();
    if spp == SPP::User {
        panic!("kernel_trap_vector: spp == SPP::User");
    }
    let enable = arch::is_interrupt_enable();
    assert!(!enable, "Interrupts should be disabled in kernel trap handler");
    
    let cause = riscv::register::scause::read().cause();
    cause.do_kernel_handle(sp)
}

/// User trap handler
/// 
/// This function is called when a trap occurs in user mode.
#[unsafe(no_mangle)]
pub fn user_trap_vector() {
    let sstatus = sstatus::read();
    let spp = sstatus.spp();
    if spp == SPP::Supervisor {
        panic!("user_trap_vector: spp == SPP::Supervisor");
    }
    
    set_kernel_trap_entry();
    let cause = riscv::register::scause::read();
    let cause = cause.cause();
    cause.do_user_handle();
    trap_return();
}

/// Return to user mode
/// 
/// Restores user context and returns to user space.
#[unsafe(no_mangle)]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let task_domain = task_domain!();
    let (user_satp, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();
    let restore_va = user_r as *const () as usize - user_v as *const () as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        )
    }
}

// ============================================================================
// Trap cause handling
// ============================================================================

pub trait TrapHandler {
    fn do_user_handle(&self);
    fn do_kernel_handle(&self, sp: usize);
}

impl TrapHandler for Trap {
    /// Handle trap from user mode
    fn do_user_handle(&self) {
        let stval = stval::read();
        let sepc = sepc::read();
        log::debug!("trap: {:?}", self);
        
        match self {
            Trap::Exception(Exception::UserEnvCall) => {
                super::exception::syscall_exception_handler();
            }
            Trap::Exception(Exception::StoreFault)
            | Trap::Exception(Exception::LoadFault)
            | Trap::Exception(Exception::InstructionFault)
            | Trap::Exception(Exception::IllegalInstruction) => {
                panic!(
                    "<do_user_handle> {:?} in application, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Exception(Exception::StorePageFault)
            | Trap::Exception(Exception::LoadPageFault) => {
                task_domain!()
                    .do_load_page_fault(stval)
                    .expect("do_load_page_fault failed");
                log::debug!(
                    "<do_user_handle> {:?}, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Exception(Exception::InstructionPageFault) => {
                panic!("<do_user_handle> instruction page fault")
            }
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                trace!("<do_user_handle> timer interrupt");
                timer::set_next_trigger();
                crate::task::yield_now();
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                trace!("[{}] <do_user_handle> external interrupt", arch::cpu_id());
                plic_domain!().handle_irq().expect("handle_irq failed");
            }
            _ => {
                panic!(
                    "unhandled trap: {:?}, stval: {:?}, sepc: {:x}",
                    self, stval, sepc
                );
            }
        }
    }

    /// Handle trap from kernel mode
    fn do_kernel_handle(&self, _sp: usize) {
        let stval = stval::read();
        let sepc = sepc::read();
        
        match self {
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                trace!("<do_kernel_handle> timer interrupt");
                timer::set_next_trigger()
            }
            Trap::Exception(_) => {
                panic!(
                    "[kernel] {:?} in kernel, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                platform::println!("<do_kernel_handle> external interrupt");
                plic_domain!().handle_irq().expect("handle_irq failed");
            }
            _ => {
                panic!(
                    "unhandled trap: {:?}, stval: {:?}, sepc: {:x}",
                    self, stval, sepc
                )
            }
        }
    }
}
