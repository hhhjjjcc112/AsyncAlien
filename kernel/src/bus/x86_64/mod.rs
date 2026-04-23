use alloc::string::String;

use crate::{
    bus::{
        CommonDeviceInfo, DeviceClass, DeviceLocator, DeviceTransport, DiscoveredDevice,
        FirmwareSource,
    },
    error::AlienResult,
};

pub mod acpi;
pub mod mmio;
pub mod pci;
pub mod platform;

#[derive(Debug, Clone)]
pub enum CommonDeviceType {
    LocalApic(CommonDeviceInfo),
    IoApic(CommonDeviceInfo),
    Hpet(CommonDeviceInfo),
    Uart(CommonDeviceInfo),
    Rtc(CommonDeviceInfo),
    Pci(CommonDeviceInfo),
    #[cfg(feature = "bench")]
    Ramdisk(CommonDeviceInfo),
    #[cfg(feature = "domain_net_test")]
    LoopBack(CommonDeviceInfo),
    // x86_64 的 virtio 通过 PCI transport 访问，bus 侧只记录枚举结果。
    Virtio(String),
}

fn from_common_device(ty: CommonDeviceType) -> DiscoveredDevice {
    match ty {
        CommonDeviceType::LocalApic(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::LocalApic,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::IoApic(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::IoApic,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::Hpet(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::Hpet,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::Uart(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::Uart,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::Rtc(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::Rtc,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::Pci(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::PciHost,
                locator,
                transport: DeviceTransport::Pci,
                irq,
                compatible,
                fw_source: FirmwareSource::Acpi,
            }
        }
        CommonDeviceType::Virtio(bdf) => {
            let locator = x86_pci::parse_bdf(&bdf)
                .map(|(segment, bus, device, function)| DeviceLocator::PciBdf {
                    segment,
                    bus,
                    device,
                    function,
                })
                .unwrap_or(DeviceLocator::None);
            DiscoveredDevice {
                class: DeviceClass::VirtioPci,
                locator,
                transport: DeviceTransport::Pci,
                irq: None,
                compatible: Some("virtio-pci".into()),
                fw_source: FirmwareSource::PciScan,
            }
        }
        #[cfg(feature = "bench")]
        CommonDeviceType::Ramdisk(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::Ramdisk,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Synthetic,
            }
        }
        #[cfg(feature = "domain_net_test")]
        CommonDeviceType::LoopBack(info) => {
            let CommonDeviceInfo {
                locator,
                irq,
                compatible,
                ..
            } = info;
            DiscoveredDevice {
                class: DeviceClass::LoopBack,
                locator,
                transport: DeviceTransport::Platform,
                irq,
                compatible,
                fw_source: FirmwareSource::Synthetic,
            }
        }
    }
}

fn locator_to_info(locator: &DeviceLocator, irq: Option<u32>, compatible: Option<String>) -> Option<CommonDeviceInfo> {
    match locator {
        DeviceLocator::Mmio(range) => Some(CommonDeviceInfo {
            address_range: range.clone(),
            locator: DeviceLocator::Mmio(range.clone()),
            irq,
            compatible,
        }),
        DeviceLocator::Pio(range) => Some(CommonDeviceInfo {
            address_range: mem::PhysAddr::from(range.start as usize)
                ..mem::PhysAddr::from(range.end as usize),
            locator: DeviceLocator::Pio(range.clone()),
            irq,
            compatible,
        }),
        _ => None,
    }
}

/// 函数说明：执行对应的总线处理步骤。
fn register_discovered_devices(devices: alloc::vec::Vec<DiscoveredDevice>) {
    devices.into_iter().for_each(|dev| match dev.class {
        DeviceClass::LocalApic => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "local_apic");
            }
        }
        DeviceClass::IoApic => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "io_apic");
            }
        }
        DeviceClass::Hpet => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "hpet");
            }
        }
        DeviceClass::Uart => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "uart");
            }
        }
        DeviceClass::Rtc => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "rtc");
            }
        }
        DeviceClass::PciHost => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                pci::pci_init(info);
            }
        }
        #[cfg(feature = "bench")]
        DeviceClass::Ramdisk => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "ramdisk");
            }
        }
        #[cfg(feature = "domain_net_test")]
        DeviceClass::LoopBack => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "loopback");
            }
        }
        DeviceClass::VirtioPci => {}
        _ => {}
    });
}

/// 函数说明：执行对应的总线处理步骤。
fn fallback_with_aml(base_devices: &mut alloc::vec::Vec<CommonDeviceType>) {
    // 步骤1：当前先处理 UART 缺失场景，后续可继续补充其它设备回退。
    let has_uart = base_devices
        .iter()
        .any(|dev| matches!(dev, CommonDeviceType::Uart(_)));
    if has_uart {
        return;
    }

    // 步骤2：静态表缺 UART 时，回退 AML 解析路径。
    warn!("[bus][x86_64][fallback] UART missing in static tables, try AML fallback");
    if let Some(uart) = acpi::enumerate_uart_from_aml() {
        base_devices.push(uart);
        return;
    }

    // 步骤3：若 AML 也失败，使用 COM1 兜底，保证早期串口链路可用。
    warn!("[bus][x86_64][fallback] UART missing in ACPI/AML, fallback COM1");
    base_devices.push(CommonDeviceType::Uart(CommonDeviceInfo {
        address_range: mem::PhysAddr::from(0x3f8usize)..mem::PhysAddr::from(0x400usize),
        locator: DeviceLocator::Pio(0x3f8u16..0x400u16),
        irq: Some(4),
        compatible: Some("ns16550a".into()),
    }));
}

/// 函数说明：执行对应的总线处理步骤。
pub fn init_with_acpi() -> AlienResult<()> {
    let mut base_devices = acpi::enumerate_static_devices();

    fallback_with_aml(&mut base_devices);

    #[cfg(feature = "bench")]
    {
        let ramdisk_start = RAMDISK.as_ptr() as usize;
        let len = RAMDISK.len();
        base_devices.push(CommonDeviceType::Ramdisk(CommonDeviceInfo {
            address_range: mem::PhysAddr::from(ramdisk_start)
                ..mem::PhysAddr::from(ramdisk_start + len),
            locator: DeviceLocator::Mmio(
                mem::PhysAddr::from(ramdisk_start)..mem::PhysAddr::from(ramdisk_start + len),
            ),
            irq: None,
            compatible: None,
        }));
    }

    #[cfg(feature = "domain_net_test")]
    {
        let loopback_base = 0x1_0000_0000usize;
        let loopback_size = 0x1000usize;
        base_devices.push(CommonDeviceType::LoopBack(CommonDeviceInfo {
            address_range: mem::PhysAddr::from(loopback_base)
                ..mem::PhysAddr::from(loopback_base + loopback_size),
            locator: DeviceLocator::Mmio(
                mem::PhysAddr::from(loopback_base)
                    ..mem::PhysAddr::from(loopback_base + loopback_size),
            ),
            irq: None,
            compatible: None,
        }));
    }

    base_devices.extend(acpi::enumerate_pci_devices());

    let discovered_base = base_devices
        .clone()
        .into_iter()
        .map(from_common_device)
        .collect();
    register_discovered_devices(discovered_base);

    let virtio_devices = pci::collect_virtio_devices();
    let discovered_virtio = virtio_devices
        .clone()
        .into_iter()
        .map(from_common_device)
        .collect();
    register_discovered_devices(discovered_virtio);

    Ok(())
}

#[cfg(feature = "bench")]
static RAMDISK: &'static [u8] = include_bytes!("../../../../build/sdcard.img");
