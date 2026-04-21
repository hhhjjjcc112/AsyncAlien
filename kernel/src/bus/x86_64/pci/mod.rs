mod device;

#[cfg(target_arch = "x86_64")]
use alloc::{format, vec::Vec};

use basic::io::SafeIORegion;
use device::PciBus;
use ksync::Mutex;

use crate::bus::{CommonDeviceInfo, CommonDeviceType};

use self::device::PciCommonDevice;

pub static PCI_BUS: Mutex<PciBus> = Mutex::new(PciBus::new());

/// 把 PCI ECAM 区域统一封装成总线层设备描述，供不同架构的发现入口复用。
/// 这里不做具体枚举，只把“可访问的配置空间”交给后面的 `PciBus`。
pub fn ecam_device(pci_info: CommonDeviceInfo) -> CommonDeviceType {
    // 步骤1：把 ECAM 区域包装成统一 PCI 设备类型。
    CommonDeviceType::Pci(pci_info)
}

/// 把一个 ECAM 区域挂到 PCI 总线对象里。
/// `pci_init` 的职责只有两步：先把内存映射包成安全访问区，再交给总线统一管理。
pub fn pci_init(pci_info: CommonDeviceInfo) {
    if !pci_info.validate_locator_or_warn("x86_64.pci_host") {
        return;
    }
    // 步骤1：创建受边界保护的 IO 区域。
    let io_region = SafeIORegion::new(pci_info.address_range.clone());
    // 步骤2：注册到 PCI 总线统一管理。
    let pci_device = PciCommonDevice::new(io_region, pci_info);
    PCI_BUS.lock().register_common_device(pci_device);
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
pub fn collect_virtio_devices() -> Vec<CommonDeviceType> {
    // 步骤1：遍历 PCI 端点，筛选 virtio 设备。
    let bus = PCI_BUS.lock();
    let mut devices = Vec::new();

    for endpoint in bus.endpoint_devices().iter() {
        let Some(_kind) = endpoint.virtio_kind() else {
            continue;
        };

        let addr = endpoint.address();
        let bdf = format!(
            "{:04x}:{:02x}:{:02x}.{}",
            addr.segment(),
            addr.bus(),
            addr.device(),
            addr.function()
        );

        // 步骤2：统一为 virtio 设备类型。
        devices.push(CommonDeviceType::Virtio(bdf));
    }

    devices
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! pci_bus {
    () => {
        $crate::bus::pci::PCI_BUS
    };
}
