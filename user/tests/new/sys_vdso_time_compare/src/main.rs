#![no_std]
#![no_main]

extern crate alloc;

use core::hint::black_box;

use Mstd::{println, time::{clock_gettime_raw, clock_gettime_vdso}};
use pconst::time::TimeSpec;

const CLOCK_MONOTONIC: usize = 1;
const WARMUP_ITERS: usize = 64;
const VERIFY_ITERS: usize = 16;
const VERIFY_THRESHOLD_US: u64 = 10000;
const BENCH_UNIT: &str = "ns";
const REPEATS: usize = 5;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_vdso_time_compare] start");

    // Phase 1: correctness verification
    match verify_correctness(VERIFY_THRESHOLD_US) {
        Ok(()) => println!("[VDSO_VERIFY] PASS"),
        Err(msg) => {
            println!("[VDSO_VERIFY] FAIL reason={}", msg);
            return 1;
        }
    }

    // Phase 2: performance measurement for multiple groups
    let groups: [usize; 5] = [64, 128, 256, 512, 1024];
    let results = measure_groups(&groups);

    for r in results.iter() {
        println!(
            "[VDSO_BENCH] samples={} unit={} vdso_elapsed={} vdso_avg={} raw_elapsed={} raw_avg={} speedup={:.2}x",
            r.samples, r.unit, r.vdso_elapsed, r.vdso_avg, r.raw_elapsed, r.raw_avg, r.speedup
        );
    }

    println!("[sys_vdso_time_compare] PASS");
    0
}

// legacy bench_compare removed—simplified single-path implementation below

fn timespec_to_ns(ts: &TimeSpec) -> u128 {
    ts.tv_sec as u128 * 1_000_000_000u128 + ts.tv_nsec as u128
}

fn verify_correctness(threshold_us: u64) -> Result<(), &'static str> {
    for i in 0..VERIFY_ITERS {
        let mut vdso_ts = TimeSpec::default();
        let mut raw_ts = TimeSpec::default();
        // call vdso then raw, with no other work in between
        assert_eq!(clock_gettime_vdso(CLOCK_MONOTONIC, &mut vdso_ts), 0);
        assert_eq!(clock_gettime_raw(CLOCK_MONOTONIC, &mut raw_ts), 0);

        let vdso_ns = timespec_to_ns(&vdso_ts);
        let raw_ns = timespec_to_ns(&raw_ts);
        let diff_ns = if vdso_ns > raw_ns { vdso_ns - raw_ns } else { raw_ns - vdso_ns };
        let diff_us = (diff_ns + 999) / 1000; // round up
        if diff_us > threshold_us as u128 {
            println!("[VDSO_VERIFY] iter={} vdso_ns={} raw_ns={} diff_us={}", i, vdso_ns, raw_ns, diff_us);
            return Err("vdso vs raw difference exceeds threshold");
        }
    }
    Ok(())
}

struct PerfResult {
    samples: usize,
    unit: &'static str,
    vdso_elapsed: u128,
    vdso_avg: u128,
    raw_elapsed: u128,
    raw_avg: u128,
    speedup: f64,
}

