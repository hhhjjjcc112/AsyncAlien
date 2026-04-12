use alloc::string::ToString;

use basic::io::SafeIORegion;
use device::{PlatformBus, PlatformCommonDevice};
use ksync::Mutex;

use crate::bus::CommonDeviceInfo;

mod device;

pub static PLATFORM_BUS: Mutex<PlatformBus> = Mutex::new(PlatformBus::new());

/// 函数说明：执行对应的总线处理步骤。
pub fn register_platform_device(info: CommonDeviceInfo, name: &str) {
    if !info.validate_locator_or_warn(name) {
        return;
    }
    // 步骤1：封装平台设备地址与中断信息。
    let io_region = SafeIORegion::new(info.address_range.clone());
    let platform_device = PlatformCommonDevice::new(io_region, info, name.to_string());

    // 步骤2：注册到平台总线。
    PLATFORM_BUS.lock().register_common_device(platform_device);
}

/// 函数说明：执行对应的总线处理步骤。
pub fn register_platform_driver() {
    // PLATFORM_BUS.lock().register_driver(driver);
}

#[macro_export]
macro_rules! platform_bus {
    () => {
        $crate::bus::platform::PLATFORM_BUS.lock()
    };
}
