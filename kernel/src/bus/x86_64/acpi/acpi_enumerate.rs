use acpi::{
    AcpiTables,
    address::AddressSpace,
    rsdp::Rsdp,
    sdt::{
        fadt::Fadt,
        madt::{Madt, MadtEntry},
        spcr::{Spcr, SpcrInterfaceType},
    },
};
use alloc::vec::Vec;
use spin::Once;

use crate::bus::CommonDeviceType;

use super::{BusAcpiTables, acpi_hander, make_common_device};

const DEFAULT_LAPIC_BASE: usize = 0xfee0_0000;
const DEFAULT_IOAPIC_BASE: usize = 0xfec0_0000;
const APIC_MMIO_SIZE: usize = 0x1000;
const DEFAULT_RTC_IO_BASE: usize = 0x70;
const DEFAULT_RTC_IO_SIZE: usize = 0x2;
const DEFAULT_RTC_IRQ: u32 = 8;

static ACPI_TABLES: Once<Option<BusAcpiTables>> = Once::new();

// 初始化一次ACPI表
pub fn acpi_tables() -> Option<&'static BusAcpiTables> {
    // 步骤1：全局只初始化一次 ACPI 表，后续直接复用缓存。
    ACPI_TABLES.call_once(detect_acpi_tables);
    ACPI_TABLES.get().and_then(|tables| tables.as_ref())
}

// 通过bios搜索RSDP, 获取ACPI表
fn detect_acpi_tables() -> Option<BusAcpiTables> {
    // 步骤1：先从 BIOS 搜索 RSDP。
    let host = acpi_hander::AcpiHost;

    let rsdp = match unsafe { Rsdp::search_for_on_bios(host) } {
        Ok(rsdp) => rsdp,
        Err(e) => {
            warn!("[bus][x86_64][acpi] RSDP not found: {:?}", e);
            return None;
        }
    };

    // 步骤2：根据 RSDP 解析并构建 ACPI 表集合。
    match unsafe { AcpiTables::from_rsdp(host, rsdp.physical_start) } {
        Ok(tables) => Some(tables),
        Err(e) => {
            warn!("[bus][x86_64][acpi] parse tables failed: {:?}", e);
            None
        }
    }
}

