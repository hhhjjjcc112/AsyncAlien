pub mod config;

use core::ops::Range;
use heapless::Vec;
use spin::Once;

use crate::common_riscv::basic::MachineInfo as RiscvMachineInfo;
use crate::traits::{
    ConsoleIf, MachineInfo, MemIf, MiscIf, PowerIf, RawRange,
    TimeIf,
};

pub static BOOT_INFO: Once<usize> = Once::new();
static PHYS_RAM_RANGE: Once<RawRange> = Once::new();
static MAIN_ALLOC_RANGE: Once<RawRange> = Once::new();
static RESERVED_RANGES: Once<Vec<RawRange, 4>> = Once::new();

const MMIO_RANGES: &[RawRange] = &[
    (0x10_0000, 0x1000),
    (0x10_1000, 0x1000),
    (0x200_0000, 0x10000),
    (0xc00_0000, 0x600000),
    (0x1000_0000, 0x9000),
];

pub struct QemuRiscvPlatform;

unsafe extern "C" {
    fn sheap();
}

fn kernel_heap_end() -> usize {
    sheap as *const () as usize + ::config::KERNEL_HEAP_SIZE
}

fn init_platform_state(ptr: usize) {
    let info = crate::common_riscv::basic::machine_info(ptr);
    let alloc_start = kernel_heap_end();
    PHYS_RAM_RANGE.call_once(|| (info.memory.start, info.memory.end - info.memory.start));
    MAIN_ALLOC_RANGE.call_once(|| {
        (
            alloc_start,
            info.memory.end.saturating_sub(alloc_start),
        )
    });
    RESERVED_RANGES.call_once(|| {
        let mut ranges = Vec::new();
        if alloc_start > info.memory.start {
            let _ = ranges.push((info.memory.start, alloc_start - info.memory.start));
        }
        ranges
    });
}


impl ConsoleIf for QemuRiscvPlatform {
    fn putchar(ch: u8) {
        crate::common_riscv::sbi::console_putchar(ch);
    }

    fn getchar() -> Option<u8> {
        let ch = crate::common_riscv::sbi::console_getchar();
        if ch == '\0' || ch as u8 == 0xFF {
            None
        } else {
            Some(ch as u8)
        }
    }
}

impl MemIf for QemuRiscvPlatform {
    const PHYS_VIRT_OFFSET: usize = 0;

    fn phys_ram_ranges() -> &'static [RawRange] {
        core::slice::from_ref(PHYS_RAM_RANGE.get().expect("boot info not initialized"))
    }

    fn reserved_ranges() -> &'static [RawRange] {
        RESERVED_RANGES.get().map(|ranges| ranges.as_slice()).unwrap_or(&[])
    }

    fn mmio_ranges() -> &'static [RawRange] {
        MMIO_RANGES
    }

    fn alloc_ranges() -> &'static [RawRange] {
        core::slice::from_ref(MAIN_ALLOC_RANGE.get().expect("boot info not initialized"))
    }
}


impl PowerIf for QemuRiscvPlatform {
    fn shutdown() -> ! {
        crate::println!("shutdown...");
        crate::common_riscv::sbi::system_shutdown();
    }

    fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
        let _ = crate::common_riscv::sbi::hart_start(cpu_id, start_addr, opaque);
    }

    fn cpu_count() -> usize {
        ::config::CPU_NUM
    }

    fn current_cpu_id() -> usize {
        crate::current_cpu_id()
    }

    fn halt() {
        unsafe { core::arch::asm!("wfi") };
    }

    fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
        let _ = crate::common_riscv::sbi::remote_fence_i(cpu_mask, cpu_mask_base);
    }
}

impl TimeIf for QemuRiscvPlatform {
    fn current_ticks() -> u64 {
        riscv::register::time::read() as u64
    }

    fn tick_freq() -> u64 {
        config::CLOCK_FREQ as u64
    }

    fn epochoffset_nanos() -> u64 {
        0
    }

    fn set_timer(deadline: u64) {
        crate::common_riscv::sbi::set_timer(deadline as usize);
    }
}


impl MachineInfo for RiscvMachineInfo {
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


impl MiscIf for QemuRiscvPlatform {
    type MachineInfo = RiscvMachineInfo;

    fn init_boot_info(ptr: usize) {
        BOOT_INFO.call_once(|| ptr);
        init_platform_state(ptr);
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        crate::common_riscv::basic::machine_info(*BOOT_INFO.get().unwrap())
    }
}
