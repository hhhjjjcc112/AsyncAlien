//! Trap/Interrupt handling module
//!
//! This module provides architecture-specific trap handling:
//! - RISC-V: Uses scause, stvec, sscratch, sepc, stval registers
//! - x86-64: Uses IDT (Interrupt Descriptor Table), APIC, exception vectors
//!
//! Each architecture uses its native naming conventions while
//! exposing a unified public interface.

// Architecture-specific implementations
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

// Re-export architecture-specific items
// Common includes
use alloc::sync::Arc;
use core::arch::global_asm;

use basic::sync::Once;
#[cfg(target_arch = "x86_64")]
use interface::APICDomain as InterruptControllerDomain;
#[cfg(target_arch = "riscv64")]
use interface::PLICDomain as InterruptControllerDomain;
use interface::SysCallDomain;
use platform::println;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("./riscv64/kernel_v.asm"));
#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("./riscv64/trampoline.asm"));

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("./x86_64/trampoline.asm"));

pub static SYSCALL_DOMAIN: Once<Arc<dyn SysCallDomain>> = Once::new();
/// 当前架构唯一可见的中断控制器域。
pub static INTERRUPT_CONTROLLER_DOMAIN: Once<Arc<dyn InterruptControllerDomain>> = Once::new();

#[macro_export]
macro_rules! syscall_domain {
    () => {
        basic::sync::OnceGet::get_must(&$crate::trap::SYSCALL_DOMAIN)
    };
}

#[macro_export]
macro_rules! interrupt_controller_domain {
    () => {
        basic::sync::OnceGet::get_must(&$crate::trap::INTERRUPT_CONTROLLER_DOMAIN)
    };
}

pub fn register_syscall_domain(syscall_domain: Arc<dyn SysCallDomain>) {
    SYSCALL_DOMAIN.call_once(|| syscall_domain);
}

pub fn register_interrupt_controller_domain(
    interrupt_controller_domain: Arc<dyn InterruptControllerDomain>,
) {
    INTERRUPT_CONTROLLER_DOMAIN.call_once(|| interrupt_controller_domain);
}

pub fn init_trap_subsystem() {
    println!("++++ setup interrupt ++++");
    // 架构相关的trap初始化
    init_trap();

    // 通用步骤：打开外部中断、时钟中断与全局中断使能。
    arch::external_interrupt_enable();
    arch::timer_interrupt_enable();
    arch::interrupt_enable();

    let enable = arch::is_interrupt_enable();
    println!("++++ setup interrupt done, enable:{:?} ++++", enable);
}
