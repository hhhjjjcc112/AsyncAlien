use alloc::vec::Vec;

use mem::PhysAddr;

use crate::bus::{pci::ecam_device, CommonDeviceInfo, CommonDeviceType};

pub fn enumerate_pci_devices(tables: &::acpi::AcpiTables<platform::acpi::AcpiHost>) -> Vec<CommonDeviceType> {
    let mut devices = Vec::new();

    // MCFG 决定了 PCI 配置空间的真实布局。
    // 这里用的是 `acpi` crate 的高层接口，不需要自己手工解析 ACPI 表结构。
    if let Ok(pci_regions) = ::acpi::platform::PciConfigRegions::new(tables) {
        for region in pci_regions.regions.iter() {
            // 每个 MCFG 区域对应一个连续的 ECAM 视图，按总线数量推导映射长度。
            // 这里得到的是“可访问的 PCI 配置空间区间”，后面才会交给总线层封装。
            let bus_count = (region.bus_number_end - region.bus_number_start) as usize + 1;
            let size = bus_count << 20;
            devices.push(ecam_device(CommonDeviceInfo {
                address_range: PhysAddr::from(region.base_address as usize)
                    ..PhysAddr::from(region.base_address as usize + size),
                irq: None,
                compatible: Some("pci_ecam".into()),
            }));
        }
        return devices;
    }

    // 兜底路径：如果平台没有给出可用的 MCFG，则先用固定配置保证启动链路可继续。
    // 这只应在 ACPI 信息缺失或极早期调试时出现。
    let base = platform::config::PCI_ECAM_BASE;
    let size = 0x1000_0000usize;
    devices.push(ecam_device(CommonDeviceInfo {
        address_range: PhysAddr::from(base)..PhysAddr::from(base + size),
        irq: None,
        compatible: Some("pci_ecam".into()),
    }));

    devices
}
