use core::{arch::x86_64::_rdtsc, time::Duration};

use crate::info;

static mut TSC_FREQUENCY_HZ: u64 = 0;
static mut TSC_INIT_TICKS: u64 = 0;

pub fn init_tsc() {
    let cpuid = raw_cpuid::CpuId::new();

    let has_tsc = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_tsc());
    assert!(has_tsc, "CPU does not support TSC!");

    let tsc_freq = if let Some(tsc_freq_read) = cpuid
        .get_tsc_info()
        .and_then(|tsc_info| tsc_info.tsc_frequency())
    {
        info!("TSC Frequency detected: {} Hz", tsc_freq_read);
        tsc_freq_read
    } else {
        let processor_freq = cpuid.get_processor_frequency_info()
            .map_or(3 * 1_000_000_000, |pfinfo| pfinfo.processor_base_frequency() as u64 * 1_000_000);
        info!("TSC Frequency estimated from processor base frequency: {} Hz", processor_freq);
        processor_freq
    };

    let invariant_tsc = if let Some(apmi) = cpuid.get_advanced_power_mgmt_info() {
        apmi.has_invariant_tsc()
    } else {
        false
    };
    info!("Invariant TSC: {}", invariant_tsc);

    unsafe {
        TSC_FREQUENCY_HZ = tsc_freq;
        TSC_INIT_TICKS = _rdtsc();
    }
}


pub fn duration_since_tsc_init() -> Duration {
    let current_ticks = unsafe { _rdtsc() };
    let init_ticks = unsafe { TSC_INIT_TICKS };
    let tsc_freq = unsafe { TSC_FREQUENCY_HZ };

    let elapsed_ticks = current_ticks - init_ticks;
    let elapsed_nanos = (elapsed_ticks as u128 * 1_000_000_000u128) / (tsc_freq as u128);
    let secs = (elapsed_nanos / 1_000_000_000u128) as u64;
    let nanos = (elapsed_nanos % 1_000_000_000u128) as u32;

    Duration::new(secs, nanos)
}