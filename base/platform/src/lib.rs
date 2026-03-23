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
    ConsoleIf,
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

/// x86_64 APIC 接口导出（兼容 kernel 侧 `platform::apic::*` 调用）。
#[cfg(target_arch = "x86_64")]
pub mod apic {
    pub use crate::common_x86_64::apic::*;
}

/// x86_64 ACPI 接口导出。
#[cfg(target_arch = "x86_64")]
pub mod acpi {
    pub use crate::common_x86_64::acpi::{
        device_info, device_list, init, tables, AcpiDeviceEntry, AcpiDeviceInfo, AcpiDeviceList, AcpiHost,
    };
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
pub fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *const () as *mut u8, ebss as *const () as usize - sbss as *const () as usize)
            .fill(0);
    }
}

/// BSP在 clear_bss() 后初始化percpu。
pub fn platform_init_percpu_primary(cpu_id: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        arch::init_percpu_primary(cpu_id);
    }
    #[cfg(target_arch = "riscv64")]
    {
        let _ = cpu_id;
    }
}

/// 从核初始化percpu。
pub fn platform_init_percpu_secondary(cpu_id: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        arch::init_percpu_secondary(cpu_id);
    }
    #[cfg(target_arch = "riscv64")]
    {
        let _ = cpu_id;
    }
}

/// BSP平台初始化。
pub fn platform_init_primary(_cpu_id: usize, info_ptr: usize) {
    // 不需要初始化控制台
    println!("{}", ::config::ALIEN_FLAG);
    #[cfg(target_arch = "x86_64")]
    {
        use common_x86_64::{apic, time};
        // 初始化 FPU/SSE，允许用户态浮点运算。
        arch::init_fpu();
        // 初始化 APIC。
        apic::init_primary_apic();
        // 初始化时间子系统（TSC、RTC）。
        time::init_time();
        // 初始化 APIC 定时器（依赖 TSC 校准）。
        time::init_primary_apic_timer();
    }
    Platform::init_boot_info(info_ptr);
    let machine_info = Platform::machine_info();
    MACHINE_INFO.call_once(|| machine_info);
    logger::init_logger();
}

pub fn platform_init_secondary(_cpu_id: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        use common_x86_64::{apic, time};
        // 初始化从核 FPU/SSE。
        arch::init_fpu();
        // 初始化 APIC。
        apic::init_secondary_apic();
        // 初始化 APIC 定时器（依赖 TSC 校准）。
        time::init_secondary_apic_timer();
    }
}


pub fn start_other_cpu(cpu_id: usize) {
    #[cfg(target_arch = "x86_64")] 
    {
        let total = platform_machine_info().cpu_count();
        for i in 0..total {
            if i != cpu_id {
                Platform::start_secondary_cpu(i, 0, 0);
            }
        }
    }
    #[cfg(target_arch = "riscv64")]
    {
        let start_cpu = if cfg!(plat_vf2) { 1 } else { 0 };
        for i in start_cpu..::config::CPU_NUM {
            if i != cpu_id {
                Platform::start_secondary_cpu(i, _start_secondary as *const () as usize, 0);
            }
        }
    }
    
}

unsafe extern "Rust" {
    fn main(cpu_id: usize, info_ptr: usize);
    fn secondary_main(cpu_id: usize);
    #[cfg(target_arch = "riscv64")]
    fn _start_secondary();
}

pub fn platform_boot_info_ptr() -> usize {
    Platform::boot_info_ptr()
}

static MACHINE_INFO: Once<PlatformInfo> = Once::new();

pub fn platform_machine_info() -> PlatformInfo {
    MACHINE_INFO.get().unwrap().clone()
}
