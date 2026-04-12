use core::{fmt::Write, str::FromStr};

use alloc::{boxed::Box, string::String, vec::Vec};
use aml::{
    AmlContext, AmlError, AmlName, AmlValue, DebugVerbosity, LevelType,
    value::{Args, StatusObject},
};
use mem::PhysAddr;
use spin::Lazy;

use crate::bus::{CommonDeviceInfo, CommonDeviceType, DeviceLocator};

use super::{BusAcpiTables, aml_handler, descriptor_parser};

const HID: Lazy<AmlName> = Lazy::new(|| AmlName::from_str("_HID").unwrap());
const CID: Lazy<AmlName> = Lazy::new(|| AmlName::from_str("_CID").unwrap());
const STA: Lazy<AmlName> = Lazy::new(|| AmlName::from_str("_STA").unwrap());
const CRS: Lazy<AmlName> = Lazy::new(|| AmlName::from_str("_CRS").unwrap());

/// 函数说明：执行对应的总线处理步骤。
pub fn enumerate_aml_devices(tables: &BusAcpiTables) -> Vec<CommonDeviceType> {
    let mut ctx = AmlContext::new(Box::new(aml_handler::AmlHost), DebugVerbosity::None);
    let loaded = load_aml_tables(&mut ctx, tables);
    debug!(
        "[bus][x86_64][acpi] AML tables loaded: {}",
        loaded
    );

    if loaded > 0 {
        ctx.initialize_objects().unwrap();
        let mut devices = Vec::new();

        let _ = ctx.namespace.clone().traverse(|path, level| {
            if level.typ != LevelType::Device {
                return Ok(true);
            }

            trace!("[bus][x86_64][acpi][aml] visit device: {}", path);

            if let Some(device) = classify_aml_device(&mut ctx, path) {
                trace!("[bus][x86_64][acpi][aml] matched device: {}", path);
                devices.push(device);
            }

            Ok(true)
        });

        devices
    } else {
        warn!("[bus][x86_64][acpi] No AML tables loaded, skipping AML device enumeration");
        Vec::new()
    }
    
}

/// 函数说明：执行对应的总线处理步骤。
fn classify_aml_device(ctx: &mut AmlContext, path: &AmlName) -> Option<CommonDeviceType> {
    // 统一提取设备上下文，后续分类器共享这批信息。
    let ids = lookup_aml_ids(ctx, path);
    let sta = lookup_aml_value(ctx, path, &STA)
        .and_then(|(abs_path, value)| handle_sta_amlvalue(ctx, &abs_path, value));
    if ids.is_empty() {
        warn!("[bus][x86_64][acpi][aml] {} has no _HID/_CID", path);
    } else {
        trace!("[bus][x86_64][acpi][aml] {} ids={:?}", path, ids);
    }
    if !should_enumerate_by_sta(path, sta) {
        return None;
    }

    let path_str = path.as_string();
    let uart_hint = uart_hint_by_path(&path_str);
    let is_uart_hid = ids
        .iter()
        .any(|id| matches!(id.as_str(), "PNP0500" | "PNP0501" | "PNP0502" | "PNP0503"));
    let is_uart_by_name = uart_hint.is_some();

    if is_uart_hid {
        debug!("[bus][x86_64][acpi][aml] {} matched UART HID", path);
    }
    if let Some((hint, _, _)) = uart_hint {
        debug!("[bus][x86_64][acpi][aml] {} matched UART path hint {}", path, hint);
    }

    // 仅在 UART 候选设备上读取/执行 _CRS，避免无关设备触发 AML 方法副作用。
    if is_uart_hid || is_uart_by_name {
        let crs = lookup_aml_value(ctx, path, &CRS)
            .and_then(|(abs_path, value)| handle_crs_amlvalue(ctx, &abs_path, value));
        if let Some(dev) = classify_uart_device(ctx, path, is_uart_hid, uart_hint, crs.as_ref()) {
            return Some(dev);
        }
    }

    classify_known_non_uart_device(path, &ids);
    None
}

