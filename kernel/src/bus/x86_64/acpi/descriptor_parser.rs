use aml::{AmlContext, AmlValue};

// 这组函数只负责把 _CRS 里的资源缓冲区拆成可读字段。
// 资源的语义仍然由 AML/ACPI 决定，这里只做最小解析。
pub fn first_irq(ctx: &AmlContext, value: &AmlValue) -> Option<u32> {
    for descriptor in read_resource_bytes(ctx, value)? {
        if descriptor.kind == ResourceKind::Irq {
            return descriptor.irq;
        }
    }

    None
}

/// 函数说明：执行对应的总线处理步骤。
pub fn first_io_port_base(ctx: &AmlContext, value: &AmlValue) -> Option<usize> {
    for descriptor in read_resource_bytes(ctx, value)? {
        if descriptor.kind == ResourceKind::IoPort {
            return descriptor.base;
        }
    }

    None
}

/// 函数说明：执行对应的总线处理步骤。
pub fn first_io_port_length(ctx: &AmlContext, value: &AmlValue) -> Option<usize> {
    for descriptor in read_resource_bytes(ctx, value)? {
        if descriptor.kind == ResourceKind::IoPort {
            return descriptor.length;
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ResourceKind {
    Irq,
    IoPort,
}

#[derive(Copy, Clone)]
struct ParsedResource {
    kind: ResourceKind,
    irq: Option<u32>,
    base: Option<usize>,
    length: Option<usize>,
}

/// 函数说明：执行对应的总线处理步骤。
fn read_resource_bytes(ctx: &AmlContext, value: &AmlValue) -> Option<alloc::vec::Vec<ParsedResource>> {
    // _CRS 常见为 buffer 或 resource template；这里统一按字节流扫描。
    let buffer = match value.as_buffer(ctx) {
        Ok(buffer) => buffer,
        Err(_) => {
            warn!("[bus][x86_64][acpi][aml] _CRS is not buffer, skip descriptor parse");
            return None;
        }
    };
    let bytes = buffer.lock();
    let bytes = bytes.as_slice();

    let mut parsed: alloc::vec::Vec<ParsedResource> = alloc::vec::Vec::new();
    let mut index = 0usize;

    while index < bytes.len() {
        let tag = bytes[index];

        if tag == 0x79 {
            break;
        }

        if tag & 0x80 != 0 {
            // 大项描述符：0x80 置位后，高 7 位是类型号，后两字节是长度。
            let descriptor_type = tag & 0x7f;
            if index + 2 >= bytes.len() {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS large descriptor(type={:#x}) header truncated",
                    descriptor_type
                );
                break;
            }

            let length = u16::from_le_bytes([bytes[index + 1], bytes[index + 2]]) as usize;
            let payload_start = index + 3;
            let payload_end = payload_start.saturating_add(length);
            if payload_end > bytes.len() {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS large descriptor(type={:#x}) payload truncated",
                    descriptor_type
                );
                break;
            }

            // 最小实现只支持 UART 常用的 IRQ/IO/FixedIO，其他资源先忽略并告警。
            if descriptor_type == 0x06 {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS large descriptor(type=0x06, fixed memory) ignored in minimal parser"
                );
            } else {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS large descriptor(type={:#x}) ignored in minimal parser",
                    descriptor_type
                );
            }

            index = payload_end;
            continue;
        }

        let descriptor_type = (tag >> 3) & 0x0f;
        let length = (tag & 0x07) as usize;
        let payload_start = index + 1;
        let payload_end = payload_start.saturating_add(length);
        if payload_end > bytes.len() {
            warn!(
                "[bus][x86_64][acpi][aml] _CRS small descriptor(type={:#x}) payload truncated",
                descriptor_type
            );
            break;
        }

        match descriptor_type {
            0x04 if length >= 2 => {
                // IRQ descriptor：取出第一个置位的 IRQ 编号即可。
                let irq_mask = u16::from_le_bytes([bytes[payload_start], bytes[payload_start + 1]]);
                let irq = first_set_bit(irq_mask).map(|bit| bit as u32);
                parsed.push(ParsedResource {
                    kind: ResourceKind::Irq,
                    irq,
                    base: None,
                    length: None,
                });
            }
            0x04 => {
                warn!("[bus][x86_64][acpi][aml] _CRS IRQ descriptor length({}) invalid", length);
            }
            0x08 if length >= 7 => {
                // 传统 I/O 端口资源，最常用的是串口、RTC 一类设备。
                let base = u16::from_le_bytes([bytes[payload_start + 1], bytes[payload_start + 2]]) as usize;
                let length = bytes[payload_start + 6] as usize;
                parsed.push(ParsedResource {
                    kind: ResourceKind::IoPort,
                    irq: None,
                    base: Some(base),
                    length: Some(length),
                });
            }
            0x08 => {
                warn!("[bus][x86_64][acpi][aml] _CRS IO descriptor length({}) invalid", length);
            }
            0x09 if length >= 3 => {
                // 固定端口资源有时会直接写死基址，这里尽量兜底解析。
                let base = u16::from_le_bytes([bytes[payload_start], bytes[payload_start + 1]]) as usize;
                let length = bytes[payload_start + 2] as usize;
                parsed.push(ParsedResource {
                    kind: ResourceKind::IoPort,
                    irq: None,
                    base: Some(base),
                    length: Some(length),
                });
            }
            0x09 => {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS fixed IO descriptor length({}) invalid",
                    length
                );
            }
            _ => {
                warn!(
                    "[bus][x86_64][acpi][aml] _CRS small descriptor(type={:#x}) ignored in minimal parser",
                    descriptor_type
                );
            }
        }

        index = payload_end;
    }

    Some(parsed)
}

/// 函数说明：执行对应的总线处理步骤。
fn first_set_bit(mask: u16) -> Option<u16> {
    if mask == 0 {
        return None;
    }

    // IRQ mask 里第一个置位位号通常就是设备实际使用的中断线。
    Some(mask.trailing_zeros() as u16)
}
