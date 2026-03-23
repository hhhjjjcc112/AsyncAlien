extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};

use aml::{AmlContext, AmlName, DebugVerbosity, LevelType};
use mem::PhysAddr;

use crate::bus::{CommonDeviceInfo, CommonDeviceType};

mod aml_handler;
mod descriptor_parser;
mod pci;

pub fn enumerate_devices() -> Vec<CommonDeviceType> {
    // 先把平台层给出的静态表信息转成统一设备描述。
    // 这一步只负责“看见硬件”，不做任何控制方法求值。
    let static_info = platform::acpi::device_info();
    let mut devices = enumerate_static_devices(&static_info);

    // 再使用完整 ACPI 表做 AML 和 PCI 枚举。
    // 这是 bus 侧的核心职责，因为这里已经具备更完整的内存与设备抽象。
    if let Some(tables) = platform::acpi::tables() {
        let mut aml = AmlContext::new(Box::new(aml_handler::AmlHost), DebugVerbosity::None);
        let loaded = load_aml_tables(&mut aml, tables);

        if loaded > 0 {
            // 先把控制方法依赖的对象初始化好，再遍历 namespace 提取设备。
            let _ = aml.initialize_objects();
            devices.extend(enumerate_aml_devices(&mut aml));
        }

        // PCI 设备不从 AML namespace 里找，而是从 MCFG/ECAM 里单独枚举。
        devices.extend(pci::enumerate_pci_devices(tables));
    }

    devices
}

fn enumerate_static_devices(info: &platform::acpi::AcpiDeviceInfo) -> Vec<CommonDeviceType> {
    let mut devices = Vec::new();

    // LAPIC / IOAPIC / HPET 这类核心设备先由静态表确认，后续 domain 只需要消费结果。
    for entry in info.devices.entries.iter() {
        let device = match entry.name {
            "local_apic" => Some(CommonDeviceType::LocalApic(make_common_device(
                entry.base,
                entry.size,
                None,
                Some("local_apic"),
            ))),
            "io_apic" => Some(CommonDeviceType::IoApic(make_common_device(
                entry.base,
                entry.size,
                None,
                Some("io_apic"),
            ))),
            "hpet" => Some(CommonDeviceType::Hpet(make_common_device(
                entry.base,
                entry.size,
                None,
                Some("hpet"),
            ))),
            _ => None,
        };

        if let Some(device) = device {
            devices.push(device);
        }
    }

    devices
}

fn enumerate_aml_devices(ctx: &mut AmlContext) -> Vec<CommonDeviceType> {
    let mut devices = Vec::new();

    // namespace.traverse 会把所有 namespace level 走一遍。
    // 我们只关心 Device 节点，其余节点留给后续阶段或其他子系统。
    let _ = ctx.namespace.clone().traverse(|path, level| {
        if level.typ != LevelType::Device {
            return Ok(true);
        }

        // 每个 Device 节点再按 _HID/_CRS 细分成平台可消费的统一设备类型。
        if let Some(device) = classify_aml_device(ctx, path) {
            devices.push(device);
        }

        Ok(true)
    });

    devices
}

fn classify_aml_device(ctx: &mut AmlContext, path: &AmlName) -> Option<CommonDeviceType> {
    let hid = lookup_aml_string(ctx, path, "_HID")?;
    let crs = lookup_aml_value(ctx, path, "_CRS");

    // 这里先只覆盖对启动最重要的基础设备。
    // 未来如果要加 SATA / NVMe / GPU，只需要在这里扩展 HID 分类即可。
    if matches!(hid.as_str(), "PNP0500" | "PNP0501" | "PNP0502" | "PNP0503") {
        // 串口通常由 _CRS 给出 IO 端口和 IRQ；没有 _CRS 时就退回 COM1 默认值。
        let (address, size, irq) = if let Some(value) = crs.as_ref() {
            (
                descriptor_parser::first_io_port_base(ctx, value).unwrap_or(0x3f8),
                descriptor_parser::first_io_port_length(ctx, value).unwrap_or(8),
                descriptor_parser::first_irq(ctx, value),
            )
        } else {
            (0x3f8, 8, None)
        };

        return Some(CommonDeviceType::Uart(CommonDeviceInfo {
            address_range: PhysAddr::from(address)..PhysAddr::from(address.saturating_add(size)),
            irq,
            compatible: Some("ns16550a".into()),
        }));
    }

    if matches!(hid.as_str(), "PNP0B00" | "PNP0B01") {
        // RTC 也是通过 _CRS 描述实际资源，不提供时保留兼容默认值。
        let (address, size, irq) = if let Some(value) = crs.as_ref() {
            (
                descriptor_parser::first_io_port_base(ctx, value).unwrap_or(0x70),
                descriptor_parser::first_io_port_length(ctx, value).unwrap_or(8),
                descriptor_parser::first_irq(ctx, value),
            )
        } else {
            (0x70, 8, None)
        };

        return Some(CommonDeviceType::Rtc(CommonDeviceInfo {
            address_range: PhysAddr::from(address)..PhysAddr::from(address.saturating_add(size)),
            irq,
            compatible: Some("rtc".into()),
        }));
    }

    None
}

fn lookup_aml_value(ctx: &mut AmlContext, path: &AmlName, name: &str) -> Option<aml::AmlValue> {
    // 按当前设备路径查找控制方法或对象值，供 _HID/_CRS 等字段读取。
    let lookup_name = AmlName::from_str(name).ok()?;
    let (_, handle) = ctx.namespace.search(&lookup_name, path).ok()?;
    ctx.namespace.get(handle).ok().cloned()
}

fn lookup_aml_string(ctx: &mut AmlContext, path: &AmlName, name: &str) -> Option<String> {
    // 字符串型控制方法一般用来读取 HID/CID 之类的设备标识。
    let value = lookup_aml_value(ctx, path, name)?;
    value.as_string(ctx).ok()
}

fn load_aml_tables(ctx: &mut AmlContext, tables: &::acpi::AcpiTables<platform::acpi::AcpiHost>) -> usize {
    let mut loaded = 0usize;

    // 先加载 DSDT，再加载所有 SSDT，保证命名空间和设备定义按 ACPI 约定展开。
    if let Ok(dsdt) = tables.dsdt()
        && load_aml_table(ctx, dsdt).is_ok()
    {
        loaded += 1;
    }

    for ssdt in tables.ssdts() {
        if load_aml_table(ctx, ssdt).is_ok() {
            loaded += 1;
        }
    }

    loaded
}

fn load_aml_table(ctx: &mut AmlContext, table: ::acpi::AmlTable) -> Result<(), aml::AmlError> {
    // AmlContext 只需要 SDT 里的 AML 字节部分，头部由 ACPI 表格式自行处理。
    let raw = aml_handler::AmlHost::aml_table_bytes(table)?;
    ctx.parse_table(raw)
}

fn make_common_device(
    base: usize,
    size: usize,
    irq: Option<u32>,
    compatible: Option<&str>,
) -> CommonDeviceInfo {
    CommonDeviceInfo {
        address_range: PhysAddr::from(base)..PhysAddr::from(base + size),
        irq,
        compatible: compatible.map(String::from),
    }
}