// 通过acpi的MADT表获取local_apic和io_apic的地址
pub fn enumerate_apic(tables: &BusAcpiTables) -> Vec<CommonDeviceType> {
    // 步骤1：解析 MADT，获取 LAPIC/IOAPIC 地址。
    let mut devices = Vec::new();

    // 设置lapic的默认地址
    let mut lapic_base = DEFAULT_LAPIC_BASE;
    let mut ioapic_bases: Vec<usize> = Vec::new();

    // 查找MADT表
    if let Some(madt) = tables.find_table::<Madt>() {
        let madt = madt.get();
        // 首先读取MADT头部的local_apic_address字段作为lapic_base的初始值
        debug!(
            "[bus][x86_64][acpi] initial local_apic_address={:#x}",
            madt.local_apic_address as usize
        );
        lapic_base = madt.local_apic_address as usize;

        for entry in madt.entries() {
            match entry {
                // IO APIC的地址在MADT的IO APIC entry中
                MadtEntry::IoApic(ioapic) => {

                    ioapic_bases.push(ioapic.io_apic_address as usize);
                }
                // 如果存在local_apic_address override entry，则覆盖之前的lapic_base
                MadtEntry::LocalApicAddressOverride(override_entry) => {
                    debug!(
                        "[bus][x86_64][acpi] MADT local_apic_address override: {:#x}",
                        override_entry.local_apic_address as usize
                    );
                    lapic_base = override_entry.local_apic_address as usize;
                }
                _ => {}
            }
        }
    } else {
        warn!(
            "[bus][x86_64][acpi] MADT missing, fallback local_apic={:#x}, io_apic={:#x}",
            DEFAULT_LAPIC_BASE,
            DEFAULT_IOAPIC_BASE
        );
    }

    if ioapic_bases.is_empty() {
        warn!(
            "[bus][x86_64][acpi] IO APIC entry missing, fallback io_apic={:#x}",
            DEFAULT_IOAPIC_BASE
        );
        ioapic_bases.push(DEFAULT_IOAPIC_BASE);
    }

    // 步骤2：把解析结果转换成统一设备类型。
    devices.push(CommonDeviceType::LocalApic(make_common_device(
        lapic_base,
        APIC_MMIO_SIZE,
        None,
        Some("local_apic"),
    )));

    for base in ioapic_bases {
        devices.push(CommonDeviceType::IoApic(make_common_device(
            base,
            APIC_MMIO_SIZE,
            None,
            Some("io_apic"),
        )));
    }

    devices
}

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_uart(tables: &BusAcpiTables) -> Vec<CommonDeviceType> {
    // 步骤1：优先使用 SPCR 静态表解析串口信息。
    let mut devices = Vec::new();

    if let Some(spcr) = tables.find_table::<Spcr>() {
        let spcr = spcr.get();
        let interface = spcr.interface_type();
        let irq = spcr.irq().map(u32::from).or_else(|| spcr.global_system_interrupt());

        let (base, size) = match spcr.base_address() {
            Some(Ok(address)) => {
                let base = address.address as usize;
                match address.address_space {
                    // x86 常见 UART 是系统 I/O 端口，按 16550 8 字节窗口处理。
                    AddressSpace::SystemIo => (base, 8),
                    // 对 MMIO UART 保守给 8 字节，避免误扫过大范围。
                    AddressSpace::SystemMemory => (base, 8),
                    other => {
                        warn!(
                            "[bus][x86_64][acpi] SPCR unsupported address space: {:?}",
                            other
                        );
                        return devices;
                    }
                }
            }
            Some(Err(e)) => {
                warn!("[bus][x86_64][acpi] SPCR invalid base address: {:?}", e);
                return devices;
            }
            None => {
                warn!("[bus][x86_64][acpi] SPCR base address is empty");
                return devices;
            }
        };

        let compatible = uart_compatible_from_spcr(interface);
        debug!(
            "[bus][x86_64][acpi] SPCR UART: interface={:?}, base={:#x}, size={:#x}, irq={:?}, compatible={}",
            interface,
            base,
            size,
            irq,
            compatible
        );

        // 步骤2：封装 UART 设备并返回。
        devices.push(CommonDeviceType::Uart(make_common_device(
            base,
            size,
            irq,
            Some(compatible),
        )));
    } else {
        debug!("[bus][x86_64][acpi] SPCR missing, skip UART static enumeration");
    }

    devices
}

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_rtc(tables: &BusAcpiTables) -> Vec<CommonDeviceType> {
    // 步骤1：检测 FADT 是否存在，存在则接入 CMOS RTC 固定端口。
    let mut devices = Vec::new();

    if tables.find_table::<Fadt>().is_some() {
        // CMOS RTC 在 PC 架构是固定硬件，FADT 存在即可按固定 I/O 端口接入。
        devices.push(CommonDeviceType::Rtc(make_common_device(
            DEFAULT_RTC_IO_BASE,
            DEFAULT_RTC_IO_SIZE,
            Some(DEFAULT_RTC_IRQ),
            Some("cmos_rtc"),
        )));
    } else {
        debug!("[bus][x86_64][acpi] FADT missing, skip RTC static enumeration");
    }

    devices
}

/// 函数说明：执行对应的总线处理步骤。
fn uart_compatible_from_spcr(interface: SpcrInterfaceType) -> &'static str {
    // 步骤1：根据 SPCR 接口类型映射到驱动兼容串口名。
    match interface {
        SpcrInterfaceType::Full16550
        | SpcrInterfaceType::Full16450
        | SpcrInterfaceType::Nvidia16550
        | SpcrInterfaceType::Generic16550 => "ns16550a",
        _ => "uart",
    }
}
