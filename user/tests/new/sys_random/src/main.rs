#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{println, random::{getrandom, getrandom_with_flags, GRND_NONBLOCK}};
use pconst::LinuxErrno;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_random] start");
    if !run_case() {
        println!("[sys_random] FAIL");
        return 1;
    }
    println!("[sys_random] PASS");
    0
}

fn run_case() -> bool {
    let mut first = [0u8; 64];
    let mut second = [0u8; 64];
    let mut empty = [0u8; 0];

    if getrandom(&mut first) != first.len() as isize {
        println!("[sys_random] first getrandom failed");
        return false;
    }
    if getrandom(&mut second) != second.len() as isize {
        println!("[sys_random] second getrandom failed");
        return false;
    }

    if all_same(&first) || all_same(&second) {
        println!("[sys_random] buffer is constant");
        return false;
    }

    if first == second {
        println!("[sys_random] repeated buffers are identical");
        return false;
    }

    if getrandom_with_flags(&mut empty, 0) != 0 {
        println!("[sys_random] zero length failed");
        return false;
    }

    if getrandom_with_flags(&mut first, GRND_NONBLOCK) != first.len() as isize {
        println!("[sys_random] nonblock flag rejected");
        return false;
    }

    if getrandom_with_flags(&mut second, 0x8000_0000) != isize::from(LinuxErrno::EINVAL) {
        println!("[sys_random] invalid flags not rejected");
        return false;
    }

    true
}

fn all_same(buf: &[u8]) -> bool {
    buf.windows(2).all(|window| window[0] == window[1])
}