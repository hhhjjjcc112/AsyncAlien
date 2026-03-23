#![allow(unused)]
use alloc::{string::String, vec};
use core::ops::Range;

use ::fdt::Fdt;
use mem::PhysAddr;

use crate::error::AlienResult;

#[cfg(target_arch = "riscv64")]
mod fdt;
mod acpi;
pub mod mmio;
pub mod pci;
pub mod platform;

#[derive(Debug, Clone)]
pub struct CommonDeviceInfo {
    pub address_range: Range<PhysAddr>,
    pub irq: Option<u32>,
    pub compatible: Option<String>,
}
#[derive(Debug, Clone)]
pub enum CommonDeviceType {
    #[cfg(target_arch = "riscv64")]
    Plic(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    LocalApic(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    IoApic(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    Hpet(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    Uart(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    Rtc(CommonDeviceInfo),
    #[cfg(target_arch = "riscv64")]
    VirtIo(CommonDeviceInfo),
    #[cfg(target_arch = "x86_64")]
    Pci(CommonDeviceInfo),
    #[cfg(any(feature = "bench", all(target_arch = "riscv64", plat_vf2, not(plat_vf2_sd))))]
    Ramdisk(CommonDeviceInfo),
    #[cfg(all(target_arch = "riscv64", plat_vf2))]
    LoopBack(CommonDeviceInfo),
    #[cfg(all(target_arch = "riscv64", plat_vf2, plat_vf2_sd))]
    SdCard(CommonDeviceInfo),
}

#[cfg(target_arch = "riscv64")]
fn register_detected_devices(devices: alloc::vec::Vec<CommonDeviceType>) {
    devices.into_iter().for_each(|ty| match ty {
        #[cfg(target_arch = "riscv64")]
        CommonDeviceType::Plic(info) => platform::register_platform_device(info, "plic"),
        CommonDeviceType::VirtIo(info) => mmio::register_mmio_device(info),
        #[cfg(any(feature = "bench", all(target_arch = "riscv64", plat_vf2, not(plat_vf2_sd))))]
        CommonDeviceType::Ramdisk(info) => platform::register_platform_device(info, "ramdisk"),
        #[cfg(all(target_arch = "riscv64", plat_vf2))]
        CommonDeviceType::LoopBack(info) => platform::register_platform_device(info, "loopback"),
        #[cfg(all(target_arch = "riscv64", plat_vf2, plat_vf2_sd))]
        CommonDeviceType::SdCard(info) => platform::register_platform_device(info, "sdcard"),
    });
}

#[cfg(target_arch = "x86_64")]
fn register_detected_devices(devices: alloc::vec::Vec<CommonDeviceType>) {
    devices.into_iter().for_each(|ty| match ty {
        CommonDeviceType::LocalApic(info) => platform::register_platform_device(info, "local_apic"),
        CommonDeviceType::IoApic(info) => platform::register_platform_device(info, "io_apic"),
        CommonDeviceType::Hpet(info) => platform::register_platform_device(info, "hpet"),
        CommonDeviceType::Uart(info) => platform::register_platform_device(info, "uart"),
        CommonDeviceType::Rtc(info) => platform::register_platform_device(info, "rtc"),
        CommonDeviceType::Pci(info) => pci::pci_init(info),
    });
}

#[cfg(target_arch = "riscv64")]
pub fn init_with_dtb() -> AlienResult<()> {
    let ptr = ::platform::platform_boot_info_ptr();
    let dtb = unsafe { Fdt::from_ptr(ptr as *const u8) }.unwrap();

    let mut devices = vec![];
    if let Some(ty) = dtb.probe_rtc() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_uart() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_plic() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_pci() {
        devices.push(ty);
    }
    let virtio = dtb.probe_virtio();
    if let Some(virtio) = virtio {
        for ty in virtio {
            devices.push(ty);
        }
    }

    #[cfg(feature = "bench")]
    {
        let ramdisk_start = RAMDISK.as_ptr() as usize;
        let len = RAMDISK.len();
        let info = CommonDeviceInfo {
            address_range: PhysAddr::from(ramdisk_start)..PhysAddr::from(ramdisk_start + len),
            irq: None,
            compatible: None,
        };
        devices.push(CommonDeviceType::Ramdisk(info));
    }

    #[cfg(plat_vf2)]
    {
        #[cfg(not(plat_vf2_sd))]
        {
            let ramdisk_start = RAMDISK.as_ptr() as usize;
            let len = RAMDISK.len();
            let info = CommonDeviceInfo {
                address_range: PhysAddr::from(ramdisk_start)..PhysAddr::from(ramdisk_start + len),
                irq: None,
                compatible: None,
            };
            devices.push(CommonDeviceType::Ramdisk(info));
        }

        let fake_nic = CommonDeviceInfo {
            address_range: PhysAddr::from(0)..PhysAddr::from(0 + 0),
            irq: Some(0),
            compatible: None,
        };
        devices.push(CommonDeviceType::LoopBack(fake_nic));
    }
    #[cfg(all(plat_vf2, plat_vf2_sd))]
    dtb.probe_sd().map(|ty| {
        devices.push(ty);
    });

    register_detected_devices(devices);
    Ok(())
}

#[cfg(target_arch = "x86_64")]
fn init_with_acpi() -> AlienResult<()> {
    let mut devices = acpi::enumerate_devices();

    if devices.is_empty() {
        // 如果 ACPI 没给出任何可用信息，至少保留一个串口，保证早期调试链路不中断。
        devices.push(CommonDeviceType::Uart(CommonDeviceInfo {
            address_range: PhysAddr::from(0x3f8usize)..PhysAddr::from(0x400usize),
            irq: Some(4),
            compatible: Some("ns16550a".into()),
        }));
    }

    register_detected_devices(devices);
    Ok(())
}

pub fn init_with_boot_info() -> AlienResult<()> {
    #[cfg(target_arch = "riscv64")]
    {
        return init_with_dtb();
    }

    #[cfg(target_arch = "x86_64")]
    {
        return init_with_acpi();
    }

    #[allow(unreachable_code)]
    Ok(())
}

#[cfg(feature = "bench")]
static RAMDISK: &'static [u8] = &[0u8; 1024];
#[cfg(all(plat_vf2, not(plat_vf2_sd)))]
static RAMDISK: &'static [u8] = include_bytes!("../../../build/sdcard.img");
