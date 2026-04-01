use acpi::PciAddress;
use x86_pci::{cfg_read32, cfg_write32};

#[inline]
pub(super) fn phys_to_virt(paddr: usize) -> usize {
    if paddr < config::LOW_PHYS_MAP_SIZE {
        return config::LOW_PHYS_MAP_BASE + paddr;
    }
    paddr + crate::common_x86_64::boot::PHYS_VIRT_OFFSET as usize
}

pub(super) fn pci_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    cfg_read32(
        address.segment(),
        address.bus(),
        address.device(),
        address.function(),
        offset,
    )
}

pub(super) fn pci_cfg_write32(address: PciAddress, offset: u16, value: u32) {
    cfg_write32(
        address.segment(),
        address.bus(),
        address.device(),
        address.function(),
        offset,
        value,
    )
}
