use core::arch::asm;

use basic::sync::OnceGet;
use config::TRAMPOLINE;
use riscv::register::{
    scause::Trap,
    sepc, sscratch, sstatus,
    sstatus::SPP,
    stvec,
    stvec::TrapMode,
};

use crate::task_domain;

use super::cause::TrapHandler;

unsafe extern "C" {
    fn kernel_v();
    fn user_v();
    fn user_r();
}

/// 设置内核态 trap 入口（stvec + sscratch）
#[inline]
pub fn set_kernel_trap_entry() {
    unsafe {
        sscratch::write(kernel_trap_vector as *const () as usize);
        stvec::write(kernel_v as *const () as usize, TrapMode::Direct);
    }
}

/// 设置用户态 trap 入口（trampoline）
#[inline]
pub fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn kernel_trap_vector(sp: usize) {
    let sstatus = sstatus::read();
    if sstatus.spp() == SPP::User {
        panic!("kernel_trap_vector: spp == SPP::User");
    }
    assert!(
        !arch::is_interrupt_enable(),
        "Interrupts should be disabled in kernel trap handler"
    );

    let cause = riscv::register::scause::read().cause();
    cause.do_kernel_handle(sp)
}

#[unsafe(no_mangle)]
pub fn user_trap_vector() {
    let sstatus = sstatus::read();
    if sstatus.spp() == SPP::Supervisor {
        panic!("user_trap_vector: spp == SPP::Supervisor");
    }

    set_kernel_trap_entry();
    let cause: Trap = riscv::register::scause::read().cause();
    cause.do_user_handle();
    trap_return();
}

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
