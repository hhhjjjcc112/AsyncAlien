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
    pub locator: DeviceLocator,
    pub irq: Option<u32>,
    pub compatible: Option<String>,
}

impl CommonDeviceInfo {
    /// 函数说明：检查 `address_range` 与 `locator` 是否语义一致。
    pub fn is_locator_consistent(&self) -> bool {
        if self.address_range.start >= self.address_range.end {
            return false;
        }

        match &self.locator {
            DeviceLocator::Mmio(range) => {
                range.start < range.end
                    && self.address_range.start == range.start
                    && self.address_range.end == range.end
            }
            DeviceLocator::Pio(range) => {
                if range.start >= range.end {
                    return false;
                }
                let expected_start = usize::from(range.start);
                let expected_end = usize::from(range.end);
                self.address_range.start.as_usize() == expected_start
                    && self.address_range.end.as_usize() == expected_end
            }
            DeviceLocator::PciBdf { .. } | DeviceLocator::None => false,
        }
    }

    /// 函数说明：在设备注册入口进行定位语义校验。
    pub fn validate_locator_or_warn(&self, entry: &str) -> bool {
        let ok = self.is_locator_consistent();
        debug_assert!(
            ok,
            "[bus][locator] inconsistent info at {}: locator={:?}, address_range={:#x}..{:#x}",
            entry,
            self.locator,
            self.address_range.start.as_usize(),
            self.address_range.end.as_usize()
        );
        if !ok {
            warn!(
                "[bus][locator] skip inconsistent device at {}: locator={:?}, address_range={:#x}..{:#x}",
                entry,
                self.locator,
                self.address_range.start.as_usize(),
                self.address_range.end.as_usize()
            );
            return false;
        }
        true
    }
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

#[cfg(test)]
mod tests {
    use super::{CommonDeviceInfo, DeviceLocator};
    use mem::PhysAddr;

    fn mmio_info(addr_start: usize, addr_end: usize, loc_start: usize, loc_end: usize) -> CommonDeviceInfo {
        CommonDeviceInfo {
            address_range: PhysAddr::from(addr_start)..PhysAddr::from(addr_end),
            locator: DeviceLocator::Mmio(PhysAddr::from(loc_start)..PhysAddr::from(loc_end)),
            irq: None,
            compatible: None,
        }
    }

    fn pio_info(addr_start: usize, addr_end: usize, loc_start: u16, loc_end: u16) -> CommonDeviceInfo {
        CommonDeviceInfo {
            address_range: PhysAddr::from(addr_start)..PhysAddr::from(addr_end),
            locator: DeviceLocator::Pio(loc_start..loc_end),
            irq: None,
            compatible: None,
        }
    }

    #[test]
    fn mmio_locator_consistency_cases() {
        assert!(mmio_info(0x1000, 0x2000, 0x1000, 0x2000).is_locator_consistent());
        assert!(!mmio_info(0x1000, 0x2000, 0x1000, 0x3000).is_locator_consistent());
        assert!(!mmio_info(0x1000, 0x1000, 0x1000, 0x1000).is_locator_consistent());
        assert!(!mmio_info(0x2000, 0x1000, 0x2000, 0x1000).is_locator_consistent());
    }

    #[test]
    fn pio_locator_consistency_cases() {
        assert!(pio_info(0x3f8, 0x400, 0x3f8, 0x400).is_locator_consistent());
        assert!(!pio_info(0x3f8, 0x400, 0x3f8, 0x3ff).is_locator_consistent());
        assert!(!pio_info(0x3f8, 0x400, 0x3f8, 0x3f8).is_locator_consistent());
        assert!(!pio_info(0x400, 0x3f8, 0x3f8, 0x400).is_locator_consistent());
    }

    #[test]
    fn non_mmio_pio_locator_is_inconsistent() {
        let info_none = CommonDeviceInfo {
            address_range: PhysAddr::from(0x1000)..PhysAddr::from(0x2000),
            locator: DeviceLocator::None,
            irq: None,
            compatible: None,
        };
        assert!(!info_none.is_locator_consistent());
    }
}
