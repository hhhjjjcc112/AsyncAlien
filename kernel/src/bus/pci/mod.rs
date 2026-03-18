mod device;

use basic::io::SafeIORegion;
use device::PciBus;
use ksync::Mutex;

use crate::bus::{pci::device::PciCommonDevice, CommonDeviceInfo};

pub static PCI_BUS: Mutex<PciBus> = Mutex::new(PciBus::new());

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
