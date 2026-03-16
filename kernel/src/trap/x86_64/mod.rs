mod context;
mod gdt;
mod handler;
mod idt;
mod syscall;
mod vector;

use gdt::init_gdt;
pub use handler::{trap_return, user_trap_vector};
use idt::init_idt;
use syscall::init_syscall_registers;

pub fn init_trap() {
    // x86_64: 先初始化 GDT/TSS，再装载 IDT，最后初始化 syscall MSR。
    init_gdt();
    init_idt();
    init_syscall_registers();
}