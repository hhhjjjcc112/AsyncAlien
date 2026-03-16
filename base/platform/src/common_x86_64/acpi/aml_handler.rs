use acpi::PciAddress;
use aml::Handler as AmlHandler;

use super::support::{pci_cfg_read32, pci_cfg_write32, phys_to_virt};

pub(super) struct AmlHost;

impl AmlHandler for AmlHost {
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (phys_to_virt(address) as *const u8).read_volatile() }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (phys_to_virt(address) as *const u16).read_volatile() }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (phys_to_virt(address) as *const u32).read_volatile() }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (phys_to_virt(address) as *const u64).read_volatile() }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        unsafe { (phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        unsafe { (phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        unsafe { (phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        unsafe { (phys_to_virt(address) as *mut u64).write_volatile(value) }
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