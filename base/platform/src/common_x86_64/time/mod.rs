//! Time subsystem for x86-64
//!
//! Provides TSC-based timing, RTC wallclock, and APIC timer support.

pub mod apic_timer;
pub mod rtc;
pub mod tsc;

use core::time::Duration;

pub use apic_timer::{init_primary_apic_timer, init_secondary_apic_timer, set_apic_timer};
pub use rtc::{get_rtc_epoch_seconds, init_rtc};
pub use tsc::{
    current_ticks, duration_since_tsc_init, init_tsc, ticks_to_nanos, tsc_frequency,
    NANOS_PER_TICK,
};

/// Initialize all time-related subsystems
pub fn init_time() {
    log::info!("Initializing Time Subsystem...");
    init_tsc();
    init_rtc();
}

/// Busy-wait for a specified duration using TSC
pub fn busy_wait(duration: Duration) {
    let start = duration_since_tsc_init();
    while duration_since_tsc_init() - start < duration {
        core::hint::spin_loop();
    }
}

/// Get current time in nanoseconds since boot
pub fn current_time_nanos() -> u64 {
    let ticks = current_ticks();
    ticks_to_nanos(ticks)
}
