#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{
    println,
    process::{exit, exec, fork, waitpid},
};

const ENV: &[*const u8] = &[
    "SHELL=/bin/sh\0".as_ptr(),
    "PWD=/\0".as_ptr(),
    "HOME=/root\0".as_ptr(),
    "TERM=vt220\0".as_ptr(),
    "PATH=/:/bin:/sbin:/tests:/tests/new\0".as_ptr(),
    core::ptr::null(),
];

const TESTS: &[&str] = &[
    "/sys_time\0",
    "/sys_fs\0",
    "/sys_link\0",
    "/sys_proc\0",
];

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[syscall_all] start");
    for test in TESTS {
        if !run_one(test) {
            println!("[syscall_all] FAIL: {}", test.trim_end_matches('\0'));
            return 1;
        }
    }
    println!("[syscall_all] PASS");
    0
}

fn run_one(path: &str) -> bool {
    let pid = fork();
    if pid < 0 {
        println!("[syscall_all] fork failed: {}", pid);
        return false;
    }

    if pid == 0 {
        let args = [path.as_ptr(), core::ptr::null()];
        let res = exec(path, &args, ENV);
        println!("[syscall_all] exec failed: {} -> {}", path.trim_end_matches('\0'), res);
        exit(1);
    }

    let mut exit_code = 0i32;
    let waited = waitpid(pid as usize, &mut exit_code);
    if waited != pid {
        println!("[syscall_all] waitpid mismatch: {} vs {}", waited, pid);
        return false;
    }
    if exit_code != 0 {
        println!("[syscall_all] child exit {} for {}", exit_code, path.trim_end_matches('\0'));
        return false;
    }
    true
}
