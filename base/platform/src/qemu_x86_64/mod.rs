//! QEMU x86_64 平台支持。

pub mod config;

use core::ops::Range;
use heapless::Vec;
use spin::Once;

use crate::common_x86_64::basic::MachineInfo as X86MachineInfo;
use crate::common_x86_64::time;
use crate::traits::{
    ConsoleIf, MachineInfo, MemIf, MiscIf, PowerIf, RawRange,
    TimeIf,
};

pub static BOOT_INFO: Once<usize> = Once::new();
static MAIN_ALLOC_RANGE: Once<RawRange> = Once::new();
static ALLOC_RANGES: Once<Vec<RawRange, 16>> = Once::new();

/// QEMU x86_64 平台类型。
pub struct QemuX86Platform;

unsafe extern "C" {
    fn sheap();
}

fn kernel_heap_end_paddr() -> usize {
    let heap_start = sheap as *const () as usize;
    heap_start
        .wrapping_sub(crate::common_x86_64::boot::PHYS_VIRT_OFFSET as usize)
        .saturating_add(::config::KERNEL_HEAP_SIZE)
}

fn build_alloc_ranges(kernel_heap_end: usize) -> Vec<RawRange, 16> {
    let mut excludes = Vec::<RawRange, 16>::new();
    let _ = excludes.push((0, kernel_heap_end));
    for &(start, size) in crate::common_x86_64::mem::RESERVED_REGIONS {
        let _ = excludes.push((start, size));
    }
    excludes.sort_unstable_by_key(|&(start, _)| start);

    let mut out = Vec::<RawRange, 16>::new();
    for &(ram_start, ram_size) in crate::common_x86_64::mem::phys_ram_ranges() {
        if ram_size == 0 {
            continue;
        }
        let ram_end = ram_start.saturating_add(ram_size);
        let mut cur = ram_start;
        for &(ex_start, ex_size) in excludes.iter() {
            let ex_end = ex_start.saturating_add(ex_size);
            if ex_end <= cur {
                continue;
            }
            if ex_start >= ram_end {
                break;
            }
            if ex_start > cur {
                let _ = out.push((cur, ex_start - cur));
            }
            if ex_end > cur {
                cur = ex_end;
            }
            if cur >= ram_end {
                break;
            }
        }
        if cur < ram_end {
            let _ = out.push((cur, ram_end - cur));
        }
    }
    out
}

fn pick_main_alloc_range(ranges: &[RawRange], fallback_start: usize) -> RawRange {
    let mut fallback = (fallback_start, 0);

    for &(start, size) in ranges {
        if size > fallback.1 {
            fallback = (start, size);
        }
    }

    fallback
}

fn init_platform_state(ptr: usize) {
    crate::common_x86_64::mem::init_from_multiboot(ptr);
    let kernel_heap_end = kernel_heap_end_paddr();
    ALLOC_RANGES.call_once(|| build_alloc_ranges(kernel_heap_end));
    MAIN_ALLOC_RANGE.call_once(|| {
        let ranges = ALLOC_RANGES.get().expect("boot info not initialized");
        pick_main_alloc_range(ranges.as_slice(), kernel_heap_end)
    });
}

impl ConsoleIf for QemuX86Platform {
    fn putchar(ch: u8) {
        crate::common_x86_64::services::console_putchar(ch);
    }

    fn getchar() -> Option<u8> {
        crate::common_x86_64::services::console_getchar()
    }
}

impl MemIf for QemuX86Platform {
    const PHYS_VIRT_OFFSET: usize = crate::common_x86_64::boot::PHYS_VIRT_OFFSET as usize;

    fn phys_ram_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::phys_ram_ranges()
    }

    fn reserved_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::RESERVED_REGIONS
    }

    fn mmio_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::mmio_ranges()
    }

    fn alloc_ranges() -> &'static [RawRange] {
        ALLOC_RANGES
            .get()
            .map(|ranges| ranges.as_slice())
            .unwrap_or(&[])
    }
}

impl PowerIf for QemuX86Platform {
    fn shutdown() -> ! {
        crate::common_x86_64::services::system_shutdown()
    }

    fn start_secondary_cpu(cpu_id: usize, _start_addr: usize, _opaque: usize) {
        crate::common_x86_64::ap::boot_secondary_cpu(cpu_id);
    }

    fn cpu_count() -> usize {
        ::config::CPU_NUM
    }

    fn current_cpu_id() -> usize {
        crate::current_cpu_id()
    }

    fn halt() {
        x86_64::instructions::hlt();
    }

    fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
        crate::common_x86_64::services::flush_cache(cpu_mask, cpu_mask_base)
    }
}

impl TimeIf for QemuX86Platform {
    fn current_ticks() -> u64 {
        time::current_ticks()
    }

    fn tick_freq() -> u64 {
        let freq = time::tsc_frequency();
        if freq == 0 {
            crate::qemu_x86_64::config::CLOCK_FREQ as u64
        } else {
            freq
        }
    }

    fn epochoffset_nanos() -> u64 {
        time::get_rtc_epoch_seconds().saturating_mul(1_000_000_000)
    }

    fn set_timer(deadline: u64) {
        crate::common_x86_64::services::set_timer(deadline as usize);
    }
}


impl MachineInfo for X86MachineInfo {
    fn memory_start(&self) -> usize {
        self.memory.start
    }

    fn memory_size(&self) -> usize {
        self.memory.end - self.memory.start
    }

    fn cpu_count(&self) -> usize {
        self.smp
    }

    fn initrd(&self) -> Option<Range<usize>> {
        self.initrd.clone()
    }

    fn bootargs(&self) -> Option<&str> {
        self.bootargs.as_ref().and_then(|args| {
            core::str::from_utf8(&args[..self.bootargs_len]).ok()
        })
    }
}


impl MiscIf for QemuX86Platform {
    type MachineInfo = X86MachineInfo;

    fn init_boot_info(ptr: usize) {
        BOOT_INFO.call_once(|| ptr);
        init_platform_state(ptr);
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        crate::common_x86_64::basic::machine_info_from_boot_info(*BOOT_INFO.get().unwrap_or(&0))
    }
}

#[allow(dead_code)]
pub fn init_boot_info(boot_info: usize) {
    <QemuX86Platform as MiscIf>::init_boot_info(boot_info);
}

#[deprecated(note = "use init_boot_info")]
#[allow(dead_code)]
pub fn init_dtb(boot_info: Option<usize>) {
    init_boot_info(boot_info.unwrap_or(0));
}

#[allow(dead_code)]
pub fn boot_info_ptr() -> usize {
    <QemuX86Platform as MiscIf>::boot_info_ptr()
}

#[allow(dead_code)]
pub fn basic_machine_info() -> X86MachineInfo {
    <QemuX86Platform as MiscIf>::machine_info()
}

#[allow(dead_code)]
pub fn set_timer(time: usize) {
    crate::common_x86_64::services::set_timer(time);
}

#[allow(dead_code)]
pub fn system_shutdown() -> ! {
    <QemuX86Platform as PowerIf>::shutdown()
}

#[allow(dead_code)]
pub fn console_putchar(ch: u8) {
    <QemuX86Platform as ConsoleIf>::putchar(ch);
}

#[allow(dead_code)]
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
    <QemuX86Platform as PowerIf>::start_secondary_cpu(cpu_id, start_addr, opaque)
}

#[allow(dead_code)]
pub fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
    crate::common_x86_64::services::flush_cache(cpu_mask, cpu_mask_base)
}

