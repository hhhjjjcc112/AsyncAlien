pub mod config;

use core::ops::Range;
use spin::Once;

use crate::common_riscv::basic::MachineInfo as RiscvMachineInfo;
use crate::traits::{ConsoleIf, MachineInfo, MiscIf, PowerIf, PlatformCallRet};

pub static BOOT_INFO: Once<usize> = Once::new();
#[deprecated(note = "use BOOT_INFO")]
pub use BOOT_INFO as DTB;

/// QEMU RISC-V platform type
pub struct QemuRiscvPlatform;

// ============================================================================
// ConsoleIf implementation
// ============================================================================
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

// ============================================================================
// PowerIf implementation
// ============================================================================
impl PowerIf for QemuRiscvPlatform {
    fn system_off() -> ! {
        crate::println!("shutdown...");
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
// MachineInfo implementation for RiscvMachineInfo
// ============================================================================
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

// ============================================================================
// MiscIf implementation
// ============================================================================
impl MiscIf for QemuRiscvPlatform {
    type MachineInfo = RiscvMachineInfo;

    fn init_boot_info(ptr: usize) {
        BOOT_INFO.call_once(|| ptr);
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        crate::common_riscv::basic::machine_info_from_boot_info(*BOOT_INFO.get().unwrap())
    }
}

// ============================================================================
// Legacy compatibility functions
// ============================================================================
#[allow(dead_code)]
pub fn init_dtb(dtb: Option<usize>) {
    let dtb_ptr = dtb.expect("No dtb found");
    <QemuRiscvPlatform as MiscIf>::init_boot_info(dtb_ptr);
}

#[allow(dead_code)]
pub fn basic_machine_info() -> RiscvMachineInfo {
    <QemuRiscvPlatform as MiscIf>::machine_info()
}

#[allow(dead_code)]
pub fn set_timer(time: usize) {
    crate::common_riscv::sbi::set_timer(time);
}

#[allow(dead_code)]
pub fn system_shutdown() -> ! {
    <QemuRiscvPlatform as PowerIf>::system_off()
}

#[allow(dead_code)]
pub fn console_putchar(ch: u8) {
    <QemuRiscvPlatform as ConsoleIf>::putchar(ch);
}
