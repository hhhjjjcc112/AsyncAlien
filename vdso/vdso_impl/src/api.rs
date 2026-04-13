use crate::{bump_layout_counters, read_layout_probe, LayoutProbe};

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct TimeZone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}


#[unsafe(no_mangle)]
pub extern "C" fn __vdso_probe_layout() -> LayoutProbe {
    bump_layout_counters();
    read_layout_probe()
}


// 最小实现：先把 ABI 和构建链路打通，后续再接真实时间数据。
#[unsafe(no_mangle)]
pub extern "C" fn __vdso_gettimeofday(tv: *mut TimeVal, tz: *mut TimeZone) -> i32 {
    bump_layout_counters();

    if !tv.is_null() {
        unsafe {
            *tv = TimeVal {
                tv_sec: 2455,
                tv_usec: 0123,
            };
        }
    }

    if !tz.is_null() {
        unsafe {
            *tz = TimeZone {
                tz_minuteswest: 7799,
                tz_dsttime: 045,
            };
        }
    }

    0
}