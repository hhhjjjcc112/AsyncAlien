#![no_std]

// Compile-time checks for two-level cfg model.
// Level 1: architecture cfg (target_arch = riscv64 / x86_64)
// Level 2: platform cfg (plat_qemu_riscv / plat_qemu_x86_64 / plat_vf2)
#[cfg(not(any(target_arch = "riscv64", target_arch = "x86_64")))]
compile_error!("Unsupported architecture. Expected target_arch = riscv64 or x86_64");

#[cfg(not(any(plat_qemu_riscv, plat_vf2, plat_qemu_x86_64)))]
compile_error!("No valid platform selected! Use --cfg plat_qemu_riscv, --cfg plat_vf2, or --cfg plat_qemu_x86_64");

#[cfg(any(
    all(plat_qemu_riscv, plat_vf2),
    all(plat_qemu_riscv, plat_qemu_x86_64),
    all(plat_vf2, plat_qemu_x86_64)
))]
compile_error!("Multiple platforms selected! Select exactly one platform cfg");

#[cfg(all(target_arch = "x86_64", not(plat_qemu_x86_64)))]
compile_error!("ARCH x86_64 requires PLATFORM=plat_qemu_x86_64");

#[cfg(all(target_arch = "riscv64", not(any(plat_qemu_riscv, plat_vf2))))]
compile_error!("ARCH riscv64 requires PLATFORM=plat_qemu_riscv or plat_vf2");

#[macro_use]
pub mod console;

/// Unified platform abstraction traits (ArceOS-style)
pub mod traits;
pub mod platform_trait;

// Re-export common types from traits
pub use traits::{
    ConsoleIf, IpiTarget, IrqIf, 
    MemIf, MemRegionFlags, PhysMemRegion, RawRange,
    MachineInfo, MiscIf, PlatformCallRet,
    PowerIf, TimeIf,
};

#[cfg(target_arch = "x86_64")]
mod common_x86_64;
#[cfg(target_arch = "riscv64")]
mod common_riscv;
mod logger;
#[cfg(plat_qemu_riscv)]
mod qemu_riscv;
#[cfg(plat_qemu_x86_64)]
mod qemu_x86_64;

#[cfg(plat_vf2)]
mod starfive2_riscv;

use spin::Once;

// Type aliases for platform-specific implementations
#[cfg(plat_qemu_x86_64)]
pub type Platform = qemu_x86_64::QemuX86Platform;
#[cfg(plat_qemu_riscv)]
pub type Platform = qemu_riscv::QemuRiscvPlatform;
#[cfg(plat_vf2)]
pub type Platform = starfive2_riscv::Vf2Platform;

// Re-export platform config
#[cfg(plat_qemu_x86_64)]
pub use qemu_x86_64::config;
#[cfg(plat_qemu_riscv)]
pub use qemu_riscv::config;
#[cfg(plat_vf2)]
pub use starfive2_riscv::config;

// Export MachineInfo type based on platform
#[cfg(target_arch = "x86_64")]
pub type PlatformInfo = common_x86_64::basic::MachineInfo;
#[cfg(target_arch = "riscv64")]
pub type PlatformInfo = common_riscv::basic::MachineInfo;

// Export APIC functionality for x86-64
#[cfg(target_arch = "x86_64")]
pub mod apic {
    pub use crate::common_x86_64::apic::*;
}

// ============================================================================
// Unified platform operations using trait methods
// ============================================================================

/// Set a one-shot timer
#[cfg(target_arch = "riscv64")]
pub fn set_timer(time: usize) {
    crate::common_riscv::sbi::set_timer(time);
}

#[cfg(target_arch = "x86_64")]
pub fn set_timer(time: usize) {
    crate::common_x86_64::services::set_timer(time);
}

/// System shutdown
pub fn system_shutdown() -> ! {
    <Platform as PowerIf>::system_off()
}

/// Console output (single character)
pub fn console_putchar(ch: u8) {
    <Platform as ConsoleIf>::putchar(ch);
}

/// Flush instruction cache on remote CPUs
/// On RISC-V: remote FENCE.I via SBI
/// On x86-64: no-op (coherent I-cache)
pub fn remote_instruction_fence(cpu_mask: usize, cpu_mask_base: usize) -> PlatformCallRet {
    <Platform as PowerIf>::remote_fence_i(cpu_mask, cpu_mask_base)
}

/// Compatibility alias for remote_instruction_fence
#[deprecated(note = "use remote_instruction_fence instead")]
pub fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> PlatformCallRet {
    remote_instruction_fence(hart_mask, hart_mask_base)
}

/// Start a secondary CPU core
/// On RISC-V: hart_start via SBI HSM extension
/// On x86-64: INIT-SIPI-SIPI via APIC
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformCallRet {
    <Platform as PowerIf>::cpu_boot(cpu_id, start_addr, opaque)
}

unsafe extern "C" {
    fn sbss();
    fn ebss();
}

/// 清空.bss段
fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *const () as *mut u8, ebss as *const () as usize - sbss as *const () as usize)
            .fill(0);
    }
}

pub fn platform_init_with_boot_info(hart_id: usize, boot_info: usize) {
    clear_bss();
    println!("{}", ::config::ALIEN_FLAG);
    <Platform as MiscIf>::init_boot_info(boot_info);
    let machine_info = <Platform as MiscIf>::machine_info();
    MACHINE_INFO.call_once(|| machine_info);
    logger::init_logger();
    init_other_hart(hart_id);
    unsafe { main(hart_id) }
}

#[deprecated(note = "use platform_init_with_boot_info")]
pub fn platform_init(hart_id: usize, dtb: usize) {
    platform_init_with_boot_info(hart_id, dtb)
}

#[cfg(target_arch = "x86_64")]
fn init_other_hart(_hart_id: usize) {}

#[cfg(target_arch = "riscv64")]
fn init_other_hart(hart_id: usize) {
    let start_hart = if cfg!(plat_vf2) { 1 } else { 0 };
    for i in start_hart..::config::CPU_NUM {
        if i != hart_id {
            let res = <Platform as PowerIf>::cpu_boot(i, _start_secondary as *const () as usize, 0);
            assert_eq!(res.error, 0);
        }
    }
}

unsafe extern "C" {
    fn main(hart_id: usize);
    #[cfg(target_arch = "riscv64")]
    fn _start_secondary();
}

#[deprecated(note = "use platform_boot_info_ptr")]
pub fn platform_dtb_ptr() -> usize {
    platform_boot_info_ptr()
}

pub fn platform_boot_info_ptr() -> usize {
    <Platform as MiscIf>::boot_info_ptr()
}

static MACHINE_INFO: Once<PlatformInfo> = Once::new();

pub fn platform_machine_info() -> PlatformInfo {
    MACHINE_INFO.get().unwrap().clone()
}
