use crate::{read_clock_timespec, TimeSpec};

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
pub extern "C" fn __vdso_clock_gettime(clk: usize, ts: *mut TimeSpec) -> i32 {
    if ts.is_null() {
        return -1;
    }

    // 这里不进内核：只从共享 vVAR 里读快照，命中后直接把结果写回用户缓冲区。
    // 用户态拿到 `AT_SYSINFO_EHDR` 后，调用的就是这条快路径。
    match read_clock_timespec(clk) {
        Some(time_spec) => {
            unsafe {
                *ts = time_spec;
            }
            0
        }
        None => -1,
    }
}


// 最小实现：先把 ABI 和构建链路打通，后续再接真实时间数据。
#[unsafe(no_mangle)]
pub extern "C" fn __vdso_gettimeofday(tv: *mut TimeVal, tz: *mut TimeZone) -> i32 {
    // 这一版实现和 `__vdso_clock_gettime` 共享同一份时间快照，优先保持最小闭环。
    // 这条路径和 clock_gettime 一样，只消费共享快照，不依赖 syscall。
    if !tv.is_null() {
        if let Some(time_spec) = read_clock_timespec(crate::CLOCK_REALTIME) {
            unsafe {
                *tv = TimeVal {
                    tv_sec: time_spec.tv_sec as i64,
                    tv_usec: (time_spec.tv_nsec / 1_000) as i64,
                };
            }
        }
    }

    if !tz.is_null() {
        unsafe {
            *tz = TimeZone {
                tz_minuteswest: 0,
                tz_dsttime: 0,
            };
        }
    }

    0
}