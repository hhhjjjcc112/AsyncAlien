//! 平台抽象 trait 集合。
//!
//! 各平台只需实现这些 trait，即可对外提供统一接口。

#![allow(unused)]

mod console;
mod mem;
mod misc;
mod power;
mod time;

pub use console::ConsoleIf;
pub use mem::{MemIf, MemRegionFlags, PhysMemRegion, RawRange};
pub use misc::{MachineInfo, MiscIf};
pub use power::PowerIf;
pub use time::TimeIf;
