mod context;
mod gdt;
mod handler;
mod idt;
mod syscall;
mod user_ctx;
mod vectors;

use gdt::init_gdt;
pub use handler::{trap_return, user_trap_vector};
pub use idt::set_trap_entry;
use idt::init_idt;
use syscall::init_syscall;

#[inline]
pub fn write_tss_rsp0(rsp0: usize) {
    gdt::write_tss_rsp0(rsp0);
}

pub fn init_trap() {
    // x86_64: 先初始化 GDT/TSS，再装载 IDT，最后初始化 syscall MSR。
    init_gdt();
    init_idt();
    set_trap_entry();
    init_syscall();
}