#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{
    println,
    process::{exit, fork, waitid, waitpid},
};
use pconst::{signal::{SigInfo, SignalNumber}, task::WaitOptions};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_proc] start");
    if !run_waitpid_case() {
        println!("[sys_proc] waitpid FAIL");
        return 1;
    }
    if !run_waitid_case() {
        println!("[sys_proc] waitid FAIL");
        return 1;
    }
    println!("[sys_proc] PASS");
    0
}

fn run_waitpid_case() -> bool {
    let pid = fork();
    if pid < 0 {
        println!("[sys_proc] fork failed: {}", pid);
        return false;
    }

    if pid == 0 {
        exit(7);
    }

    let mut exit_code = 0i32;
    let waited = waitpid(pid as usize, &mut exit_code);
    if waited != pid {
        println!("[sys_proc] waitpid pid mismatch: {} vs {}", waited, pid);
        return false;
    }
    if exit_code != 7 {
        println!("[sys_proc] waitpid exit code mismatch: {}", exit_code);
        return false;
    }
    true
}

fn run_waitid_case() -> bool {
    let pid = fork();
    if pid < 0 {
        println!("[sys_proc] fork2 failed: {}", pid);
        return false;
    }

    if pid == 0 {
        exit(11);
    }

    let mut info = SigInfo::default();
    let res = waitid(1, pid as usize, &mut info, WaitOptions::WEXITED);
    if res < 0 {
        println!("[sys_proc] waitid failed: {}", res);
        return false;
    }
    if info.si_signo != SignalNumber::SIGCHLD as i32 {
        println!("[sys_proc] waitid signo mismatch: {}", info.si_signo);
        return false;
    }
    if info.si_code != 1 {
        println!("[sys_proc] waitid code mismatch: {}", info.si_code);
        return false;
    }
    true
}
