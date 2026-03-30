//! x86_64 平台基础服务。

#![allow(unused)]

use core::arch::asm;

use super::ap;
use super::time::apic_timer;

const QEMU_EXIT_PORT: u16 = 0xf4;
const COM1_PORT: u16 = 0x3f8;

/// 设置定时器触发时间（绝对 TSC deadline）。
pub fn set_timer(deadline_tsc: usize) {
    apic_timer::set_timer_at_tsc(deadline_tsc as u64);
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
    // 先查询 UART 状态寄存器。
    let lsr: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") COM1_PORT + 5,
            out("al") lsr,
            options(nomem, nostack, preserves_flags)
        );
    }
    if lsr & 0x01 != 0 {
        // 数据就绪后再读。
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

/// 通过 APIC IPI 启动从核。
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
    ap::start_secondary_cpu(cpu_id, start_addr, opaque);
}

/// x86 指令缓存一致，保持空操作。
pub fn flush_cache(_cpu_mask: usize, _cpu_mask_base: usize) {
    // 无需额外处理。
}
