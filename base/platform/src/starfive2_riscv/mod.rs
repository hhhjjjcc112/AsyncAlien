pub mod config;
use core::ops::Range;

use spin::Once;

use crate::common_riscv::basic::MachineInfo as RiscvMachineInfo;
use crate::traits::{ConsoleIf, MachineInfo, MiscIf, PowerIf, PlatformCallRet};

pub const FDT: &[u8] = include_bytes!("../../../../tools/jh7110-visionfive-v2.dtb");

pub static DTB: Once<usize> = Once::new();

static INITRD: &'static [u8] = include_bytes!("../../../../build/initramfs.cpio.gz");

/// VisionFive2 RISC-V platform type
pub struct Vf2Platform;

// ============================================================================
// ConsoleIf implementation
// ============================================================================
impl ConsoleIf for Vf2Platform {
    fn putchar(ch: u8) {
        crate::common_riscv::sbi::console_putchar(ch);
    }

    fn getchar() -> Option<u8> {
        let ch = crate::common_riscv::sbi::console_getchar();
        if ch == '\0' || ch == '\xff' {
            None
        } else {
            Some(ch as u8)
        }
    }
}

// ============================================================================
// PowerIf implementation
// ============================================================================
impl PowerIf for Vf2Platform {
    fn system_off() -> ! {
        crate::common_riscv::sbi::system_shutdown();
    }

    fn cpu_boot(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformCallRet {
        let ret = crate::common_riscv::sbi::hart_start(cpu_id, start_addr, opaque);
        PlatformCallRet {
            error: ret.error,
            value: ret.value,
        }
    }

    fn cpu_num() -> usize {
        ::config::CPU_NUM
    }

    fn current_cpu_id() -> usize {
        arch::cpu_id()
    }

    fn cpu_halt() {
        unsafe { core::arch::asm!("wfi") };
    }

    fn remote_fence_i(cpu_mask: usize, cpu_mask_base: usize) -> PlatformCallRet {
        let ret = crate::common_riscv::sbi::remote_fence_i(cpu_mask, cpu_mask_base);
        PlatformCallRet {
            error: ret.error,
            value: ret.value,
        }
    }
}

// ============================================================================
// MiscIf implementation
// ============================================================================
impl MiscIf for Vf2Platform {
    type MachineInfo = RiscvMachineInfo;

    fn init_boot_info(_ptr: usize) {
        // VF2 uses embedded DTB
        let dtb_ptr = FDT.as_ptr() as usize;
        DTB.call_once(|| dtb_ptr);
    }

    fn boot_info_ptr() -> usize {
        *DTB.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        let mut info = crate::common_riscv::basic::machine_info_from_dtb(*DTB.get().unwrap());
        info.initrd = Some(Range {
            start: INITRD.as_ptr() as usize,
            end: INITRD.as_ptr() as usize + INITRD.len(),
        });
        info
    }
}

// ============================================================================
// Legacy compatibility functions
// ============================================================================
pub fn init_dtb(_dtb: Option<usize>) {
    <Vf2Platform as MiscIf>::init_boot_info(0);
}

pub fn basic_machine_info() -> RiscvMachineInfo {
    <Vf2Platform as MiscIf>::machine_info()
}

pub fn set_timer(time: usize) {
    crate::common_riscv::sbi::set_timer(time);
}

pub fn system_shutdown() -> ! {
    <Vf2Platform as PowerIf>::system_off()
}

pub fn console_putchar(ch: u8) {
    <Vf2Platform as ConsoleIf>::putchar(ch);
}

