pub use pconst::time::{TimeSpec, TimeVal, Times};

use crate::syscall::{sys_clock_gettime, sys_gettimeofday, sys_nanosleep};

pub fn now_timeval() -> TimeVal {
    let mut tv = TimeVal::default();
    let _ = get_time_of_day(&mut tv);
    tv
}

pub fn get_time_ms() -> isize {
    let mut tv = TimeVal::default();
    if get_time_of_day(&mut tv) != 0 {
        return 0;
    }
    tv.tv_sec as isize * 1000 + tv.tv_usec as isize / 1000
}

pub fn get_time_of_day(tv: &mut TimeVal) -> isize {
    let res = sys_gettimeofday(tv as *mut TimeVal as *mut u8, core::ptr::null_mut());
    if res != 0 {
        return -1;
    }
    0
}

pub fn clock_gettime_vdso(clk_id: usize, ts: &mut TimeSpec) -> isize {
    if crate::vdso::clock_gettime_vdso(clk_id, ts) {
        return 0;
    }
    -1
}

pub fn clock_gettime_raw(clk_id: usize, ts: &mut TimeSpec) -> isize {
    let res = sys_clock_gettime(clk_id, ts as *mut TimeSpec as *mut u8);
    if res != 0 {
        return -1;
    }
    0
}

pub fn clock_gettime(clk_id: usize, ts: &mut TimeSpec) -> isize {
    // 用户态优先走 vDSO；这条路径失败时，再退回到原来的 syscall 实现。
    if clock_gettime_vdso(clk_id, ts) == 0 {
        return 0;
    }

    clock_gettime_raw(clk_id, ts)
}

pub fn sleep(ms: usize) {
    let mut ts = TimeSpec::default();
    ts.tv_sec = ms / 1000;
    ts.tv_nsec = (ms % 1000) * 1_000_000;
    sys_nanosleep(&mut ts as *mut TimeSpec as *mut u8, core::ptr::null_mut());
}
