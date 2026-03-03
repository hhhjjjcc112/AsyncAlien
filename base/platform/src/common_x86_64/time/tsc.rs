//! TSC (Time Stamp Counter) support for x86-64
//!
//! Provides high-resolution timing based on the CPU's TSC.

use core::{
    arch::x86_64::_rdtsc,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

static TSC_FREQUENCY_HZ: AtomicU64 = AtomicU64::new(0);
static TSC_INIT_TICKS: AtomicU64 = AtomicU64::new(0);

/// Nanoseconds per TSC tick (fixed-point, 32.32 format for precision)
pub static NANOS_PER_TICK: AtomicU64 = AtomicU64::new(0);

/// Initialize TSC timing subsystem
pub fn init_tsc() {
    let cpuid = raw_cpuid::CpuId::new();

    let has_tsc = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_tsc());
    assert!(has_tsc, "CPU does not support TSC!");

    // Try to get TSC frequency from CPUID
    let tsc_freq = if let Some(tsc_freq_read) = cpuid
        .get_tsc_info()
        .and_then(|tsc_info| tsc_info.tsc_frequency())
    {
        log::info!("TSC Frequency detected: {} Hz", tsc_freq_read);
        tsc_freq_read
    } else {
        // Fall back to processor base frequency
        let processor_freq = cpuid.get_processor_frequency_info().map_or(
            3 * 1_000_000_000,
            |pfinfo| pfinfo.processor_base_frequency() as u64 * 1_000_000,
        );
        log::info!(
            "TSC Frequency estimated from processor base frequency: {} Hz",
            processor_freq
        );
        processor_freq
    };

    // Check for invariant TSC (constant rate regardless of CPU state)
    let invariant_tsc = if let Some(apmi) = cpuid.get_advanced_power_mgmt_info() {
        apmi.has_invariant_tsc()
    } else {
        false
    };
    log::info!("Invariant TSC: {}", invariant_tsc);

    // Calculate nanos per tick in fixed-point format (32.32)
    // nanos_per_tick = 1e9 / freq
    // In fixed-point: (1e9 << 32) / freq
    let nanos_per_tick_fp = ((1_000_000_000u128) << 32) / (tsc_freq as u128);

    TSC_FREQUENCY_HZ.store(tsc_freq, Ordering::SeqCst);
    NANOS_PER_TICK.store(nanos_per_tick_fp as u64, Ordering::SeqCst);
    TSC_INIT_TICKS.store(unsafe { _rdtsc() }, Ordering::SeqCst);
}

/// Get TSC frequency in Hz
pub fn tsc_frequency() -> u64 {
    TSC_FREQUENCY_HZ.load(Ordering::Relaxed)
}

/// Read current TSC value
#[inline]
pub fn current_ticks() -> u64 {
    unsafe { _rdtsc() }
}

/// Convert TSC ticks to nanoseconds
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    let nanos_fp = NANOS_PER_TICK.load(Ordering::Relaxed);
    // Fixed-point multiplication: (ticks * nanos_per_tick) >> 32
    ((ticks as u128 * nanos_fp as u128) >> 32) as u64
}

/// Get elapsed duration since TSC initialization
pub fn duration_since_tsc_init() -> Duration {
    let current_ticks = unsafe { _rdtsc() };
    let init_ticks = TSC_INIT_TICKS.load(Ordering::Relaxed);
    let tsc_freq = TSC_FREQUENCY_HZ.load(Ordering::Relaxed);

    let elapsed_ticks = current_ticks.saturating_sub(init_ticks);
    let elapsed_nanos = (elapsed_ticks as u128 * 1_000_000_000u128) / (tsc_freq as u128);
    let secs = (elapsed_nanos / 1_000_000_000u128) as u64;
    let nanos = (elapsed_nanos % 1_000_000_000u128) as u32;

    Duration::new(secs, nanos)
}

/// Get elapsed ticks since TSC initialization
pub fn ticks_since_init() -> u64 {
    let current = unsafe { _rdtsc() };
    let init = TSC_INIT_TICKS.load(Ordering::Relaxed);
    current.saturating_sub(init)
}
