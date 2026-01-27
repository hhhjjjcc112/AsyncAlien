use time::{PrimitiveDateTime};

static mut RTC_EPOCH_SECONDS: u64 = 0;
static mut RTC_BOOT_TIME: PrimitiveDateTime = datetime!(1970-01-01 00:00:00);

pub fn init_rtc() {
    let rtc_time = x86_rtc::Rtc::new().get_unix_timestamp();
    unsafe {
        RTC_EPOCH_SECONDS = rtc_time;
    }
}

pub fn get_rtc_epoch_seconds() -> u64 {
    unsafe { RTC_EPOCH_SECONDS }
}