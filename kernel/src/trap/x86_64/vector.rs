use config::{APIC_ERROR_VECTOR, APIC_SPURIOUS_VECTOR, APIC_TIMER_VECTOR, SYSCALL_VECTOR};

pub const DIVIDE_ERROR: u8 = 0;
pub const DEBUG: u8 = 1;
pub const NMI: u8 = 2;
pub const BREAKPOINT: u8 = 3;
pub const OVERFLOW: u8 = 4;
pub const BOUND_RANGE: u8 = 5;
pub const INVALID_OPCODE: u8 = 6;
pub const DEVICE_NOT_AVAILABLE: u8 = 7;
pub const DOUBLE_FAULT: u8 = 8;
pub const INVALID_TSS: u8 = 10;
pub const SEGMENT_NOT_PRESENT: u8 = 11;
pub const STACK_SEGMENT_FAULT: u8 = 12;
pub const GENERAL_PROTECTION: u8 = 13;
pub const PAGE_FAULT: u8 = 14;
pub const X87_FLOATING_POINT: u8 = 16;
pub const ALIGNMENT_CHECK: u8 = 17;
pub const MACHINE_CHECK: u8 = 18;
pub const SIMD_FLOATING_POINT: u8 = 19;
pub const VIRTUALIZATION: u8 = 20;
pub const SECURITY_EXCEPTION: u8 = 30;

pub const IRQ_BASE: u8 = 32;
pub const TIMER: u8 = IRQ_BASE + 0;
pub const KEYBOARD: u8 = IRQ_BASE + 1;
pub const SYSCALL: u8 = SYSCALL_VECTOR;

pub const APIC_TIMER: u8 = APIC_TIMER_VECTOR;
pub const APIC_ERROR: u8 = APIC_ERROR_VECTOR;
pub const APIC_SPURIOUS: u8 = APIC_SPURIOUS_VECTOR;
