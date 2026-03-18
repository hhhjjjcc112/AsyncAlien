use alloc::collections::VecDeque;
use core::ops::Range;

use basic::io::SafeIORegion;
use mem::PhysAddr;

use crate::bus::CommonDeviceInfo;

pub struct PciBus {
    common_devices: VecDeque<PciCommonDevice>,
}
#[derive(Debug)]
pub struct PciCommonDevice {
    io_region: SafeIORegion,
    info: CommonDeviceInfo,
}

impl PciBus {
    pub(super) const fn new() -> Self {
        Self {
            common_devices: VecDeque::new(),
        }
    }
    pub(super) fn register_common_device(&mut self, device: PciCommonDevice) {
        self.common_devices.push_back(device);
    }

    pub fn register_driver(&mut self) {
        // self.drivers.push(driver);
    }

    pub fn common_devices(&self) -> &VecDeque<PciCommonDevice> {
        &self.common_devices
    }
}

impl PciCommonDevice {
    pub(super) fn new(io_region: SafeIORegion, info: CommonDeviceInfo) -> Self {
        let res = Self { io_region, info };
        info!(
            "[PciCommonDevice]: Found PCI ECAM region, addr: {:#x?}",
            res.address_range()
        );
        res
    }

    pub fn address(&self) -> PhysAddr {
        self.io_region.phys_addr()
    }

    pub fn address_range(&self) -> Range<PhysAddr> {
        self.io_region.phys_addr_range()
    }

    pub fn io_region(&self) -> &SafeIORegion {
        &self.io_region
    }

    pub fn irq(&self) -> Option<u32> {
        self.info.irq
    }

    pub fn compatible(&self) -> Option<&str> {
        self.info.compatible.as_deref()
    }
}
