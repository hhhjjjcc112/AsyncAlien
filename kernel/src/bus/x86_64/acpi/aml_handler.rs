use core::{
    mem::{size_of, transmute},
    slice,
};

use acpi::PciAddress;
use aml::Handler as AmlHandler;

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
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x3) * 8)) & 0xff) as u8
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x2) * 8)) & 0xffff) as u16
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_read32(addr, offset)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x3) * 8;
        cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x2) * 8;
        cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_write32(addr, offset, value);
    }
}

#[inline]
pub(super) fn pci_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    let config_address = 0x8000_0000
        | ((address.bus() as u32) << 16)
        | ((address.device() as u32) << 11)
        | ((address.function() as u32) << 8)
        | ((offset as u32) & 0xfc);
    unsafe {
        x86::io::outl(0xcf8, config_address);
        x86::io::inl(0xcfc)
    }
}

#[inline]
pub(super) fn pci_cfg_write32(address: PciAddress, offset: u16, value: u32) {
    let config_address = 0x8000_0000
        | ((address.bus() as u32) << 16)
        | ((address.device() as u32) << 11)
        | ((address.function() as u32) << 8)
        | ((offset as u32) & 0xfc);
    unsafe {
        x86::io::outl(0xcf8, config_address);
        x86::io::outl(0xcfc, value);
    }
}
