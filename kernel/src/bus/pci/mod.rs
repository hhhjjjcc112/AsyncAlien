mod device;

use basic::io::SafeIORegion;
use device::PciBus;
use ksync::Mutex;

use crate::bus::{pci::device::PciCommonDevice, CommonDeviceInfo, CommonDeviceType};

pub static PCI_BUS: Mutex<PciBus> = Mutex::new(PciBus::new());

/// 把 PCI ECAM 区域统一封装成总线层设备描述，供不同架构的发现入口复用。
/// 这里不做具体枚举，只把“可访问的配置空间”交给后面的 `PciBus`。
pub fn ecam_device(pci_info: CommonDeviceInfo) -> CommonDeviceType {
    CommonDeviceType::Pci(pci_info)
}

/// 把一个 ECAM 区域挂到 PCI 总线对象里。
/// `pci_init` 的职责只有两步：先把内存映射包成安全访问区，再交给总线统一管理。
pub fn pci_init(pci_info: CommonDeviceInfo) {
	let io_region = SafeIORegion::new(pci_info.address_range.clone());
	let pci_device = PciCommonDevice::new(io_region, pci_info);
	PCI_BUS.lock().register_common_device(pci_device);
}

#[macro_export]
macro_rules! pci_bus {
	() => {
		$crate::bus::pci::PCI_BUS
	};
}
