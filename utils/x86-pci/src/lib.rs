#![no_std]

/// 解析 BDF 字符串（`ssss:bb:dd.f`）。
pub fn parse_bdf(bdf: &str) -> Option<(u16, u8, u8, u8)> {
    let (seg, rest) = bdf.split_once(':')?;
    let (bus, rest) = rest.split_once(':')?;
    let (dev, func) = rest.split_once('.')?;
    let segment = u16::from_str_radix(seg, 16).ok()?;
    let bus = u8::from_str_radix(bus, 16).ok()?;
    let device = u8::from_str_radix(dev, 16).ok()?;
    let function = func.parse::<u8>().ok()?;
    Some((segment, bus, device, function))
}

#[inline]
fn cfg_address(_segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    // 传统 CF8/CFC 机制不区分 segment，当前只覆盖 segment 0 的最小实现。
    (1u32 << 31)
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xfc)
}

/// 读取 32 位 PCI 配置空间。
#[inline]
pub fn cfg_read32(segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    unsafe {
        x86::io::outl(0xcf8, cfg_address(segment, bus, device, function, offset));
        x86::io::inl(0xcfc)
    }
}

/// 写入 32 位 PCI 配置空间。
#[inline]
pub fn cfg_write32(segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
    unsafe {
        x86::io::outl(0xcf8, cfg_address(segment, bus, device, function, offset));
        x86::io::outl(0xcfc, value);
    }
}

/// 读取 16 位 PCI 配置空间。
#[inline]
pub fn cfg_read16(segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
    let aligned = offset & !0x3;
    let value = cfg_read32(segment, bus, device, function, aligned);
    ((value >> ((offset & 0x2) * 8)) & 0xffff) as u16
}

/// 读取 8 位 PCI 配置空间。
#[inline]
pub fn cfg_read8(segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
    let aligned = offset & !0x3;
    let value = cfg_read32(segment, bus, device, function, aligned);
    ((value >> ((offset & 0x3) * 8)) & 0xff) as u8
}

/// 写入 16 位 PCI 配置空间（读改写）。
#[inline]
pub fn cfg_write16(segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
    let aligned = offset & !0x3;
    let mut cur = cfg_read32(segment, bus, device, function, aligned);
    let shift = (offset & 0x2) * 8;
    cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
    cfg_write32(segment, bus, device, function, aligned, cur);
}

/// 写入 8 位 PCI 配置空间（读改写）。
#[inline]
pub fn cfg_write8(segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
    let aligned = offset & !0x3;
    let mut cur = cfg_read32(segment, bus, device, function, aligned);
    let shift = (offset & 0x3) * 8;
    cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
    cfg_write32(segment, bus, device, function, aligned, cur);
}
