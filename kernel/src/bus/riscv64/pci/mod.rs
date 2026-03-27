mod device;

use device::PciBus;
use ksync::Mutex;

use crate::bus::CommonDeviceInfo;

pub static PCI_BUS: Mutex<PciBus> = Mutex::new(PciBus::new());

/// 初始化 RISC-V 平台 PCI 总线入口。
/// 与原始 Alien 保持一致：当前阶段仅保留入口，不在此处做 ECAM 端点扫描。
pub fn pci_init(_pci_info: CommonDeviceInfo) {}

#[macro_export]
macro_rules! pci_bus {
    () => {
        $crate::bus::pci::PCI_BUS
    };
}
