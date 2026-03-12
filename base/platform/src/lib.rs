#![no_std]

// 两级 cfg 校验：先架构，再具体平台。
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

/// 平台抽象 trait（参考 ArceOS 风格）。
pub mod traits;

// 导出公共 trait 与类型。
pub use traits::{
    ConsoleIf, IpiTarget, IrqIf,
    MemIf, MemRegionFlags, PhysMemRegion, RawRange,
    MachineInfo, MiscIf,
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

// 当前构建目标对应的平台实现。
#[cfg(plat_qemu_x86_64)]
pub type Platform = qemu_x86_64::QemuX86Platform;
#[cfg(plat_qemu_riscv)]
pub type Platform = qemu_riscv::QemuRiscvPlatform;
#[cfg(plat_vf2)]
pub type Platform = starfive2_riscv::Vf2Platform;

// 导出平台配置。
#[cfg(plat_qemu_x86_64)]
pub use qemu_x86_64::config;
#[cfg(plat_qemu_riscv)]
pub use qemu_riscv::config;
#[cfg(plat_vf2)]
pub use starfive2_riscv::config;

// 导出统一的平台机器信息类型。
#[cfg(target_arch = "x86_64")]
pub type PlatformInfo = common_x86_64::basic::MachineInfo;
#[cfg(target_arch = "riscv64")]
pub type PlatformInfo = common_riscv::basic::MachineInfo;

// x86_64 下导出 APIC 能力。
#[cfg(target_arch = "x86_64")]
pub mod apic {
    pub use crate::common_x86_64::apic::*;
}


/// 设置单次定时器。
pub fn set_timer(time: usize) {
    Platform::set_timer(time as u64);
}

/// 关机。
pub fn system_shutdown() -> ! {
    Platform::shutdown()
}

/// 控制台输出一个字符。
pub fn console_putchar(ch: u8) {
    Platform::putchar(ch);
}

/// 刷新远端 CPU 的指令可见性。
pub fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
    Platform::flush_cache(cpu_mask, cpu_mask_base)
}

/// 启动从核。
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
    Platform::start_secondary_cpu(cpu_id, start_addr, opaque)
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

pub fn platform_init(cpu_id: usize, info_ptr: usize) {
    clear_bss();
    println!("{}", ::config::ALIEN_FLAG);
    // 启动阶段先解析 boot_info，再发布 machine_info。
    Platform::init_boot_info(info_ptr);
    let machine_info = Platform::machine_info();
    MACHINE_INFO.call_once(|| machine_info);
    logger::init_logger();
    init_other_cpu(cpu_id);
    unsafe { main(cpu_id) }
}

#[cfg(target_arch = "x86_64")]
fn init_other_cpu(_cpu_id: usize) {}

#[cfg(target_arch = "riscv64")]
fn init_other_cpu(cpu_id: usize) {
    // vf2 的启动核不是 0，避免重复拉起。
    let start_cpu = if cfg!(plat_vf2) { 1 } else { 0 };
    for i in start_cpu..::config::CPU_NUM {
        if i != cpu_id {
            Platform::start_secondary_cpu(i, _start_secondary as *const () as usize, 0);
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
    Platform::boot_info_ptr()
}

static MACHINE_INFO: Once<PlatformInfo> = Once::new();

pub fn platform_machine_info() -> PlatformInfo {
    MACHINE_INFO.get().unwrap().clone()
}
