use alloc::{string::ToString, vec::Vec};
use core::ops::Range;

use fdt::{standard_nodes::Compatible, Fdt};
use mem::PhysAddr;

use crate::bus::{pci::ecam_device, CommonDeviceInfo, CommonDeviceType};

pub trait Probe {
    /// Get the base address and irq number of the uart device from the device tree.
    fn probe_uart(&self) -> Option<CommonDeviceType>;
    /// Get the base address and irq number of the rtc device from the device tree.
    fn probe_rtc(&self) -> Option<CommonDeviceType>;
    fn probe_plic(&self) -> Option<CommonDeviceType>;
    /// Get the base address and irq number of the virtio devices from the device tree.
    fn probe_virtio(&self) -> Option<Vec<CommonDeviceType>>;
    fn probe_common(&self, device_name: &str, has_irq: bool) -> Option<CommonDeviceInfo>;
    fn probe_pci(&self) -> Option<CommonDeviceType>;
    #[cfg(all(plat_vf2, plat_vf2_sd))]
    fn probe_sd(&self) -> Option<CommonDeviceType>;
}

impl Probe for Fdt<'_> {
    fn probe_uart(&self) -> Option<CommonDeviceType> {
        match self.probe_common("uart", true) {
            Some(device_info) => Some(CommonDeviceType::Uart(device_info)),
            None => self
                .probe_common("serial", true)
                .map(CommonDeviceType::Uart),
        }
    }

    fn probe_rtc(&self) -> Option<CommonDeviceType> {
        self.probe_common("rtc", true).map(CommonDeviceType::Rtc)
    }

    fn probe_plic(&self) -> Option<CommonDeviceType> {
        self.probe_common("plic", false).map(CommonDeviceType::Plic)
    }

    fn probe_virtio(&self) -> Option<Vec<CommonDeviceType>> {
        let mut virtio_devices = Vec::new();
        for node in self.all_nodes() {
            if node.name.starts_with("virtio") {
                let reg = node.reg().unwrap().next().unwrap();
                let range = Range {
                    start: PhysAddr::from(reg.starting_address as usize),
                    end: PhysAddr::from(reg.starting_address as usize + reg.size.unwrap()),
                };
                let irq = node.property("interrupts").unwrap().value;
                let irq = u32::from_be_bytes(irq[0..4].try_into().ok().unwrap());
                let compatible = node.compatible().map(Compatible::first).unwrap();
                let device_info = CommonDeviceInfo {
                    address_range: range,
                    irq: Some(irq),
                    compatible: Some(compatible.to_string()),
                };
                virtio_devices.push(CommonDeviceType::VirtIo(device_info));
            }
        }
        Some(virtio_devices)
    }

    fn probe_common(&self, device_name: &str, has_irq: bool) -> Option<CommonDeviceInfo> {
        let node = self
            .all_nodes()
            .find(|node| node.name.starts_with(device_name))?;
        let reg = node.reg()?.next()?;
        let range = Range {
            start: PhysAddr::from(reg.starting_address as usize),
            end: PhysAddr::from(reg.starting_address as usize + reg.size.unwrap()),
        };
        let irq = if has_irq {
            let irq = node.property("interrupts").unwrap().value;
            let irq = u32::from_be_bytes(irq[0..4].try_into().unwrap());
            Some(irq)
        } else {
            None
        };
        let compatible = node.compatible().map(Compatible::first).unwrap();

        let device_info = CommonDeviceInfo {
            address_range: range,
            irq,
            compatible: Some(compatible.to_string()),
        };

        Some(device_info)
    }

    fn probe_pci(&self) -> Option<CommonDeviceType> {
        // riscv64 侧只负责找到 PCI 设备树节点，再把它转换成统一的 ECAM 设备描述。
        // 真正“怎么扫描配置空间”仍然由 `kernel::bus::pci` 统一处理。
        self.probe_common("pci", false).map(ecam_device)
    }

    #[cfg(all(plat_vf2, plat_vf2_sd))]
    fn probe_sd(&self) -> Option<CommonDeviceType> {
        match self.probe_common("sdio1", true) {
            Some(device_info) => Some(CommonDeviceType::SdCard(device_info)),
            None => None,
        }
    }
}
