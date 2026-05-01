extern crate alloc;

use alloc::{string::String, vec::Vec, vec};

use acpi::AcpiTables;
use mem::PhysAddr;

use crate::bus::{CommonDeviceInfo, CommonDeviceType, DeviceLocator, pci::ecam_device};
use self::{
    acpi_enumerate::{acpi_tables, enumerate_apic, enumerate_hpet, enumerate_rtc, enumerate_uart},
    aml_enumerate::enumerate_aml_devices,
};

mod aml_handler;
mod acpi_hander;
mod descriptor_parser;
mod aml_enumerate;
mod acpi_enumerate;
mod pci_enumerate;

type BusAcpiTables = AcpiTables<acpi_hander::AcpiHost>;

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_static_devices() -> Vec<CommonDeviceType> {
    // 步骤1：获取 ACPI 表入口。
    let tables = acpi_tables();
    let mut devices = Vec::new();

    if let Some(tables) = tables {
        // 步骤2：按“静态表优先”顺序枚举 APIC/HPET/UART/RTC。
        debug!("[bus][x86_64] ACPI tables ready, enumerate static devices from MADT/SPCR/HPET/FADT");
        devices.extend(enumerate_apic(tables));
        devices.extend(enumerate_hpet(tables));
        devices.extend(enumerate_uart(tables));
        devices.extend(enumerate_rtc(tables));
    } else {
        warn!("[bus][x86_64] ACPI tables unavailable, fallback to empty static device list");
    }

    devices
}

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_uart_from_aml() -> Option<CommonDeviceType> {
    // 步骤1：复用 ACPI 表，进入 AML 回退路径。
    let tables = acpi_tables();
    if let Some(tables) = tables {
        warn!("[bus][x86_64][acpi] enter AML UART fallback path");
        return enumerate_aml_devices(tables)
            .into_iter()
            .find(|dev| matches!(dev, CommonDeviceType::Uart(_)));
    }
    warn!("[bus][x86_64][acpi] ACPI tables unavailable, AML UART fallback skipped");
    None
}

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_pci_devices() -> Vec<CommonDeviceType> {
    // 步骤1：优先从 ACPI MCFG 枚举 PCI ECAM。
    if let Some(tables) = acpi_tables() {
        debug!("[bus][x86_64][acpi][pci] ACPI tables ready, try MCFG first");
        return pci_enumerate::enumerate_pci_devices(tables);
    }

    // 步骤2：ACPI 不可用时使用固定 ECAM 兜底。
    let base = platform::config::PCI_ECAM_BASE;
    let size = 0x1000_0000usize;
    warn!(
        "[bus][x86_64][acpi][pci] ACPI tables unavailable, fallback ecam={:#x}..{:#x}",
        base,
        base + size
    );
    vec![ecam_device(CommonDeviceInfo {
        address_range: PhysAddr::from(base)..PhysAddr::from(base + size),
        locator: DeviceLocator::Mmio(PhysAddr::from(base)..PhysAddr::from(base + size)),
        irq: None,
        compatible: Some("pci_ecam".into()),
    })]
}

/// 函数说明：执行对应的总线处理步骤。
fn make_common_device(
    base: usize,
    size: usize,
    locator: DeviceLocator,
    irq: Option<u32>,
    compatible: Option<&str>,
) -> CommonDeviceInfo {
    // 步骤1：把原始地址与中断信息统一封装成通用设备描述。
    debug!(
        "[bus][x86_64] make_common_device: base={:#x}, size={:#x}, irq={:?}, compatible={:?}",
        base,
        size,
        irq,
        compatible
    );
    CommonDeviceInfo {
        address_range: PhysAddr::from(base)..PhysAddr::from(base + size),
        locator,
        irq,
        compatible: compatible.map(String::from),
    }
}
