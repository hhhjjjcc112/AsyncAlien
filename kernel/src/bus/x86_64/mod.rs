use alloc::string::String;

use crate::{
    bus::CommonDeviceInfo,
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

/// 函数说明：执行对应的总线处理步骤。
fn register_detected_devices(devices: alloc::vec::Vec<CommonDeviceType>) {
    // 步骤1：把统一设备枚举结果分发到 platform/pci 子总线。
    devices.into_iter().for_each(|ty| match ty {
        CommonDeviceType::LocalApic(info) => platform::register_platform_device(info, "local_apic"),
        CommonDeviceType::IoApic(info) => platform::register_platform_device(info, "io_apic"),
        CommonDeviceType::Uart(info) => platform::register_platform_device(info, "uart"),
        CommonDeviceType::Rtc(info) => platform::register_platform_device(info, "rtc"),
        CommonDeviceType::Pci(info) => pci::pci_init(info),
        CommonDeviceType::Virtio(bdf) => {
            debug!("[bus][x86_64][virtio] detected virtio @ {}", bdf);
        }
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
    debug!("[bus][x86_64][fallback] UART missing in static tables, try AML fallback");
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
    register_detected_devices(base_devices.clone());

    // 步骤5：基于 PCI 端点收集 virtio 设备。
    debug!("[bus][x86_64][init_with_acpi] step5: collect virtio devices from PCI endpoints");
    let virtio_devices = pci::collect_virtio_devices();
    register_detected_devices(virtio_devices.clone());

    // 步骤6：输出汇总日志，便于链路验收。
    debug!("[bus][x86_64][init_with_acpi] step6: print consolidated probe summary");
    let mut all_devices = base_devices;
    all_devices.extend(virtio_devices);
    log_probe(&all_devices);

    pci::log_virtio_summary();
    Ok(())
}
