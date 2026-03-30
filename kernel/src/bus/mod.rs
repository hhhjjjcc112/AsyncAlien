#![allow(unused)]
use core::ops::Range;

use alloc::string::String;
use mem::PhysAddr;

use crate::error::AlienResult;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "riscv64")]
pub use self::riscv64::{fdt, mmio, pci, platform};
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::{acpi, mmio, pci, platform};

#[derive(Debug, Clone)]
pub struct CommonDeviceInfo {
    pub address_range: Range<PhysAddr>,
    pub irq: Option<u32>,
    pub compatible: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Plic,
    LocalApic,
    IoApic,
    Uart,
    Rtc,
    PciHost,
    VirtioMmio,
    VirtioPci,
    Ramdisk,
    LoopBack,
    SdCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTransport {
    Platform,
    Mmio,
    Pci,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmwareSource {
    Fdt,
    Acpi,
    Aml,
    PciScan,
    Fallback,
    Synthetic,
}

#[derive(Debug, Clone)]
pub enum DeviceLocator {
    Mmio(Range<PhysAddr>),
    Pio(Range<u16>),
    PciBdf {
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
    },
    None,
}

#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    pub class: DeviceClass,
    pub locator: DeviceLocator,
    pub transport: DeviceTransport,
    pub irq: Option<u32>,
    pub compatible: Option<String>,
    pub fw_source: FirmwareSource,
}

#[cfg(target_arch = "riscv64")]
pub use self::riscv64::CommonDeviceType;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::CommonDeviceType;

pub fn init_with_boot_info() -> AlienResult<()> {
    #[cfg(target_arch = "riscv64")]
    {
        return riscv64::init_with_dtb();
    }

    #[cfg(target_arch = "x86_64")]
    {
        return x86_64::init_with_acpi();
    }

    #[allow(unreachable_code)]
    Ok(())
}
