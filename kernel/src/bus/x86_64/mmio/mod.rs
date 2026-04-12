mod device;

use basic::io::SafeIORegion;
use ksync::Mutex;

use crate::bus::CommonDeviceInfo;

use self::device::{MmioBus, MmioCommonDevice};

pub static MMIO_BUS: Mutex<MmioBus> = Mutex::new(MmioBus::new());

const VIRTIO_MMIO_MAGIC: u32 = 0x74726976;

/// 函数说明：执行对应的总线处理步骤。
pub fn register_mmio_device(info: CommonDeviceInfo) {
    if !info.validate_locator_or_warn("x86_64.mmio") {
        return;
    }
    // 步骤1：读取 virtio-mmio 标识并判断设备是否有效。
    let io_region = SafeIORegion::new(info.address_range.clone());
    let magic = io_region.read_at::<u32>(0).unwrap();
    let device_id = io_region.read_at::<u32>(8).unwrap();
    if magic == VIRTIO_MMIO_MAGIC && device_id != 0 {
        // 步骤2：将合法设备注册到 MMIO 总线。
        let mmio_device = MmioCommonDevice::new(io_region, info);
        MMIO_BUS.lock().register_mmio_device(mmio_device);
    }
}

/// 函数说明：执行对应的总线处理步骤。
pub fn register_mmio_driver() {
    // MMIO_BUS.lock().register_driver(driver);
}

#[macro_export]
macro_rules! mmio_bus {
    () => {
        $crate::bus::mmio::MMIO_BUS
    };
}
