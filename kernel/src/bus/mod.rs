#![allow(unused)]
use alloc::{string::String, vec};
use core::ops::Range;

use ::fdt::Fdt;
use mem::PhysAddr;

use crate::{bus::fdt::Probe, error::AlienResult};

mod fdt;
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
    Plic(CommonDeviceInfo),
    Uart(CommonDeviceInfo),
    Rtc(CommonDeviceInfo),
    VirtIo(CommonDeviceInfo),
    Pci(CommonDeviceInfo),
    Ramdisk(CommonDeviceInfo),
    LoopBack(CommonDeviceInfo),
    SdCard(CommonDeviceInfo),
}

fn register_detected_devices(devices: alloc::vec::Vec<CommonDeviceType>) {
    devices.into_iter().for_each(|ty| match ty {
        CommonDeviceType::Plic(info) => platform::register_platform_device(info, "plic"),
        CommonDeviceType::Uart(info) => platform::register_platform_device(info, "uart"),
        CommonDeviceType::Rtc(info) => platform::register_platform_device(info, "rtc"),
        CommonDeviceType::VirtIo(info) => mmio::register_mmio_device(info),
        CommonDeviceType::Pci(info) => pci::pci_init(info),
        CommonDeviceType::Ramdisk(info) => platform::register_platform_device(info, "ramdisk"),
        CommonDeviceType::LoopBack(info) => platform::register_platform_device(info, "loopback"),
        CommonDeviceType::SdCard(info) => platform::register_platform_device(info, "sdcard"),
    });
}

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
pub fn init_with_acpi() -> AlienResult<()> {
    let mut devices = vec![];

    // Register ACPI-discovered x86 devices into the common bus model.
    for (name, base, size) in ::platform::config::device_space_dynamic().iter() {
        let info = CommonDeviceInfo {
            address_range: PhysAddr::from(*base)..PhysAddr::from(*base + *size),
            irq: None,
            compatible: Some((*name).into()),
        };

        match *name {
            // Keep using the "plic" abstraction; x86 backend interprets it as APIC/IOAPIC.
            "io_apic" | "local_apic" => devices.push(CommonDeviceType::Plic(info)),
            "pci_ecam" => devices.push(CommonDeviceType::Pci(info)),
            _ => {}
        }
    }

    // Legacy COM1 for early serial/uart domain compatibility on x86.
    let uart_info = CommonDeviceInfo {
        address_range: PhysAddr::from(0x3f8usize)..PhysAddr::from(0x400usize),
        irq: Some(4),
        compatible: Some("ns16550a".into()),
    };
    devices.push(CommonDeviceType::Uart(uart_info));

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
