use alloc::collections::VecDeque;

use crate::bus::CommonDeviceInfo;

pub struct PciBus {
    common_devices: VecDeque<PciCommonDevice>,
}

#[derive(Debug, Clone, Copy)]
pub struct PciDeviceId {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub header_type: u8,
}

#[derive(Debug, Clone)]
pub struct PciCommonDevice {
    dev_id: PciDeviceId,
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
    pub fn new(dev_id: PciDeviceId, info: CommonDeviceInfo) -> Self {
        Self { dev_id, info }
    }

    pub fn dev_id(&self) -> PciDeviceId {
        self.dev_id
    }

    pub fn info(&self) -> &CommonDeviceInfo {
        &self.info
    }
}