/// 函数说明：执行对应的总线处理步骤。
fn should_enumerate_by_sta(path: &AmlName, sta: Option<Option<StatusObject>>) -> bool {
    match sta {
        None => {
            trace!(
                "[bus][x86_64][acpi][aml] {} _STA missing, use default(enumerate=true)",
                path
            );
            true
        }
        Some(None) => {
            warn!(
                "[bus][x86_64][acpi][aml] {} _STA invalid, default allow enumeration",
                path
            );
            true
        }
        Some(Some(status)) => {
            trace!(
                "[bus][x86_64][acpi][aml] {} _STA parsed: present={}, enabled={}, functional={}",
                path,
                status.present,
                status.enabled,
                status.functional
            );
            if status.present && status.enabled {
                true
            } else {
                warn!(
                    "[bus][x86_64][acpi][aml] {} filtered by _STA: present={}, enabled={}",
                    path,
                    status.present,
                    status.enabled
                );
                false
            }
        }
    }
}

/// 函数说明：执行对应的总线处理步骤。
fn classify_uart_device(
    ctx: &AmlContext,
    path: &AmlName,
    is_uart_hid: bool,
    uart_hint: Option<(&'static str, usize, Option<u32>)>,
    crs: Option<&AmlValue>,
) -> Option<CommonDeviceType> {
    let is_uart_by_name = uart_hint.is_some();

    if !is_uart_hid && !is_uart_by_name {
        return None;
    }

    if is_uart_by_name && !is_uart_hid {
        warn!(
            "[bus][x86_64][acpi][aml] {} has no UART HID, fallback by ACPI name",
            path
        );
    }

    let (fallback_base, fallback_irq) = uart_hint
        .map(|(_, base, irq)| (base, irq))
        .unwrap_or((0x3f8usize, Some(4)));

    let (address, size, irq) = if let Some(value) = crs {
        let address = descriptor_parser::first_io_port_base(ctx, value);
        let size = descriptor_parser::first_io_port_length(ctx, value);
        let irq = descriptor_parser::first_irq(ctx, value).or(fallback_irq);

        if address.is_none() {
            warn!(
                "[bus][x86_64][acpi][aml] UART _CRS missing io base, fallback {:#x}",
                fallback_base
            );
        }
        if size.is_none() {
            warn!("[bus][x86_64][acpi][aml] UART _CRS missing io length, fallback 8");
        }
        if irq.is_none() {
            warn!("[bus][x86_64][acpi][aml] UART _CRS missing irq, keep irq=None");
        }
        (
            address.unwrap_or(fallback_base),
            size.unwrap_or(8),
            irq,
        )
    } else {
        warn!(
            "[bus][x86_64][acpi][aml] UART missing _CRS, fallback io={:#x}, len=8, irq={:?}",
            fallback_base,
            fallback_irq
        );
        (fallback_base, 8, fallback_irq)
    };

    trace!(
        "[bus][x86_64][acpi][aml] UART detected: path={}, io={:#x}..{:#x}, irq={:?}",
        path,
        address,
        address.saturating_add(size),
        irq
    );

    let end = address.saturating_add(size);
    let Ok(start_port) = u16::try_from(address) else {
        warn!(
            "[bus][x86_64][acpi][aml] UART io base out of range, skip: {:#x}",
            address
        );
        return None;
    };
    let Ok(end_port) = u16::try_from(end) else {
        warn!(
            "[bus][x86_64][acpi][aml] UART io end out of range, skip: {:#x}",
            end
        );
        return None;
    };

    Some(CommonDeviceType::Uart(CommonDeviceInfo {
        address_range: PhysAddr::from(address)..PhysAddr::from(end),
        locator: DeviceLocator::Pio(start_port..end_port),
        irq,
        compatible: Some("ns16550a".into()),
    }))
}

/// 函数说明：执行对应的总线处理步骤。
fn uart_hint_by_path(path: &str) -> Option<(&'static str, usize, Option<u32>)> {
    if path.ends_with(".COM1") {
        return Some(("COM1", 0x3f8, Some(4)));
    }
    if path.ends_with(".COMA") {
        return Some(("COMA", 0x3f8, Some(4)));
    }
    if path.ends_with(".COM2") {
        return Some(("COM2", 0x2f8, Some(3)));
    }
    if path.ends_with(".COMB") {
        return Some(("COMB", 0x2f8, Some(3)));
    }
    if path.ends_with(".COM3") {
        return Some(("COM3", 0x3e8, Some(4)));
    }
    if path.ends_with(".COMC") {
        return Some(("COMC", 0x3e8, Some(4)));
    }
    if path.ends_with(".COM4") {
        return Some(("COM4", 0x2e8, Some(3)));
    }
    if path.ends_with(".COMD") {
        return Some(("COMD", 0x2e8, Some(3)));
    }
    None
}

/// 函数说明：执行对应的总线处理步骤。
fn classify_known_non_uart_device(path: &AmlName, ids: &[String]) {
    // 这里只做“可识别但暂未接入 CommonDeviceType”的发现记录，方便后续扩展。
    let known = ids.iter().find_map(|id| match id.as_str() {
        "PNP0303" | "PNP030B" => Some("ps2-keyboard"),
        "PNP0F03" | "PNP0F13" => Some("ps2-mouse"),
        "PNP0B00" => Some("rtc"),
        "PNP0400" | "PNP0401" => Some("parallel-port"),
        "PNP0700" => Some("floppy"),
        _ => None,
    });

    if let Some(kind) = known {
        trace!(
            "[bus][x86_64][acpi][aml] recognized {} @ {} (not mapped to CommonDeviceType yet)",
            kind,
            path
        );
    }
}

// 在path下查找name对象, 返回绝对路径和对象值
fn lookup_aml_value(ctx: &mut AmlContext, path: &AmlName, name: &AmlName) -> Option<(AmlName, AmlValue)> {
    let lookup_name = name.clone();
    // 仅负责“取对象”，不做方法求值，调用方自行决定如何解释该对象。
    // 快速路径: 先尝试直接解析 path 下的 name 对象
    if let Ok(abs) = lookup_name.resolve(path) {
        if let Ok(value) = ctx.namespace.get_by_path(&abs).cloned() {
            return Some((abs, value));
        }
    }

    // 慢速路径: 从 path 逐级向上查找，返回首个命中的对象
    let (resolved_path, handle) = ctx.namespace.search(&lookup_name, path).ok()?;
    let value = ctx.namespace.get(handle).ok()?.clone();
    Some((resolved_path, value))
}

/// 函数说明：执行对应的总线处理步骤。
fn lookup_aml_ids(ctx: &mut AmlContext, path: &AmlName) -> Vec<String> {
    let mut ids = Vec::new();

    // 先查找_HID
    if let Some((abs_path, value)) = lookup_aml_value(ctx, path, &HID) {
        handle_hid_amlvalue(ctx, &abs_path, value, &mut ids);
    }
    // 再查找_CID
    if let Some((abs_path, value)) = lookup_aml_value(ctx, path, &CID) {
        handle_cid_amlvalue(ctx, &abs_path, value, &mut ids);
    }

    ids
}

// _HID 类型
// String: 直接是如 "PNP0501"
// Integer: EISA ID 编码整数（需解码）
fn handle_hid_amlvalue(
    ctx: &mut AmlContext,
    path: &AmlName,
    value: AmlValue,
    ids: &mut Vec<String>,
) {
    match value {
        // 如果是字符串或整数，直接尝试解析设备ID
        AmlValue::String(_) | AmlValue::Integer(_) => {
            push_id_from_scalar(ctx, path, &value, ids);
        }
        _ => {
            warn!(
                "[bus][x86_64][acpi][aml] {}._HID has unsupported AML type {:?}, skip",
                path,
                value.type_of()
            );
        }
    }
}

// _CID 类型
// String / Integer
// Package: 包含多个兼容ID（每项可为 String/Integer）
fn handle_cid_amlvalue(
    ctx: &mut AmlContext,
    path: &AmlName,
    value: AmlValue,
    ids: &mut Vec<String>,
) {
    match value {
        AmlValue::String(_) | AmlValue::Integer(_) => {
            push_id_from_scalar(ctx, path, &value, ids);
        }
        AmlValue::Package(items) => {
            for item in items {
                handle_cid_amlvalue(ctx, path, item, ids);
            }
        }
        _ => {
            warn!(
                "[bus][x86_64][acpi][aml] {}._CID has unsupported AML type {:?}, skip",
                path,
                value.type_of()
            );
        }
    }
}

// _CRS 类型
// Buffer: 资源描述符字节流（最常见）
// Method(0 arg): 动态生成并返回 Buffer
fn handle_crs_amlvalue(ctx: &mut AmlContext, path: &AmlName, value: AmlValue) -> Option<AmlValue> {
    match value {
        AmlValue::Buffer(buffer) => Some(AmlValue::Buffer(buffer)),
        AmlValue::Method { flags, .. } => {
            if flags.arg_count() != 0 {
                warn!("[bus][x86_64][acpi][aml] {} is method(arg_count={}), skip", path, flags.arg_count());
                return None;
            }

            let ret = match ctx.invoke_method(path, Args::EMPTY) {
                Ok(v) => v,
                Err(e) => {
                    warn!("[bus][x86_64][acpi][aml] invoke {} failed: {:?}", path, e);
                    return None;
                }
            };

            if ret.as_buffer(ctx).is_ok() {
                Some(ret)
            } else {
                warn!("[bus][x86_64][acpi][aml] {} method returned unsupported AML type {:?}, skip", path, ret.type_of());
                None
            }
        },
        other => {
            if other.as_buffer(ctx).is_ok() {
                Some(other)
            } else {
                warn!("[bus][x86_64][acpi][aml] {} has unsupported AML type {:?}, skip", path, other.type_of());
                None
            }
        }
    }
}

// _STA 类型
// Integer: 状态位图（as_status）
// Method(0 arg): 动态生成并返回 Integer
// 返回:
// - None: _STA 缺失
// - Some(None): _STA 存在但无效
// - Some(Some(status)): _STA 解析成功
fn handle_sta_amlvalue(
    ctx: &mut AmlContext,
    path: &AmlName,
    value: AmlValue,
) -> Option<Option<StatusObject>> {
    let status = match value {
        AmlValue::Integer(_) => value.as_status().ok(),
        AmlValue::Method { flags, .. } => {
            if flags.arg_count() != 0 {
                warn!(
                    "[bus][x86_64][acpi][aml] {}._STA is method(arg_count={}), invalid",
                    path,
                    flags.arg_count()
                );
                return Some(None);
            }
            match ctx.invoke_method(path, Args::EMPTY) {
                Ok(v) => v.as_status().ok(),
                Err(e) => {
                    warn!(
                        "[bus][x86_64][acpi][aml] invoke {}._STA failed: {:?}",
                        path,
                        e
                    );
                    None
                }
            }
        }
        _ => {
            warn!(
                "[bus][x86_64][acpi][aml] {}._STA has unsupported AML type {:?}",
                path,
                value.type_of()
            );
            None
        }
    };
    Some(status)
}

/// 函数说明：执行对应的总线处理步骤。
fn push_id_from_scalar(
    ctx: &mut AmlContext,
    path: &AmlName,
    value: &AmlValue,
    ids: &mut Vec<String>,
) {
    let id = if let Ok(id) = value.as_string(ctx) {
        Some(id)
    } else if let Ok(raw) = value.as_integer(ctx) {
        eisa_id_to_string(raw)
    } else {
        None
    };

    if let Some(id) = id {
        if !ids.iter().any(|x| x == &id) {
            trace!("[bus][x86_64][acpi][aml] {}={}", path, id);
            ids.push(id);
        }
    }
}

/// 函数说明：执行对应的总线处理步骤。
fn eisa_id_to_string(raw: u64) -> Option<String> {
    // AML 的 EISAID 是按小端整数存储，需先做字节序转换再按位解码
    let value = u32::try_from(raw).ok()?.swap_bytes();
    let c1 = ((value >> 26) & 0x1f) as u8;
    let c2 = ((value >> 21) & 0x1f) as u8;
    let c3 = ((value >> 16) & 0x1f) as u8;
    if c1 == 0 || c2 == 0 || c3 == 0 {
        return None;
    }

    let mut hid = String::with_capacity(7); // 7字节
    hid.push((c1 + 0x40) as char);
    hid.push((c2 + 0x40) as char);
    hid.push((c3 + 0x40) as char);
    let _ = write!(&mut hid, "{:04X}", value & 0xffff);
    trace!("[bus][x86_64][acpi][aml] decoded EISAID: {:#x} -> {}", raw, hid);
    Some(hid)
}

/// 函数说明：执行对应的总线处理步骤。
fn load_aml_tables(ctx: &mut AmlContext, tables: &BusAcpiTables) -> usize {
    let mut loaded = 0usize;

    if let Ok(dsdt) = tables.dsdt() {
        debug!("[bus][x86_64][acpi] DSDT found");
        if load_aml_table(ctx, dsdt).is_ok() {
            debug!("[bus][x86_64][acpi] DSDT loaded");
            loaded += 1;
        }
    }

    for ssdt in tables.ssdts() {
        debug!("[bus][x86_64][acpi] SSDT found");
        if load_aml_table(ctx, ssdt).is_ok() {
            debug!("[bus][x86_64][acpi] SSDT loaded");
            loaded += 1;
        }
    }

    loaded
}

/// 函数说明：执行对应的总线处理步骤。
fn load_aml_table(ctx: &mut AmlContext, table: acpi::AmlTable) -> Result<(), AmlError> {
    let raw = aml_handler::AmlHost::aml_table_bytes(table)?;
    ctx.parse_table(raw)
}
