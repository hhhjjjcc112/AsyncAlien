//! x86_64 的 RTC 墙钟支持。

use core::sync::atomic::{AtomicU64, Ordering};

static RTC_EPOCH_SECONDS: AtomicU64 = AtomicU64::new(0);
static RTC_INIT_TSC: AtomicU64 = AtomicU64::new(0);

/// 初始化 RTC 并记录启动时墙钟时间。
pub fn init_rtc() {
    let rtc_time = x86_rtc::Rtc::new().get_unix_timestamp();
    RTC_EPOCH_SECONDS.store(rtc_time, Ordering::SeqCst);
    RTC_INIT_TSC.store(super::tsc::current_ticks(), Ordering::SeqCst);
    log::info!("RTC initialized: Unix timestamp = {}", rtc_time);
}

/// 获取启动时 RTC 的 Unix 时间戳。
pub fn get_rtc_epoch_seconds() -> u64 {
    RTC_EPOCH_SECONDS.load(Ordering::Relaxed)
}

/// 获取当前墙钟秒数（Unix 时间）。
/// 由启动时 RTC 时间与 TSC 运行时长合成。
pub fn current_wall_time_secs() -> u64 {
    let boot_time = RTC_EPOCH_SECONDS.load(Ordering::Relaxed);
    let elapsed = super::tsc::duration_since_tsc_init();
    boot_time + elapsed.as_secs()
}

/// 获取当前墙钟纳秒时间（Unix 时间）。
pub fn current_wall_time_nanos() -> u128 {
    let boot_time = RTC_EPOCH_SECONDS.load(Ordering::Relaxed) as u128;
    let elapsed = super::tsc::duration_since_tsc_init();
    boot_time * 1_000_000_000 + elapsed.as_nanos()
}