fn measure_groups(groups: &[usize]) -> alloc::vec::Vec<PerfResult> {
    let mut results: alloc::vec::Vec<PerfResult> = alloc::vec::Vec::new();
    for &g in groups.iter() {
        // warmup
        for _ in 0..WARMUP_ITERS {
            let _ = { let mut ts = TimeSpec::default(); assert_eq!(clock_gettime_vdso(CLOCK_MONOTONIC, &mut ts), 0); ts };
            let _ = { let mut ts = TimeSpec::default(); assert_eq!(clock_gettime_raw(CLOCK_MONOTONIC, &mut ts), 0); ts };
        }

        // perf measurement: repeat each group internally to reduce variance
        let mut vdso_avgs: alloc::vec::Vec<u128> = alloc::vec::Vec::new();
        let mut raw_avgs: alloc::vec::Vec<u128> = alloc::vec::Vec::new();

        for rep in 0..REPEATS {
            let vdso_elapsed = measure_vdso_ns(g);
            let raw_elapsed = measure_raw_ns(g);
            let vdso_avg = vdso_elapsed / g as u128;
            let raw_avg = raw_elapsed / g as u128;
            let speedup = if vdso_avg == 0 { 0.0 } else { raw_avg as f64 / vdso_avg as f64 };

            // print per-repeat raw line for parsing
            println!("[VDSO_BENCH] repeat={} samples={} unit={} vdso_elapsed={} vdso_avg={} raw_elapsed={} raw_avg={} speedup={:.2}x",
                rep+1, g, BENCH_UNIT, vdso_elapsed, vdso_avg, raw_elapsed, raw_avg, speedup);

            vdso_avgs.push(vdso_avg);
            raw_avgs.push(raw_avg);
        }

        // compute aggregate mean and population stddev (integer sqrt)
        let (vdso_mean, vdso_std) = compute_stats(&vdso_avgs);
        let (raw_mean, raw_std) = compute_stats(&raw_avgs);
        let agg_speedup = if vdso_mean == 0 { 0.0 } else { raw_mean as f64 / vdso_mean as f64 };

        println!("[VDSO_AGG] samples={} repeats={} vdso_mean={} vdso_std={} raw_mean={} raw_std={} speedup_mean={:.2}x",
            g, REPEATS, vdso_mean, vdso_std, raw_mean, raw_std, agg_speedup);

        results.push(PerfResult {
            samples: g,
            unit: BENCH_UNIT,
            vdso_elapsed: vdso_mean * g as u128, // approximate total
            vdso_avg: vdso_mean,
            raw_elapsed: raw_mean * g as u128,
            raw_avg: raw_mean,
            speedup: agg_speedup,
        });
    }
    results
}

fn compute_stats(values: &alloc::vec::Vec<u128>) -> (u128, u128) {
    let n = values.len() as u128;
    if n == 0 {
        return (0, 0);
    }
    let mut sum: u128 = 0;
    for &v in values.iter() { sum += v; }
    let mean = sum / n;

    // variance = sum((x-mean)^2)/n
    let mut var: u128 = 0;
    for &v in values.iter() {
        let diff = if v > mean { v - mean } else { mean - v };
        var += diff * diff;
    }
    var = var / n;
    let std = integer_sqrt(var);
    (mean, std)
}

fn integer_sqrt(mut x: u128) -> u128 {
    if x == 0 { return 0; }
    // initial estimate via bit shifting
    let mut res: u128 = 1 << ((127 - x.leading_zeros() as usize) / 2);
    // Newton iterations
    for _ in 0..8 {
        let res_next = (res + x / res) >> 1;
        if res_next >= res { break; }
        res = res_next;
    }
    res
}

#[inline(always)]
fn measure_vdso_ns(samples: usize) -> u128 {
    let mut ts = TimeSpec::default();
    let start = monotonic_now_ns();
    for _ in 0..samples {
        black_box(clock_gettime_vdso(CLOCK_MONOTONIC, &mut ts));
    }
    let end = monotonic_now_ns();
    end - start
}

#[inline(always)]
fn measure_raw_ns(samples: usize) -> u128 {
    let mut ts = TimeSpec::default();
    let start = monotonic_now_ns();
    for _ in 0..samples {
        black_box(clock_gettime_raw(CLOCK_MONOTONIC, &mut ts));
    }
    let end = monotonic_now_ns();
    end - start
}

// Remove architecture-specific helpers; provide unified raw time getter
#[inline(always)]
fn monotonic_now_ns() -> u128 {
    let mut ts = TimeSpec::default();
    assert_eq!(clock_gettime_raw(CLOCK_MONOTONIC, &mut ts), 0);
    timespec_to_ns(&ts)
}