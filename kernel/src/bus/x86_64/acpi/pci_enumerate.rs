use alloc::vec::Vec;

use mem::PhysAddr;

use crate::bus::{pci::ecam_device, CommonDeviceInfo, CommonDeviceType, DeviceLocator};

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_pci_devices<H: ::acpi::Handler>(tables: &::acpi::AcpiTables<H>) -> Vec<CommonDeviceType> {
    // 步骤1：优先从 MCFG 读取 ECAM 区域并转换为统一设备描述。
    let mut devices = Vec::new();

    // MCFG 决定了 PCI 配置空间的真实布局。
    if let Ok(pci_regions) = ::acpi::platform::PciConfigRegions::new(tables) {
        for region in pci_regions.regions.iter() {
            let segment = region.pci_segment_group;
            let bus_start = region.bus_number_start;
            let bus_end = region.bus_number_end;
            let base = region.base_address;

            if segment != 0 {
                warn!(
                    "[bus][x86_64][acpi][pci] segment={} not fully supported in minimal model, keep scanning as-is",
                    segment
                );
            }
            if bus_start != 0 {
                warn!(
                    "[bus][x86_64][acpi][pci] bus_start={} not fully supported in minimal model, keep scanning from ecam base",
                    bus_start
                );
            }
            let bus_count = (bus_end - bus_start) as usize + 1;
            let size = bus_count << 20;
            let address_range =
                PhysAddr::from(base as usize)..PhysAddr::from(base as usize + size);
            devices.push(ecam_device(CommonDeviceInfo {
                address_range: address_range.clone(),
                locator: DeviceLocator::Mmio(address_range),
                irq: None,
                compatible: Some("pci_ecam".into()),
            }));
        }
        return devices;
    }

    // 步骤2：MCFG 缺失时使用固定 ECAM 兜底。
    let base = platform::config::PCI_ECAM_BASE;
    let size = 0x1000_0000usize;
    warn!(
        "[bus][x86_64][acpi][pci] MCFG unavailable, fallback ecam={:#x}..{:#x}",
        base,
        base + size
    );
    let address_range = PhysAddr::from(base)..PhysAddr::from(base + size);
    devices.push(ecam_device(CommonDeviceInfo {
        address_range: address_range.clone(),
        locator: DeviceLocator::Mmio(address_range),
        irq: None,
        compatible: Some("pci_ecam".into()),
    }));

    devices
}
