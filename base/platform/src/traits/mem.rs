//! Physical memory information interface
//!
//! Provides memory layout information abstraction.

use core::ops::Range;

bitflags::bitflags! {
    /// Flags describing a physical memory region
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MemRegionFlags: usize {
        /// Readable
        const READ = 1 << 0;
        /// Writable
        const WRITE = 1 << 1;
        /// Executable
        const EXECUTE = 1 << 2;
        /// Device memory (MMIO)
        const DEVICE = 1 << 4;
        /// Uncacheable memory
        const UNCACHED = 1 << 5;
        /// Reserved (not for general allocation)
        const RESERVED = 1 << 6;
        /// Free for allocation
        const FREE = 1 << 7;
    }
}

impl Default for MemRegionFlags {
    fn default() -> Self {
        Self::READ | Self::WRITE | Self::FREE
    }
}

/// Raw memory range: (start_address, size)
pub type RawRange = (usize, usize);

/// A physical memory region descriptor
#[derive(Debug, Clone, Copy)]
pub struct PhysMemRegion {
    /// Start physical address
    pub paddr: usize,
    /// Size in bytes
    pub size: usize,
    /// Region flags
    pub flags: MemRegionFlags,
    /// Region name (for debugging)
    pub name: &'static str,
}

impl PhysMemRegion {
    /// Create a new RAM region (readable, writable, allocatable)
    pub const fn new_ram(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::FREE),
            name,
        }
    }

    /// Create a new MMIO region (readable, writable, device)
    pub const fn new_mmio(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::DEVICE),
            name,
        }
    }

    /// Create a new reserved region (readable, writable, not allocatable)
    pub const fn new_reserved(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::RESERVED),
            name,
        }
    }

    /// Get the address range
    pub const fn range(&self) -> Range<usize> {
        self.paddr..(self.paddr + self.size)
    }
}

/// Physical memory information trait
///
/// Platform implementations provide memory layout information.
pub trait MemIf {
    /// Physical-to-virtual offset for direct mapping
    const PHYS_VIRT_OFFSET: usize;

    /// Returns all physical RAM ranges
    fn phys_ram_ranges() -> &'static [RawRange];

    /// Returns reserved physical memory ranges (kernel, DTB, etc.)
    fn reserved_ranges() -> &'static [RawRange];

    /// Returns MMIO (device memory) ranges
    fn mmio_ranges() -> &'static [RawRange];

    /// Translate physical address to virtual address
    fn phys_to_virt(paddr: usize) -> usize {
        paddr.wrapping_add(Self::PHYS_VIRT_OFFSET)
    }

    /// Translate virtual address to physical address
    fn virt_to_phys(vaddr: usize) -> usize {
        vaddr.wrapping_sub(Self::PHYS_VIRT_OFFSET)
    }

    /// Get total RAM size
    fn total_ram_size() -> usize {
        Self::phys_ram_ranges().iter().map(|(_, size)| *size).sum()
    }
}
