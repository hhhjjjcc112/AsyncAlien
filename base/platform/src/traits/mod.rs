//! Platform abstraction traits (ArceOS-style)
//!
//! This module defines unified platform interface traits. Each platform
//! implements these traits to provide a consistent API across architectures.
//!
//! ## Design Philosophy
//!
//! Following ArceOS's `axplat` design pattern:
//! - Define common traits for platform operations
//! - Each platform (QEMU RISC-V, QEMU x86-64, VF2) implements traits
//! - lib.rs selects implementation via cfg and exports unified functions
//!
//! ## What Cannot Be Unified (platform-specific assembly/structure):
//! - Boot sequence (Multiboot vs SBI)
//! - Interrupt vector table (IDT vs stvec)
//! - Page table format (4-level x86 vs Sv39 RISC-V)
//! - Privilege registers (RFLAGS/CS vs sstatus/SPP)
//!
//! ## What Can Be Unified Through Traits:
//! - Console I/O
//! - Time management
//! - Power control
//! - Memory information
//! - IRQ handling

#![allow(unused)]

mod console;
mod irq;
mod mem;
mod misc;
mod power;
mod time;

pub use console::ConsoleIf;
pub use irq::{IpiTarget, IrqIf};
pub use mem::{MemIf, MemRegionFlags, PhysMemRegion, RawRange};
pub use misc::{MachineInfo, MiscIf, PlatformCallRet};
pub use power::PowerIf;
pub use time::TimeIf;
