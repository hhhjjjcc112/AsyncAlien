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
    Uart(CommonDeviceInfo),
    Rtc(CommonDeviceInfo),
    Pci(CommonDeviceInfo),
    // x86_64 的 virtio 通过 PCI transport 访问，bus 侧只记录枚举结果。
    Virtio(String),
}

fn parse_bdf(bdf: &str) -> Option<(u16, u8, u8, u8)> {
    let (seg, rest) = bdf.split_once(':')?;
    let (bus, rest) = rest.split_once(':')?;
    let (dev, func) = rest.split_once('.')?;
    let segment = u16::from_str_radix(seg, 16).ok()?;
    let bus = u8::from_str_radix(bus, 16).ok()?;
    let device = u8::from_str_radix(dev, 16).ok()?;
    let function = func.parse::<u8>().ok()?;
    Some((segment, bus, device, function))
}

fn info_locator(info: &CommonDeviceInfo) -> DeviceLocator {
    let start = info.address_range.start.as_usize();
    let end = info.address_range.end.as_usize();
    if end <= 0x1_0000 && start < end {
        DeviceLocator::Pio((start as u16)..(end as u16))
    } else {
        DeviceLocator::Mmio(info.address_range.clone())
    }
}

fn from_common_device(ty: CommonDeviceType) -> DiscoveredDevice {
    match ty {
        CommonDeviceType::LocalApic(info) => DiscoveredDevice {
            class: DeviceClass::LocalApic,
            locator: info_locator(&info),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Acpi,
        },
        CommonDeviceType::IoApic(info) => DiscoveredDevice {
            class: DeviceClass::IoApic,
            locator: info_locator(&info),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Acpi,
        },
        CommonDeviceType::Uart(info) => DiscoveredDevice {
            class: DeviceClass::Uart,
            locator: info_locator(&info),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Acpi,
        },
        CommonDeviceType::Rtc(info) => DiscoveredDevice {
            class: DeviceClass::Rtc,
            locator: info_locator(&info),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Acpi,
        },
        CommonDeviceType::Pci(info) => DiscoveredDevice {
            class: DeviceClass::PciHost,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Pci,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Acpi,
        },
        CommonDeviceType::Virtio(bdf) => {
            let locator = parse_bdf(&bdf)
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
    }
}

fn locator_to_info(locator: &DeviceLocator, irq: Option<u32>, compatible: Option<String>) -> Option<CommonDeviceInfo> {
    match locator {
        DeviceLocator::Mmio(range) => Some(CommonDeviceInfo {
            address_range: range.clone(),
            irq,
            compatible,
        }),
        DeviceLocator::Pio(range) => Some(CommonDeviceInfo {
            address_range: mem::PhysAddr::from(range.start as usize)
                ..mem::PhysAddr::from(range.end as usize),
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
        DeviceClass::VirtioPci => {
            if let DeviceLocator::PciBdf {
                segment,
                bus,
                device,
                function,
            } = dev.locator
            {
                debug!(
                    "[bus][x86_64][virtio] detected virtio @ {:04x}:{:02x}:{:02x}.{}",
                    segment, bus, device, function
                );
            } else {
                debug!("[bus][x86_64][virtio] detected virtio with unknown bdf");
            }
        }
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
        irq: Some(4),
        compatible: Some("ns16550a".into()),
    }));
}

/// 函数说明：执行对应的总线处理步骤。
fn log_probe(devices: &[CommonDeviceType]) {
    // 步骤1：统计总线设备数量并输出关键地址信息。
    let mut local_apic = 0usize;
    let mut io_apic = 0usize;
    let mut uart = 0usize;
    let mut rtc = 0usize;
    let mut pci_ecam = 0usize;
    let mut virtio = 0usize;

    for ty in devices {
        match ty {
            CommonDeviceType::LocalApic(info) => {
                local_apic += 1;
                debug!(
                    "[bus][x86_64][probe] local_apic @ {:#x}..{:#x}",
                    info.address_range.start.as_usize(),
                    info.address_range.end.as_usize()
                );
            }
            CommonDeviceType::IoApic(info) => {
                io_apic += 1;
                debug!(
                    "[bus][x86_64][probe] io_apic @ {:#x}..{:#x}",
                    info.address_range.start.as_usize(),
                    info.address_range.end.as_usize()
                );
            }
            CommonDeviceType::Uart(info) => {
                uart += 1;
                debug!(
                    "[bus][x86_64][probe] uart @ {:#x}..{:#x}, irq={:?}, compatible={:?}",
                    info.address_range.start.as_usize(),
                    info.address_range.end.as_usize(),
                    info.irq,
                    info.compatible
                );
            }
            CommonDeviceType::Pci(info) => {
                pci_ecam += 1;
                debug!(
                    "[bus][x86_64][probe] pci_ecam @ {:#x}..{:#x}",
                    info.address_range.start.as_usize(),
                    info.address_range.end.as_usize()
                );
            }
            CommonDeviceType::Rtc(info) => {
                rtc += 1;
                debug!(
                    "[bus][x86_64][probe] rtc @ {:#x}..{:#x}, irq={:?}, compatible={:?}",
                    info.address_range.start.as_usize(),
                    info.address_range.end.as_usize(),
                    info.irq,
                    info.compatible
                );
            }
            CommonDeviceType::Virtio(bdf) => {
                virtio += 1;
                debug!("[bus][x86_64][probe] virtio @ {}", bdf);
            }
        }
    }

    debug!(
        "[bus][x86_64][probe] summary: local_apic={}, io_apic={}, uart={}, rtc={}, pci_ecam={}, virtio={}",
        local_apic,
        io_apic,
        uart,
        rtc,
        pci_ecam,
        virtio
    );
}

/// 函数说明：执行对应的总线处理步骤。
pub fn init_with_acpi() -> AlienResult<()> {
    // 步骤1：优先走 ACPI 静态表，收集基础设备。
    debug!("[bus][x86_64][init_with_acpi] step1: enumerate ACPI static devices");
    let mut base_devices = acpi::enumerate_static_devices();

    // 步骤2：处理 ACPI 静态表失败时的回退逻辑。
    debug!("[bus][x86_64][init_with_acpi] step2: apply aml fallback");
    fallback_with_aml(&mut base_devices);

    // 步骤3：显式执行 PCI 枚举步骤并并入基础设备集合。
    debug!("[bus][x86_64][init_with_acpi] step3: enumerate PCI devices");
    base_devices.extend(acpi::enumerate_pci_devices());

    // 步骤4：注册基础设备（APIC/RTC/UART/PCI ECAM）。
    debug!("[bus][x86_64][init_with_acpi] step4: register base devices");
    let discovered_base = base_devices
        .clone()
        .into_iter()
        .map(from_common_device)
        .collect();
    register_discovered_devices(discovered_base);

    // 步骤5：基于 PCI 端点收集 virtio 设备。
    debug!("[bus][x86_64][init_with_acpi] step5: collect virtio devices from PCI endpoints");
    let virtio_devices = pci::collect_virtio_devices();
    let discovered_virtio = virtio_devices
        .clone()
        .into_iter()
        .map(from_common_device)
        .collect();
    register_discovered_devices(discovered_virtio);

    // 步骤6：输出汇总日志，便于链路验收。
    debug!("[bus][x86_64][init_with_acpi] step6: print consolidated probe summary");
    let mut all_devices = base_devices;
    all_devices.extend(virtio_devices);
    log_probe(&all_devices);

    pci::log_virtio_summary();
    Ok(())
}
