//! x86_64 早期内存信息。

use core::ops::Range;

use heapless::Vec;
use multiboot::information::{MemoryManagement, MemoryType, Multiboot, PAddr};
use spin::Once;

use crate::traits::RawRange;

use super::boot::PHYS_VIRT_OFFSET;

const FALLBACK_RAM_START: usize = 0x0010_0000; // 1 MiB
const FALLBACK_RAM_SIZE: usize = 0x0800_0000; // 128 MiB
const MAX_RAM_REGIONS: usize = 16;

static PHYS_RAM_RANGES: Once<Vec<RawRange, MAX_RAM_REGIONS>> = Once::new();

/// 预留区间（低端传统设备区）。
pub const RESERVED_REGIONS: &[RawRange] = crate::qemu_x86_64::config::RESERVED_MEMORY;

struct BootInfoMemHelper;

impl MemoryManagement for BootInfoMemHelper {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let vaddr = addr as usize + PHYS_VIRT_OFFSET as usize;
        Some(unsafe { core::slice::from_raw_parts(vaddr as *const u8, size) })
    }

    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, _addr: PAddr) {}
}

fn fallback_range() -> RawRange {
    (FALLBACK_RAM_START, FALLBACK_RAM_SIZE)
}

fn parse_ram_from_multiboot(multiboot_ptr: usize) -> Vec<RawRange, MAX_RAM_REGIONS> {
    let mut regions = Vec::new();
    let mut mm = BootInfoMemHelper;
    let Some(info) = (unsafe { Multiboot::from_ptr(multiboot_ptr as PAddr, &mut mm) }) else {
        let _ = regions.push(fallback_range());
        return regions;
    };

    // 参考 axplat-x86-pc：收集全部 Available 区间。
    if let Some(entries) = info.memory_regions() {
        for entry in entries {
            if entry.memory_type() != MemoryType::Available {
                continue;
            }
            let start = entry.base_address() as usize;
            let size = entry.length() as usize;
            if size == 0 {
                continue;
            }
            let end = start.saturating_add(size);
            let clipped_start = start.max(FALLBACK_RAM_START);
            if clipped_start >= end {
                continue;
            }
            let clipped_size = end - clipped_start;
            let _ = regions.push((clipped_start, clipped_size));
        }
        if !regions.is_empty() {
            return regions;
        }
    }

    // 无 e820 时回退到 multiboot 上界字段。
    if let Some(upper_kib) = info.upper_memory_bound() {
        let size = (upper_kib as usize) * 1024;
        if size != 0 {
            let _ = regions.push((FALLBACK_RAM_START, size));
            return regions;
        }
    }

    let _ = regions.push(fallback_range());
    regions
}

/// 从 multiboot 信息初始化 RAM 区间。
pub fn init_from_multiboot(multiboot_ptr: usize) {
    PHYS_RAM_RANGES.call_once(|| parse_ram_from_multiboot(multiboot_ptr));
}

/// 返回主 RAM 区间（start..end）。
pub fn memory_range() -> Range<usize> {
    if let Some(ranges) = PHYS_RAM_RANGES.get() {
        let mut min_start = usize::MAX;
        let mut max_end = 0usize;
        for (start, size) in ranges.iter().copied() {
            min_start = min_start.min(start);
            max_end = max_end.max(start.saturating_add(size));
        }
        if min_start != usize::MAX && max_end > min_start {
            return min_start..max_end;
        }
    }
    FALLBACK_RAM_START..FALLBACK_RAM_START.saturating_add(FALLBACK_RAM_SIZE)
}

/// 返回可用 RAM 原始区间列表。
pub fn phys_ram_ranges() -> &'static [RawRange] {
    static FALLBACK: RawRange = (FALLBACK_RAM_START, FALLBACK_RAM_SIZE);
    PHYS_RAM_RANGES
        .get()
        .map(|ranges| ranges.as_slice())
        .unwrap_or(core::slice::from_ref(&FALLBACK))
}

/// 返回 MMIO 区间列表。
pub fn mmio_ranges() -> &'static [RawRange] {
    crate::qemu_x86_64::config::MMIO_RANGES
}
