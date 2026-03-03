//! QEMU x86-64 platform support

pub mod config;

use core::ops::Range;
use spin::Once;

use crate::common_x86_64::basic::MachineInfo as X86MachineInfo;
use crate::common_x86_64::{apic, time};
use crate::traits::{ConsoleIf, MachineInfo, MiscIf, PowerIf, PlatformCallRet};

pub static BOOT_INFO: Once<usize> = Once::new();

/// QEMU x86-64 platform type
pub struct QemuX86Platform;

// ============================================================================
// ConsoleIf implementation
// ============================================================================
impl ConsoleIf for QemuX86Platform {
    fn putchar(ch: u8) {
        crate::common_x86_64::services::console_putchar(ch);
    }

    fn getchar() -> Option<u8> {
        crate::common_x86_64::services::console_getchar()
    }
}

// ============================================================================
// PowerIf implementation
// ============================================================================
impl PowerIf for QemuX86Platform {
    fn system_off() -> ! {
        crate::common_x86_64::services::system_shutdown()
    }

    fn cpu_boot(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformCallRet {
        let ret = crate::common_x86_64::services::start_secondary_cpu(cpu_id, start_addr, opaque);
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
        x86_64::instructions::hlt();
    }
}

// ============================================================================
// MachineInfo implementation for X86MachineInfo
// ============================================================================
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

// ============================================================================
// MiscIf implementation
// ============================================================================
impl MiscIf for QemuX86Platform {
    type MachineInfo = X86MachineInfo;

    fn init_boot_info(ptr: usize) {
        BOOT_INFO.call_once(|| ptr);
        
        // Initialize APIC
        apic::init_primary_apic();
        
        // Initialize time subsystem (TSC, RTC)
        time::init_time();
        
        // Initialize APIC timer (requires TSC for calibration)
        time::init_primary_apic_timer();
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        crate::common_x86_64::basic::machine_info_from_boot_info(*BOOT_INFO.get().unwrap_or(&0))
    }
}

// ============================================================================
// Legacy compatibility functions
// ============================================================================
pub fn init_boot_info(boot_info: usize) {
    <QemuX86Platform as MiscIf>::init_boot_info(boot_info);
}

#[deprecated(note = "use init_boot_info")]
pub fn init_dtb(boot_info: Option<usize>) {
    init_boot_info(boot_info.unwrap_or(0));
}

pub fn boot_info_ptr() -> usize {
    <QemuX86Platform as MiscIf>::boot_info_ptr()
}

pub fn basic_machine_info() -> X86MachineInfo {
    <QemuX86Platform as MiscIf>::machine_info()
}

pub fn set_timer(time: usize) {
    crate::common_x86_64::services::set_timer(time);
}

pub fn system_shutdown() -> ! {
    <QemuX86Platform as PowerIf>::system_off()
}

pub fn console_putchar(ch: u8) {
    <QemuX86Platform as ConsoleIf>::putchar(ch);
}

pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformCallRet {
    <QemuX86Platform as PowerIf>::cpu_boot(cpu_id, start_addr, opaque)
}

pub fn remote_instruction_fence(cpu_mask: usize, cpu_mask_base: usize) -> PlatformCallRet {
    let ret = crate::common_x86_64::services::remote_instruction_fence(cpu_mask, cpu_mask_base);
    PlatformCallRet {
        error: ret.error,
        value: ret.value,
    }
}

