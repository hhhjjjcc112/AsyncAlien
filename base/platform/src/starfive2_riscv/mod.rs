pub mod config;
use core::ops::Range;

use heapless::Vec;
use spin::Once;

use crate::common_riscv::basic::MachineInfo as RiscvMachineInfo;
use crate::traits::{
    ConsoleIf, IpiTarget, IrqIf, MachineInfo, MemIf, MiscIf, PowerIf, RawRange,
    TimeIf,
};

pub const FDT: &[u8] = include_bytes!("../../../../tools/jh7110-visionfive-v2.dtb");

pub static BOOT_INFO: Once<usize> = Once::new();
#[deprecated(note = "use BOOT_INFO")]
pub use BOOT_INFO as DTB;
static PHYS_RAM_RANGE: Once<RawRange> = Once::new();
static RESERVED_RANGES: Once<Vec<RawRange, 4>> = Once::new();
static PLIC_BASE: Once<usize> = Once::new();

const MMIO_RANGES: &[RawRange] = &[
    (0x1704_0000, 0x10000),
    (0x0c00_0000, 0x4000000),
    (0x1000_0000, 0x10000),
    (0x1602_0000, 0x10000),
];

static INITRD: &'static [u8] = include_bytes!("../../../../build/initramfs.cpio.gz");

/// VisionFive2 RISC-V 平台类型。
pub struct Vf2Platform;

fn init_platform_state(ptr: usize) {
    let info = crate::common_riscv::basic::machine_info_from_boot_info(ptr);
    PHYS_RAM_RANGE.call_once(|| (info.memory.start, info.memory.end - info.memory.start));
    PLIC_BASE.call_once(|| info.plic.start);
    RESERVED_RANGES.call_once(|| {
        let mut ranges = Vec::new();
        let _ = ranges.push((ptr, FDT.len()));
        let _ = ranges.push((INITRD.as_ptr() as usize, INITRD.len()));
        ranges
    });
}

fn plic_context(cpu_id: usize) -> usize {
    cpu_id * 2 + 1
}

fn plic_claim_addr(base: usize, cpu_id: usize) -> *mut u32 {
    (base + 0x20_0004 + plic_context(cpu_id) * 0x1000) as *mut u32
}

fn plic_threshold_addr(base: usize, cpu_id: usize) -> *mut u32 {
    (base + 0x20_0000 + plic_context(cpu_id) * 0x1000) as *mut u32
}

fn plic_enable_addr(base: usize, cpu_id: usize, irq: usize) -> *mut u32 {
    let word = (irq / 32) * 4;
    (base + 0x2000 + plic_context(cpu_id) * 0x80 + word) as *mut u32
}

// ============================================================================
// ConsoleIf 实现
// ============================================================================
impl ConsoleIf for Vf2Platform {
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

impl IrqIf for Vf2Platform {
    const MAX_IRQ_NUM: usize = 1024;

    fn set_enable(irq: usize, enabled: bool) {
        let Some(&base) = PLIC_BASE.get() else {
            return;
        };
        let cpu_id = <Self as PowerIf>::current_cpu_id();
        let reg = plic_enable_addr(base, cpu_id, irq);
        let mask = 1u32 << (irq % 32);
        unsafe {
            let value = reg.read_volatile();
            reg.write_volatile(if enabled { value | mask } else { value & !mask });
            ((base + irq * 4) as *mut u32).write_volatile(if enabled { 1 } else { 0 });
        }
    }

    fn current_irq() -> Option<usize> {
        let Some(&base) = PLIC_BASE.get() else {
            return None;
        };
        let cpu_id = <Self as PowerIf>::current_cpu_id();
        let irq = unsafe { plic_claim_addr(base, cpu_id).read_volatile() as usize };
        if irq == 0 { None } else { Some(irq) }
    }

    fn ack_irq(irq: usize) {
        let Some(&base) = PLIC_BASE.get() else {
            return;
        };
        let cpu_id = <Self as PowerIf>::current_cpu_id();
        unsafe {
            plic_claim_addr(base, cpu_id).write_volatile(irq as u32);
        }
    }

    fn send_ipi(target: IpiTarget) {
        match target {
            IpiTarget::Unicast { cpu_id } => {
                let _ = crate::common_riscv::sbi::send_ipi(1usize << cpu_id, 0);
            }
            IpiTarget::Broadcast { exclude_self } => {
                let self_id = <Self as PowerIf>::current_cpu_id();
                let mut mask = 0usize;
                for cpu_id in 0..<Self as PowerIf>::cpu_count() {
                    if exclude_self && cpu_id == self_id {
                        continue;
                    }
                    mask |= 1usize << cpu_id;
                }
                let _ = crate::common_riscv::sbi::send_ipi(mask, 0);
            }
            IpiTarget::Multicast { mask, mask_base } => {
                let _ = crate::common_riscv::sbi::send_ipi(mask, mask_base);
            }
        }
    }

    fn init_primary() {
        let Some(&base) = PLIC_BASE.get() else {
            return;
        };
        let cpu_id = <Self as PowerIf>::current_cpu_id();
        unsafe {
            plic_threshold_addr(base, cpu_id).write_volatile(0);
        }
    }

    fn init_secondary(cpu_id: usize) {
        let Some(&base) = PLIC_BASE.get() else {
            return;
        };
        unsafe {
            plic_threshold_addr(base, cpu_id).write_volatile(0);
        }
    }
}

impl MemIf for Vf2Platform {
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
}

// ============================================================================
// PowerIf 实现
// ============================================================================
impl PowerIf for Vf2Platform {
    fn shutdown() -> ! {
        crate::common_riscv::sbi::system_shutdown();
    }

    fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
        let _ = crate::common_riscv::sbi::hart_start(cpu_id, start_addr, opaque);
    }

    fn cpu_count() -> usize {
        ::config::CPU_NUM
    }

    fn current_cpu_id() -> usize {
        arch::cpu_id()
    }

    fn halt() {
        unsafe { core::arch::asm!("wfi") };
    }

    fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
        let _ = crate::common_riscv::sbi::remote_fence_i(cpu_mask, cpu_mask_base);
    }
}

impl TimeIf for Vf2Platform {
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

// ============================================================================
// MiscIf 实现
// ============================================================================
impl MiscIf for Vf2Platform {
    type MachineInfo = RiscvMachineInfo;

    fn init_boot_info(_ptr: usize) {
        // VF2 使用内置 DTB。
        let dtb_ptr = FDT.as_ptr() as usize;
        BOOT_INFO.call_once(|| dtb_ptr);
        init_platform_state(dtb_ptr);
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        let mut info =
            crate::common_riscv::basic::machine_info_from_boot_info(*BOOT_INFO.get().unwrap());
        info.initrd = Some(Range {
            start: INITRD.as_ptr() as usize,
            end: INITRD.as_ptr() as usize + INITRD.len(),
        });
        info
    }
}

