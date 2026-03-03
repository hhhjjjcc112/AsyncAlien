//! RTC (Real-Time Clock) support for x86-64
//!
//! Provides wall-clock time from the CMOS RTC.

use core::sync::atomic::{AtomicU64, Ordering};

static RTC_EPOCH_SECONDS: AtomicU64 = AtomicU64::new(0);
static RTC_INIT_TSC: AtomicU64 = AtomicU64::new(0);

/// Initialize RTC and record current wall-clock time
pub fn init_rtc() {
    let rtc_time = x86_rtc::Rtc::new().get_unix_timestamp();
    RTC_EPOCH_SECONDS.store(rtc_time, Ordering::SeqCst);
    RTC_INIT_TSC.store(super::tsc::current_ticks(), Ordering::SeqCst);
    log::info!("RTC initialized: Unix timestamp = {}", rtc_time);
}

/// Get the Unix timestamp from RTC at boot time
pub fn get_rtc_epoch_seconds() -> u64 {
    RTC_EPOCH_SECONDS.load(Ordering::Relaxed)
}

/// Get current wall-clock time in seconds since Unix epoch
/// This combines RTC boot time with TSC elapsed time
pub fn current_wall_time_secs() -> u64 {
    let boot_time = RTC_EPOCH_SECONDS.load(Ordering::Relaxed);
    let elapsed = super::tsc::duration_since_tsc_init();
    boot_time + elapsed.as_secs()
}

/// Get current wall-clock time in nanoseconds since Unix epoch
pub fn current_wall_time_nanos() -> u128 {
    let boot_time = RTC_EPOCH_SECONDS.load(Ordering::Relaxed) as u128;
    let elapsed = super::tsc::duration_since_tsc_init();
    boot_time * 1_000_000_000 + elapsed.as_nanos()
}
