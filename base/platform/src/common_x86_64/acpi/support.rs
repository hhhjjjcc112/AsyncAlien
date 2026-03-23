use acpi::PciAddress;

#[inline]
pub(super) fn phys_to_virt(paddr: usize) -> usize {
    paddr + crate::common_x86_64::boot::PHYS_VIRT_OFFSET as usize
}

#[inline]
fn pci_cfg_address(address: PciAddress, offset: u16) -> u32 {
    let _segment = address.segment();
    (1u32 << 31)
        | ((address.bus() as u32) << 16)
        | ((address.device() as u32) << 11)
        | ((address.function() as u32) << 8)
        | ((offset as u32) & 0xfc)
}

pub(super) fn pci_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    unsafe {
        x86::io::outl(0xcf8, pci_cfg_address(address, offset));
        x86::io::inl(0xcfc)
    }
}

pub(super) fn pci_cfg_write32(address: PciAddress, offset: u16, value: u32) {
    unsafe {
        x86::io::outl(0xcf8, pci_cfg_address(address, offset));
        x86::io::outl(0xcfc, value);
    }
}