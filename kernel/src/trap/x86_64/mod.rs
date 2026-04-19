mod context;
mod gdt;
mod handler;
mod idt;
#[cfg(feature = "trap_test")]
mod test;
mod syscall;
mod user_ctx;
mod vectors;

use gdt::init_gdt;
pub use handler::{trap_return, user_trap_vector};
#[cfg(feature = "trap_test")]
pub use test::run as run_trap_test;
use idt::init_idt;

pub(crate) use user_ctx::arm_user_return_trace;
pub(crate) use syscall::drain_syscall_entry_trace;
use crate::trap::x86_64::syscall::init_syscall;

#[inline]
pub fn write_tss_rsp0(rsp0: usize) {
    gdt::write_tss_rsp0(rsp0);
}

pub fn init_trap() {
    // x86_64: 先初始化 GDT/TSS，再装载 IDT，最后初始化 syscall MSR。
    init_gdt();
    init_idt();
    init_syscall();
}
