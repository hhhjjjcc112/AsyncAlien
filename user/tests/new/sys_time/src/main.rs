#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{println, time::sleep};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_time] start");
    sleep(10);
    println!("[sys_time] PASS");
    0
}
