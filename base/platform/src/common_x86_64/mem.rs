//! Physical memory information for x86-64
//!
//! Parses memory regions from Multiboot information.

use core::ops::Range;

use heapless::Vec;
use lazyinit::LazyInit;
use multiboot::information::{MemoryManagement, MemoryType, Multiboot, PAddr};

use super::boot::PHYS_VIRT_OFFSET;

const MAX_REGIONS: usize = 16;

/// Physical memory (RAM) regions discovered from Multiboot
static RAM_REGIONS: LazyInit<Vec<(usize, usize), MAX_REGIONS>> = LazyInit::new();

/// Memory management helper for Multiboot parsing
struct MemHelper;

impl MemoryManagement for MemHelper {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let vaddr = addr as usize + PHYS_VIRT_OFFSET as usize;
        Some(unsafe { core::slice::from_raw_parts(vaddr as *const u8, size) })
    }

    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, _addr: PAddr) {}
}

/// Initialize memory regions from Multiboot information
pub fn init_from_multiboot(multiboot_info_ptr: usize) {
    let mut mm = MemHelper;
    
    if let Some(info) = unsafe { Multiboot::from_ptr(multiboot_info_ptr as PAddr, &mut mm) } {
        let mut regions: Vec<(usize, usize), MAX_REGIONS> = Vec::new();
        
        if let Some(mem_regions) = info.memory_regions() {
            for r in mem_regions {
                if r.memory_type() == MemoryType::Available {
                    let base = r.base_address() as usize;
                    let size = r.length() as usize;
                    let _ = regions.push((base, size));
                    log::info!("RAM region: {:#x}..{:#x}", base, base + size);
                }
            }
        }
        
        if regions.is_empty() {
            // Fallback to default memory configuration
            let _ = regions.push((0x100000, 0x800_0000 - 0x100000)); // 1MB to 128MB
            log::warn!("No memory regions from Multiboot, using default");
        }
        
        RAM_REGIONS.init_once(regions);
    } else {
        // No Multiboot info, use default
        let mut regions: Vec<(usize, usize), MAX_REGIONS> = Vec::new();
        let _ = regions.push((0x100000, 0x800_0000 - 0x100000));
        RAM_REGIONS.init_once(regions);
        log::warn!("Invalid Multiboot info, using default memory");
    }
}

/// Get all physical memory (RAM) ranges
pub fn phys_ram_ranges() -> &'static [(usize, usize)] {
    RAM_REGIONS.as_slice()
}

/// Get total available RAM size
pub fn total_ram_size() -> usize {
    RAM_REGIONS
        .iter()
        .map(|(_, size)| *size)
        .sum()
}

/// Get memory end address (highest RAM address)
pub fn memory_end() -> usize {
    RAM_REGIONS
        .iter()
        .map(|(base, size)| base + size)
        .max()
        .unwrap_or(0x800_0000)
}

/// Get memory range for compatibility
pub fn memory_range() -> Range<usize> {
    let start = RAM_REGIONS
        .iter()
        .map(|(base, _)| *base)
        .min()
        .unwrap_or(0);
    let end = memory_end();
    start..end
}

/// Reserved memory regions (lower 1MiB)
pub const RESERVED_REGIONS: &[(usize, usize)] = &[
    (0, 0x100000),
];

/// MMIO ranges for device memory
pub fn mmio_ranges() -> &'static [(usize, usize)] {
    crate::qemu_x86_64::config::MMIO_RANGES
}

/// Translate physical address to virtual address
#[inline]
pub fn phys_to_virt(paddr: usize) -> usize {
    paddr + PHYS_VIRT_OFFSET as usize
}

/// Translate virtual address to physical address
#[inline]
pub fn virt_to_phys(vaddr: usize) -> usize {
    vaddr - PHYS_VIRT_OFFSET as usize
}
