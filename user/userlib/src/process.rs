use bitflags::bitflags;
pub use pconst::task::CloneFlags;
use pconst::{signal::SigInfo, sys::Rusage, task::WaitOptions};

use crate::syscall::{
    sys_execve, sys_exit, sys_fork, sys_getpid, sys_getpriority, sys_setpriority, sys_vfork,
    sys_waitid, sys_waitpid,
};

pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
    loop {}
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn vfork() -> isize {
    sys_vfork()
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn exec(cmd: &str, args: &[*const u8], env: &[*const u8]) -> isize {
    sys_execve(
        cmd.as_ptr(),
        args.as_ptr() as *const usize,
        env.as_ptr() as *const usize,
    )
}

pub fn wait(exit_code: &mut i32, option: WaitOptions) -> isize {
    sys_waitpid(-1, exit_code as *mut _, option.bits())
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid as isize, exit_code as *mut _, 0)
}

/// 直接透传 rusage，保持 waitid 的 Linux 原型。
pub fn waitid(
    which: usize,
    pid: usize,
    info: &mut SigInfo,
    option: WaitOptions,
    rusage: *mut Rusage,
) -> isize {
    sys_waitid(
        which,
        pid,
        info as *mut SigInfo as *mut u8,
        option.bits() as usize,
        rusage as usize,
    )
}

pub fn set_priority(which: i32, who: u32, prio: i32) -> isize {
    sys_setpriority(which, who, prio)
}

pub fn get_priority(which: i32, who: u32) -> i32 {
    sys_getpriority(which, who) as i32
}

bitflags! {
    pub struct SignalFlags:u32 {
        const SIGHUP = 1;
        const SIGINT = 2;
        const SIGQUIT = 3;
        const SIGILL = 4;
        const SIGTRAP = 5;
        const SIGABRT = 6;
        const SIGBUS = 7;
        const SIGFPE = 8;
        const SIGKILL = 9;
        const SIGUSR1 = 10;
        const SIGSEGV = 11;
        const SIGUSR2 = 12;
        const SIGPIPE = 13;
        const SIGALRM = 14;
        const SIGTERM = 15;
        const SIGSTKFLT = 16;
        const SIGCHLD = 17;
        const SIGCONT = 18;
        const SIGSTOP = 19;
        const SIGTSTP = 20;
        const SIGTTIN = 21;
        const SIGTTOU = 22;
        const SIGURG = 23;
        const SIGXCPU = 24;
        const SIGXFSZ = 25;
        const SIGVTALRM = 26;
        const SIGPROF = 27;
        const SIGWINCH = 28;
        const SIGIO = 29;
        const SIGPWR = 30;
        const SIGSYS = 31;
    }
}
