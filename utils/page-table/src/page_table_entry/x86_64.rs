//! x86-64 page table entries.
//!
//! x86-64 uses 4-level paging (PML4 → PDPT → PD → PT) with 4KB pages.
//! Large pages: 2MB (using PD entry) and 1GB (using PDPT entry).

use core::fmt;

use memory_addr::PhysAddr;

use super::{GenericPTE, MappingFlags};

bitflags::bitflags! {
    /// Page-table entry flags for x86-64.
    pub struct PTEFlags: u64 {
        /// Whether the PTE is present/valid.
        const P         = 1 << 0;
        /// Whether the page is writable.
        const RW        = 1 << 1;
        /// Whether the page is accessible from user mode.
        const US        = 1 << 2;
        /// Page-level write-through.
        const PWT       = 1 << 3;
        /// Page-level cache disable.
        const PCD       = 1 << 4;
        /// Whether the page has been accessed.
        const A         = 1 << 5;
        /// Whether the page has been written (dirty).
        const D         = 1 << 6;
        /// Page size: if set in PD/PDPT, maps a large page (2MB/1GB).
        const PS        = 1 << 7;
        /// Global page (not flushed from TLB on CR3 write).
        const G         = 1 << 8;
        /// No execute bit (requires NX/XD support).
        const NX        = 1 << 63;
    }
}

impl From<PTEFlags> for MappingFlags {
    fn from(f: PTEFlags) -> Self {
        let mut ret = Self::empty();
        // x86 pages are always readable if present
        if f.contains(PTEFlags::P) {
            ret |= Self::READ;
        }
        if f.contains(PTEFlags::RW) {
            ret |= Self::WRITE;
        }
        // NX=0 means executable
        if !f.contains(PTEFlags::NX) && f.contains(PTEFlags::P) {
            ret |= Self::EXECUTE;
        }
        if f.contains(PTEFlags::US) {
            ret |= Self::USER;
        }
        if f.contains(PTEFlags::PCD) {
            ret |= Self::UNCACHED;
        }
        ret
    }
}

impl From<MappingFlags> for PTEFlags {
    fn from(f: MappingFlags) -> Self {
        if f.is_empty() {
            return Self::empty();
        }
        // x86 pages are always readable, so we set P
        let mut ret = Self::P | Self::A | Self::D;
        if f.contains(MappingFlags::WRITE) {
            ret |= Self::RW;
        }
        // If not executable, set NX
        if !f.contains(MappingFlags::EXECUTE) {
            ret |= Self::NX;
        }
        if f.contains(MappingFlags::USER) {
            ret |= Self::US;
        }
        if f.contains(MappingFlags::UNCACHED) || f.contains(MappingFlags::DEVICE) {
            ret |= Self::PCD;
        }
        ret
    }
}

/// x86-64 page table entry (PML4E, PDPTE, PDE, PTE).
///
/// The format is:
/// - Bits 0-11: Flags
/// - Bits 12-51: Physical page number (40 bits, giving 52-bit physical addresses)
/// - Bits 52-62: Available for software
/// - Bit 63: NX (No Execute)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct X64PTE(u64);

impl X64PTE {
    /// Physical address mask: bits 12..52 (40-bit PPN)
    const PHYS_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;
    
    /// Flags mask (bits that are flags, not address)
    const FLAGS_MASK: u64 = 0xFFF0_0000_0000_0FFF;
}

impl GenericPTE for X64PTE {
    fn new_page(paddr: PhysAddr, flags: MappingFlags, is_huge: bool) -> Self {
        let mut pte_flags = PTEFlags::from(flags);
        if is_huge {
            pte_flags |= PTEFlags::PS;
        }
        Self(pte_flags.bits() | (paddr.as_usize() as u64 & Self::PHYS_ADDR_MASK))
    }
    
    fn new_table(paddr: PhysAddr) -> Self {
        // Table entries need P, RW, US to allow access at all levels
        // A and D are set for convenience
        let flags = PTEFlags::P | PTEFlags::RW | PTEFlags::US | PTEFlags::A;
        Self(flags.bits() | (paddr.as_usize() as u64 & Self::PHYS_ADDR_MASK))
    }
    
    fn paddr(&self) -> PhysAddr {
        PhysAddr::from((self.0 & Self::PHYS_ADDR_MASK) as usize)
    }
    
    fn flags(&self) -> MappingFlags {
        PTEFlags::from_bits_truncate(self.0).into()
    }
    
    fn set_paddr(&mut self, paddr: PhysAddr) {
        self.0 = (self.0 & Self::FLAGS_MASK) | (paddr.as_usize() as u64 & Self::PHYS_ADDR_MASK);
    }
    
    fn set_flags(&mut self, flags: MappingFlags, is_huge: bool) {
        let mut pte_flags = PTEFlags::from(flags);
        if is_huge {
            pte_flags |= PTEFlags::PS;
        }
        // Preserve the physical address, update flags
        let paddr_bits = self.0 & Self::PHYS_ADDR_MASK;
        self.0 = pte_flags.bits() | paddr_bits;
    }

    fn is_unused(&self) -> bool {
        self.0 == 0
    }
    
    fn is_present(&self) -> bool {
        PTEFlags::from_bits_truncate(self.0).contains(PTEFlags::P)
    }
    
    fn is_huge(&self) -> bool {
        PTEFlags::from_bits_truncate(self.0).contains(PTEFlags::PS)
    }
    
    fn clear(&mut self) {
        self.0 = 0;
    }
}

impl fmt::Debug for X64PTE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("X64PTE");
        f.field("raw", &format_args!("{:#x}", self.0))
            .field("paddr", &self.paddr())
            .field("flags", &self.flags())
            .field("present", &self.is_present())
            .field("huge", &self.is_huge())
            .finish()
    }
}
