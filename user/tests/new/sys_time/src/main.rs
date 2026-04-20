#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{println, time::{clock_gettime, sleep}};
use pconst::time::TimeSpec;

const CLOCK_REALTIME: usize = 0;
const CLOCK_MONOTONIC: usize = 1;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_time] start");
    let mut realtime = TimeSpec::default();
    let mut monotonic = TimeSpec::default();
    assert_eq!(clock_gettime(CLOCK_REALTIME, &mut realtime), 0);
    assert_eq!(clock_gettime(CLOCK_MONOTONIC, &mut monotonic), 0);
    println!("[sys_time] realtime={}s {}ns", realtime.tv_sec, realtime.tv_nsec);
    println!("[sys_time] monotonic={}s {}ns", monotonic.tv_sec, monotonic.tv_nsec);

    let mut last = monotonic;
    for _ in 0..4 {
        let mut current = TimeSpec::default();
        assert_eq!(clock_gettime(CLOCK_MONOTONIC, &mut current), 0);
        println!("[sys_time] monotonic_tick={}s {}ns", current.tv_sec, current.tv_nsec);
        assert!(current >= last);
        last = current;
    }

    sleep(10);
    println!("[sys_time] PASS");
    0
}
