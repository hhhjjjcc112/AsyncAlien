//! Platform services for x86-64
//!
//! Provides platform-level operations like timer, shutdown, console.

#![allow(unused)]

use core::arch::asm;

use super::ap;
use super::time::apic_timer;

const QEMU_EXIT_PORT: u16 = 0xf4;
const COM1_PORT: u16 = 0x3f8;

/// Platform-neutral return type for platform calls
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlatformRet {
    pub error: isize,
    pub value: isize,
}

/// Set timer to fire at specified time (in nanoseconds from boot)
pub fn set_timer(time_ns: usize) {
    apic_timer::set_apic_timer(time_ns as u64);
}

pub fn system_shutdown() -> ! {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") QEMU_EXIT_PORT,
            in("eax") 0x10_u32,
            options(nomem, nostack, preserves_flags)
        );
    }
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) }
    }
}

pub fn console_putchar(ch: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") COM1_PORT,
            in("al") ch,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn console_getchar() -> Option<u8> {
    // Check if data is available in UART
    let lsr: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") COM1_PORT + 5,  // Line Status Register
            out("al") lsr,
            options(nomem, nostack, preserves_flags)
        );
    }
    if lsr & 0x01 != 0 {
        // Data ready
        let ch: u8;
        unsafe {
            core::arch::asm!(
                "in al, dx",
                in("dx") COM1_PORT,
                out("al") ch,
                options(nomem, nostack, preserves_flags)
            );
        }
        Some(ch)
    } else {
        None
    }
}

/// Start another CPU core (AP startup via APIC IPI on x86-64)
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformRet {
    let (error, value) = ap::start_secondary_cpu(cpu_id, start_addr, opaque);
    PlatformRet { error, value }
}

/// Flush instruction cache on remote CPUs (x86 has coherent I-cache, no-op)
pub fn remote_instruction_fence(_cpu_mask: usize, _cpu_mask_base: usize) -> PlatformRet {
    // x86-64 has coherent instruction cache, no action needed
    PlatformRet { error: 0, value: 0 }
}
