use alloc::{collections::VecDeque, string::String};
use core::ops::Range;

use basic::io::SafeIORegion;
use mem::PhysAddr;

use crate::bus::{CommonDeviceInfo, DeviceLocator};

pub struct PlatformBus {
    common_devices: VecDeque<PlatformCommonDevice>,
}

impl PlatformBus {
    pub(super) const fn new() -> Self {
        Self {
            common_devices: VecDeque::new(),
        }
    }
    pub(super) fn register_common_device(&mut self, device: PlatformCommonDevice) {
        self.common_devices.push_back(device);
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn register_driver(&mut self) {
        // self.drivers.push(driver);
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn common_devices(&self) -> &VecDeque<PlatformCommonDevice> {
        &self.common_devices
    }
}
#[derive(Debug)]
pub struct PlatformCommonDevice {
    io_region: SafeIORegion,
    info: CommonDeviceInfo,
    name: String,
}

impl PlatformCommonDevice {
    pub(super) fn new(io_region: SafeIORegion, info: CommonDeviceInfo, name: String) -> Self {
        let res = Self {
            io_region,
            info,
            name,
        };
        info!(
            "[PlatformCommonDevice]: Found platform device, device name:{:?}, irq number:{:?}",
            res.name(),
            res.info.irq
        );
        res
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn address(&self) -> PhysAddr {
        self.io_region.phys_addr()
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn address_range(&self) -> Range<PhysAddr> {
        self.io_region.phys_addr_range()
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn io_region(&self) -> &SafeIORegion {
        &self.io_region
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn name(&self) -> &str {
        &self.name
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn irq(&self) -> Option<u32> {
        self.info.irq
    }

    /// 函数说明：执行对应的总线处理步骤。
    pub fn compatible(&self) -> Option<&str> {
        self.info.compatible.as_deref()
    }

    /// 函数说明：返回设备的显式 I/O 语义定位信息。
    pub fn locator(&self) -> &DeviceLocator {
        &self.info.locator
    }
}
