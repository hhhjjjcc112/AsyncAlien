//! APIC Timer support for x86-64
//!
//! Provides per-CPU timer using the Local APIC Timer.

use core::sync::atomic::{AtomicU64, Ordering};

use x2apic::lapic::{TimerDivide, TimerMode};

use crate::common_x86_64::apic::get_local_apic;
use crate::config::APIC_TIMER_ONESHOT;

/// APIC timer ticks per second (calibrated during init)
static APIC_TIMER_FREQUENCY: AtomicU64 = AtomicU64::new(0);
const KERNEL_TICKS_PER_SEC: u64 = 10;
/// 统计APIC timer编程次数
#[cfg(feature = "apic_timer_test")]
static APIC_TIMER_PROGRAM_COUNT: AtomicU64 = AtomicU64::new(0);

#[inline]
fn program_disabled_timer(local_apic: &mut x2apic::lapic::LocalApic) {
    unsafe {
        local_apic.disable_timer();
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::OneShot);
        local_apic.set_timer_initial(0);
    }
}

#[inline]
fn periodic_initial_ticks() -> u32 {
    let freq = APIC_TIMER_FREQUENCY.load(Ordering::Relaxed);
    let ticks = (freq / KERNEL_TICKS_PER_SEC).max(1);
    ticks.min(u32::MAX as u64) as u32
}

#[inline]
fn program_periodic_timer(local_apic: &mut x2apic::lapic::LocalApic) {
    let ticks = periodic_initial_ticks();
    unsafe {
        local_apic.enable_timer();
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::Periodic);
        local_apic.set_timer_initial(ticks);
    }
}

/// Initialize APIC timer for primary CPU (BSP)
///
/// Calibrates the timer frequency using TSC
pub fn init_primary_apic_timer() {
    calibrate_apic_timer();

    let local_apic = unsafe { get_local_apic() };
    if APIC_TIMER_ONESHOT {
        program_disabled_timer(local_apic);
    } else {
        program_periodic_timer(local_apic);
    }
    log::info!(
        "APIC Timer initialized, mode={}, frequency: {} Hz",
        if APIC_TIMER_ONESHOT { "oneshot" } else { "periodic" },
        APIC_TIMER_FREQUENCY.load(Ordering::Relaxed),
    );
}

/// Initialize APIC timer for secondary CPUs (AP)
pub fn init_secondary_apic_timer() {
    let local_apic = unsafe { get_local_apic() };
    if APIC_TIMER_ONESHOT {
        program_disabled_timer(local_apic);
    } else {
        program_periodic_timer(local_apic);
    }
}

/// Calibrate APIC timer frequency using TSC
fn calibrate_apic_timer() {
    let local_apic = unsafe { get_local_apic() };

    unsafe {
        local_apic.enable_timer();
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
    let frequency = ((elapsed as u64) * 100).max(1);
    if elapsed == 0 {
        log::warn!("APIC timer calibration returned 0, use fallback 1 Hz");
    }
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
    if !APIC_TIMER_ONESHOT {
        return;
    }

    let freq = APIC_TIMER_FREQUENCY.load(Ordering::Relaxed);
    if freq == 0 {
        log::warn!("APIC timer frequency not ready");
        return;
    }

    // Convert nanoseconds to APIC timer ticks
    // ticks = deadline_ns * freq / 1e9
    let ticks = ((deadline_ns as u128 * freq as u128 / 1_000_000_000) as u32).max(1);
    #[cfg(feature = "apic_timer_test")]
    {
        let count = APIC_TIMER_PROGRAM_COUNT.fetch_add(1, Ordering::Relaxed);
        if count % 10 == 0 {
            println!("[apic_timer_test] 编程计时器 count={} ticks={}", count, ticks);
        }
    }

    let local_apic = unsafe { get_local_apic() };
    unsafe {
        local_apic.enable_timer();
        local_apic.set_timer_divide(TimerDivide::Div1);
        local_apic.set_timer_mode(TimerMode::OneShot);
        local_apic.set_timer_initial(ticks);
    }
}

/// Set APIC timer to fire at absolute TSC deadline
///
/// Converts TSC deadline to relative APIC timer ticks
pub fn set_timer_at_tsc(deadline_tsc: u64) {
    if !APIC_TIMER_ONESHOT {
        return;
    }

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
#[allow(dead_code)]
pub fn disable_apic_timer() {
    let local_apic = unsafe { get_local_apic() };
    program_disabled_timer(local_apic);
}
