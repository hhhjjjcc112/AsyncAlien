use core::time::Duration;

use crate::{info, time::{apic_timer::init_primary_apic_timer, rtc::init_rtc, tsc::{duration_since_tsc_init, init_tsc}}};

mod tsc;
mod rtc;
mod apic_timer;

pub fn init_time() {
    info!("Initializing Time Subsystem...");
    init_tsc();
    init_rtc();
    // init_hpet();
    // init_primary_apic_timer();
}

pub fn busy_wait(duration: Duration) {
    let start = duration_since_tsc_init();
    while duration_since_tsc_init() - start < duration {
        core::hint::spin_loop();
    }
}

