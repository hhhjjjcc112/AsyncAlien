//! APIC Timer support for x86-64
//!
//! Provides per-CPU timer using the Local APIC Timer.

use core::sync::atomic::{AtomicU64, Ordering};

use x2apic::lapic::{TimerDivide, TimerMode};

use crate::common_x86_64::apic::get_local_apic;

/// APIC timer ticks per second (calibrated during init)
static APIC_TIMER_FREQUENCY: AtomicU64 = AtomicU64::new(0);

/// Initialize APIC timer for primary CPU (BSP)
///
/// Calibrates the timer frequency using TSC
pub fn init_primary_apic_timer() {
    calibrate_apic_timer();

    let local_apic = unsafe { get_local_apic() };
    unsafe {
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::Periodic);
        // Initially disabled (no initial count)
        local_apic.set_timer_initial(0);
    }
    log::info!(
        "APIC Timer initialized, frequency: {} Hz",
        APIC_TIMER_FREQUENCY.load(Ordering::Relaxed)
    );
}

/// Initialize APIC timer for secondary CPUs (AP)
pub fn init_secondary_apic_timer() {
    let local_apic = unsafe { get_local_apic() };
    unsafe {
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::Periodic);
        local_apic.set_timer_initial(0);
    }
}

/// Calibrate APIC timer frequency using TSC
fn calibrate_apic_timer() {
    let local_apic = unsafe { get_local_apic() };

    unsafe {
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::OneShot);
        local_apic.set_timer_initial(0xFFFF_FFFF);
    }

    // Wait 10ms using TSC
    let wait_duration = core::time::Duration::from_millis(10);
    super::busy_wait(wait_duration);

    let remaining = unsafe { local_apic.timer_current() };
    let elapsed = 0xFFFF_FFFF - remaining;

    // Calculate frequency: ticks_per_10ms * 100 = ticks_per_second
    let frequency = (elapsed as u64) * 100;
    APIC_TIMER_FREQUENCY.store(frequency, Ordering::SeqCst);

    // Stop the timer
    unsafe {
        local_apic.set_timer_initial(0);
    }
}

/// Set APIC timer to fire after specified nanoseconds
///
/// This is used for implementing `set_timer` platform interface
pub fn set_apic_timer(deadline_ns: u64) {
    let freq = APIC_TIMER_FREQUENCY.load(Ordering::Relaxed);
    if freq == 0 {
        return;
    }

    // Convert nanoseconds to APIC timer ticks
    // ticks = deadline_ns * freq / 1e9
    let ticks = (deadline_ns as u128 * freq as u128 / 1_000_000_000) as u32;

    let local_apic = unsafe { get_local_apic() };
    unsafe {
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::OneShot);
        local_apic.set_timer_initial(ticks);
    }
}

/// Set APIC timer to fire at absolute TSC deadline
///
/// Converts TSC deadline to relative APIC timer ticks
pub fn set_timer_at_tsc(deadline_tsc: u64) {
    let current_tsc = super::tsc::current_ticks();
    if deadline_tsc <= current_tsc {
        // Deadline passed, fire immediately
        set_apic_timer(0);
        return;
    }

    let delta_tsc = deadline_tsc - current_tsc;
    let delta_ns = super::tsc::ticks_to_nanos(delta_tsc);
    set_apic_timer(delta_ns);
}

/// Disable APIC timer
pub fn disable_apic_timer() {
    let local_apic = unsafe { get_local_apic() };
    unsafe {
        local_apic.set_timer_initial(0);
    }
}
