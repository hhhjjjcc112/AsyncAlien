#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{println, time::{clock_gettime_raw, clock_gettime_vdso, sleep}};
use pconst::time::TimeSpec;

const CLOCK_MONOTONIC: usize = 1;
const MAX_DELTA_NS: u128 = 1_000_000; // 1ms

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_vdso_time_compare] start");

    sleep(10000);

    let mut vdso_ts = TimeSpec::default();
    let mut raw_ts = TimeSpec::default();

    assert_eq!(clock_gettime_vdso(CLOCK_MONOTONIC, &mut vdso_ts), 0);
    assert_eq!(clock_gettime_raw(CLOCK_MONOTONIC, &mut raw_ts), 0);

    let vdso_ns = timespec_to_ns(&vdso_ts);
    let raw_ns = timespec_to_ns(&raw_ts);
    let delta_ns = vdso_ns.abs_diff(raw_ns);

    println!(
        "[sys_vdso_time_compare] vdso={}s {}ns raw={}s {}ns delta={}ns",
        vdso_ts.tv_sec,
        vdso_ts.tv_nsec,
        raw_ts.tv_sec,
        raw_ts.tv_nsec,
        delta_ns,
    );

    assert!(delta_ns <= MAX_DELTA_NS);
    println!("[sys_vdso_time_compare] PASS");
    0
}

fn timespec_to_ns(ts: &TimeSpec) -> u128 {
    ts.tv_sec as u128 * 1_000_000_000 + ts.tv_nsec as u128
}