//! x86_64 时间子系统。

pub mod apic_timer;
pub mod rtc;
pub mod tsc;

use core::time::Duration;

pub use apic_timer::{init_primary_apic_timer, init_secondary_apic_timer};
pub use rtc::{get_rtc_epoch_seconds, init_rtc};
pub use tsc::{
    current_ticks, duration_since_tsc_init, init_tsc, ticks_to_nanos, tsc_frequency,
};

/// 初始化全部时间相关子系统。
pub fn init_time() {
    log::info!("Initializing Time Subsystem...");
    init_tsc();
    init_rtc();
}

/// 使用 TSC 自旋等待指定时长。
pub fn busy_wait(duration: Duration) {
    let start = duration_since_tsc_init();
    while duration_since_tsc_init() - start < duration {
        core::hint::spin_loop();
    }
}

/// 获取开机以来的纳秒时间。
pub fn current_time_nanos() -> u64 {
    let ticks = current_ticks();
    ticks_to_nanos(ticks)
}
