use core::{
    mem::{size_of, transmute},
    slice,
};

use aml::Handler as AmlHandler;
use x86_pci::{cfg_read16, cfg_read32, cfg_read8, cfg_write16, cfg_write32, cfg_write8};

use platform::MemIf;

#[derive(Clone, Copy)]
pub(super) struct AmlHost;

#[inline]
pub(super) fn phys_to_virt(address: usize) -> usize {
    if address < config::LOW_PHYS_MAP_SIZE {
        return config::LOW_PHYS_MAP_BASE + address;
    }
    // bus 侧和 platform 侧共享同一套线性映射。
    <platform::Platform as MemIf>::phys_to_virt(address)
}

impl AmlHost {
    pub(super) fn aml_table_bytes(table: acpi::AmlTable) -> Result<&'static [u8], aml::AmlError> {
        // AML 解释器只需要表体，不需要 SDT 头。
        let raw = unsafe {
            slice::from_raw_parts(phys_to_virt(table.phys_address) as *const u8, table.length as usize)
        };

        if raw.len() <= size_of::<acpi::sdt::SdtHeader>() {
            return Ok(&[]);
        }

        // 表映射在这里保持为全局线性映射，因此可以把切片视为长期有效。
        Ok(unsafe { transmute::<&[u8], &'static [u8]>(&raw[size_of::<acpi::sdt::SdtHeader>()..]) })
    }
}

impl AmlHandler for AmlHost {
    /// 函数说明：执行对应的总线处理步骤。
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (phys_to_virt(address) as *const u8).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (phys_to_virt(address) as *const u16).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (phys_to_virt(address) as *const u32).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (phys_to_virt(address) as *const u64).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u8(&mut self, address: usize, value: u8) {
        unsafe { (phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u16(&mut self, address: usize, value: u16) {
        unsafe { (phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u32(&mut self, address: usize, value: u32) {
        unsafe { (phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u64(&mut self, address: usize, value: u64) {
        unsafe { (phys_to_virt(address) as *mut u64).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86::io::inb(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86::io::inw(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86::io::inl(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { x86::io::outb(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { x86::io::outw(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { x86::io::outl(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        cfg_read8(segment, bus, device, function, offset)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        cfg_read16(segment, bus, device, function, offset)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        cfg_read32(segment, bus, device, function, offset)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        cfg_write8(segment, bus, device, function, offset, value)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        cfg_write16(segment, bus, device, function, offset, value)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        cfg_write32(segment, bus, device, function, offset, value);
    }
}
