use config::{APIC_ERROR_VECTOR, APIC_SPURIOUS_VECTOR, APIC_TIMER_VECTOR, SYSCALL_VECTOR};

// 仅保留当前 x86_64 trap 路径实际使用的向量定义，避免无效常量堆积。
pub const DIVIDE_ERROR: u8 = 0;
pub const DEBUG: u8 = 1;
pub const BREAKPOINT: u8 = 3;
pub const INVALID_OPCODE: u8 = 6;
pub const DOUBLE_FAULT: u8 = 8;
pub const GENERAL_PROTECTION: u8 = 13;
pub const PAGE_FAULT: u8 = 14;

pub const IRQ_BASE: u8 = 32;
pub const SYSCALL: u8 = SYSCALL_VECTOR;

pub const APIC_TIMER: u8 = APIC_TIMER_VECTOR;
pub const APIC_ERROR: u8 = APIC_ERROR_VECTOR;
pub const APIC_SPURIOUS: u8 = APIC_SPURIOUS_VECTOR;
