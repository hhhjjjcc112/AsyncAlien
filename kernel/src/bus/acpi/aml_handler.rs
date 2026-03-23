use core::{mem::{size_of, transmute}, slice};

use acpi::{Handle, Handler as AcpiHandler, PhysicalMapping, PciAddress};
use aml::Handler as AmlHandler;

use platform::MemIf;

pub(super) struct AmlHost;

impl AmlHost {
    fn phys_to_virt(address: usize) -> usize {
        // bus 侧和 platform 侧共享同一套线性映射，因此这里直接转换物理地址。
        <platform::Platform as MemIf>::phys_to_virt(address)
    }

    pub(super) fn aml_table_bytes(table: acpi::AmlTable) -> Result<&'static [u8], aml::AmlError> {
        // AML 解释器只需要表体，不需要 SDT 头；这里直接切出 AML 数据区。
        let raw = unsafe {
            slice::from_raw_parts(
                Self::phys_to_virt(table.phys_address) as *const u8,
                table.length as usize,
            )
        };

        if raw.len() <= size_of::<acpi::sdt::SdtHeader>() {
            return Ok(&[]);
        }

        // 表映射在这里保持为全局线性映射，因此可以把切片视为长期有效。
        Ok(unsafe { transmute::<&[u8], &'static [u8]>(&raw[size_of::<acpi::sdt::SdtHeader>()..]) })
    }
}

impl AmlHandler for AmlHost {
    fn read_u8(&self, address: usize) -> u8 {
        // AML 解释器读取内存 / MMIO 时，直接走全局线性映射。
        unsafe { (Self::phys_to_virt(address) as *const u8).read_volatile() }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (Self::phys_to_virt(address) as *const u16).read_volatile() }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (Self::phys_to_virt(address) as *const u32).read_volatile() }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (Self::phys_to_virt(address) as *const u64).read_volatile() }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        // AML 写回同样直接落到映射后的物理地址。
        unsafe { (Self::phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        unsafe { (Self::phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        unsafe { (Self::phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        unsafe { (Self::phys_to_virt(address) as *mut u64).write_volatile(value) }
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86::io::inb(port) }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86::io::inw(port) }
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86::io::inl(port) }
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { x86::io::outb(port, value) }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { x86::io::outw(port, value) }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { x86::io::outl(port, value) }
    }

    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        // AML 里访问 PCI 配置空间时，走传统 0xCF8/0xCFC 端口机制。
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x3) * 8)) & 0xff) as u8
    }

    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x2) * 8)) & 0xffff) as u16
    }

    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_read32(addr, offset)
    }

    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x3) * 8;
        cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x2) * 8;
        cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_write32(addr, offset, value);
    }
}

fn pci_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    // x86 传统 PCI 配置访问：先写 CF8，再从 CFC 读回 32 位数据。
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

fn pci_cfg_write32(address: PciAddress, offset: u16, value: u32) {
    // 写配置空间时也先定位到对应 BDF，再把数据写回 CFC。
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
