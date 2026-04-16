pub use pconst::time::{TimeSpec, TimeVal, Times};

use crate::syscall::{sys_gettimeofday, sys_nanosleep};

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

pub fn sleep(ms: usize) {
    let mut ts = TimeSpec::default();
    ts.tv_sec = ms / 1000;
    ts.tv_nsec = (ms % 1000) * 1_000_000;
    sys_nanosleep(&mut ts as *mut TimeSpec as *mut u8, core::ptr::null_mut());
}
