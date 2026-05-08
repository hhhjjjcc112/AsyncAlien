// utils/x86-apic/src/lib.rs
// x86 APIC 库 - 安全包装 x2apic crate，集中所有 unsafe 操作

#![no_std]

pub mod error;
pub mod types;
pub mod msr;
pub mod port_io;

pub use error::{ApicError, Result};
pub use types::{LocalApicContext, IoApicContext};

pub use x2apic::ioapic::{IrqFlags, IrqMode};
pub use x2apic::lapic::{TimerDivide, TimerMode};
