#![no_std]
#![no_main]

extern crate alloc;

use pconst::{task::WaitOptions, LinuxErrno::ECHILD};
use Mstd::{
    println,
    process::{exec, exit, fork, wait, waitpid},
    thread::m_yield,
};

#[unsafe(no_mangle)]
fn main() -> isize {
    println!("Init process is running");
    for (name, path) in TEST_CASES {
        println!("[Init] start {}", path.trim_end_matches('\0'));
        if let Err(exit_code) = run_one(name, path) {
            println!(
                "[Init] FAIL {} exit_code={}",
                path.trim_end_matches('\0'),
                exit_code
            );
            return exit_code;
        }
        println!("[Init] PASS {}", path.trim_end_matches('\0'));
    }
    println!("[Init] all tests passed");
    run_shell_and_reap()
}

fn run_shell_and_reap() -> ! {
    let shell_pid = fork();
    if shell_pid < 0 {
        println!("[Init] fork shell failed: {}", shell_pid);
        exit(127);
    }
    if shell_pid == 0 {
        exec_shell();
    }

    // init 本身保留为 reaper，持续回收退出的子进程。
    loop {
        let mut exit_code = 0i32;
        let tid = wait(&mut exit_code, WaitOptions::WNOHANG);
        if tid == -1 || tid == 0 || tid == isize::from(ECHILD) {
            m_yield();
            continue;
        }
        if tid == shell_pid {
            println!("[Init] shell exit_code={}", exit_code);
        } else {
            println!("[Init] released child tid={}, exit_code={}", tid, exit_code);
        }
    }
}

fn exec_shell() -> ! {
    let shell_path = "/bin/sh\0";
    let shell_name = "sh\0";
    let argv = [shell_name.as_ptr(), core::ptr::null()];
    exec(shell_path, &argv, TEST_ENV);
    println!("[Init] exec shell failed");
    exit(127);
}

fn run_one(name: &str, path: &str) -> Result<(), isize> {
    let pid = fork();
    if pid < 0 {
        println!("[Init] fork failed for {}", path.trim_end_matches('\0'));
        return Err(pid);
    }

    if pid == 0 {
        let argv = [name.as_ptr(), core::ptr::null()];
        let rc = exec(path, &argv, TEST_ENV);
        println!(
            "[Init] exec failed: {} rc={}",
            path.trim_end_matches('\0'),
            rc
        );
        exit(127);
    }

    let mut exit_code = 0i32;
    let waited = waitpid(pid as usize, &mut exit_code);
    if waited != pid {
        println!(
            "[Init] waitpid mismatch for {}: waited={}, pid={}",
            path.trim_end_matches('\0'),
            waited,
            pid
        );
        return Err(-1);
    }
    if exit_code != 0 {
        return Err(exit_code as isize);
    }
    Ok(())
}

// 最小测试集：先 syscall_all，再 ping。
const TEST_CASES: &[(&str, &str)] = &[
    ("syscall_all\0", "/tests/new/syscall_all\0"),
    ("ping\0", "/tests/ping\0"),
];

const TEST_ENV: &[*const u8] = &[
    "SHELL=/bin/sh\0".as_ptr(),
    "PWD=/\0".as_ptr(),
    "LOGNAME=root\0".as_ptr(),
    "MOTD_SHOWN=pam\0".as_ptr(),
    "HOME=/root\0".as_ptr(),
    "LANG=C.UTF-8\0".as_ptr(),
    "TERM=vt220\0".as_ptr(),
    "USER=root\0".as_ptr(),
    "SHLVL=0\0".as_ptr(),
    "OLDPWD=/root\0".as_ptr(),
    "PS1=\x1b[1m\x1b[32mAlien\x1b[0m:\x1b[1m\x1b[34m\\w\x1b[0m\\$ \0".as_ptr(),
    "_=/tests/init\0".as_ptr(),
    "PATH=/tests/new:/tests:/bin:/sbin:/\0".as_ptr(),
    "LD_LIBRARY_PATH=/tests:/bin\0".as_ptr(),
    core::ptr::null(),
];

#[allow(unused)]
fn run_test() {
    let commands = [
        "./time-test\0",
        "./interrupts-test-1\0",
        "./interrupts-test-2\0",
        "./copy-file-range-test-1\0",
        "./copy-file-range-test-2\0",
        "./copy-file-range-test-3\0",
        "./copy-file-range-test-4\0",
        "./lua_testcode.sh\0",
        "./busybox_testcode.sh\0",
        "./run-static.sh\0",
        "./run-dynamic.sh\0",
        "./libc-bench\0",
        "./cyclictest_testcode.sh\0",
        "./netperf_testcode.sh\0",
        "./iperf_testcode.sh\0",
        "./lmbench_testcode.sh\0",
        "./iozone_testcode.sh\0",
        "./unixbench_testcode.sh\0",
    ];
    commands.into_iter().for_each(|app| {
        let args = [app.as_ptr()];
        let pid = fork();
        if pid == 0 {
            exec(app, &args, TEST_ENV);
            exit(0);
        } else {
            m_yield();
            let mut exit_code: i32 = 0;
            let _x = waitpid(pid as usize, &mut exit_code);
        }
    });
}
