use pconst::{io::PollFd, time::TimeSpec, *};

use super::syscall;
use crate::syscall;

// 两个架构都存在的标准 Linux syscall。
syscall!(sys_read, SYSCALL_READ, usize, *mut u8, usize);
syscall!(sys_write, SYSCALL_WRITE, usize, *const u8, usize);
syscall!(sys_exit, SYSCALL_EXIT, i32);
syscall!(sys_sched_yield, SYSCALL_SCHED_YIELD);
syscall!(sys_getpid, SYSCALL_GETPID);
syscall!(sys_gettid, SYSCALL_GETTID);
syscall!(sys_setpgid, SYSCALL_SETPGID, usize, usize);
syscall!(sys_getpgid, SYSCALL_GETPGID, usize);
syscall!(sys_getsid, SYSCALL_GETSID, usize);
syscall!(sys_setsid, SYSCALL_SETSID);
syscall!(sys_gettimeofday, SYSCALL_GETTIMEOFDAY, *mut u8, *mut u8);
syscall!(sys_clock_gettime, SYSCALL_CLOCK_GETTIME, usize, *mut u8);
syscall!(sys_getrandom, SYSCALL_GETRANDOM, *mut u8, usize, usize);
syscall!(sys_execve, SYSCALL_EXECVE, *const u8, *const usize, *const usize);
syscall!(sys_wait4, SYSCALL_WAIT4, isize, *mut i32, u32, usize);
syscall!(sys_waitid, SYSCALL_WAITID, usize, usize, *mut u8, usize, usize);

syscall!(sys_socket, SYSCALL_SOCKET, usize, usize, usize);
syscall!(sys_socketpair, SYSCALL_SOCKETPAIR, usize, usize, usize, *mut usize);
syscall!(sys_bind, SYSCALL_BIND, usize, *const usize, usize);
syscall!(sys_listen, SYSCALL_LISTEN, usize, usize);
syscall!(sys_accept, SYSCALL_ACCEPT, usize, *mut usize, *mut usize);
syscall!(sys_connect, SYSCALL_CONNECT, usize, *const usize, usize);
syscall!(sys_getsockname, SYSCALL_GETSOCKNAME, usize, *mut usize, *mut usize);
syscall!(sys_getpeername, SYSCALL_GETPEERNAME, usize, *mut usize, *mut usize);
syscall!(sys_sendto, SYSCALL_SENDTO, usize, *const u8, usize, usize, *const usize, usize);
syscall!(sys_recvfrom, SYSCALL_RECVFROM, usize, *mut u8, usize, usize, *mut usize, *mut usize);
syscall!(sys_setsockopt, SYSCALL_SETSOCKOPT, usize, usize, usize, *const u8, usize);
syscall!(sys_getsockopt, SYSCALL_GETSOCKOPT, usize, usize, usize, *mut u8, *mut usize);
syscall!(sys_shutdown, SYSCALL_SHUTDOWN, usize, usize);

syscall!(sys_openat, SYSCALL_OPENAT, isize, *const u8, usize, usize);
syscall!(sys_close, SYSCALL_CLOSE, usize);
syscall!(sys_getcwd, SYSCALL_GETCWD, *mut u8, usize);
syscall!(sys_chdir, SYSCALL_CHDIR, *const u8);
syscall!(sys_mount, SYSCALL_MOUNT, *const u8, *const u8, *const u8, usize, *const u8);
syscall!(sys_lseek, SYSCALL_LSEEK, usize, isize, usize);
syscall!(sys_fstat, SYSCALL_FSTAT, usize, *mut u8);
syscall!(sys_linkat, SYSCALL_LINKAT, isize, *const u8, usize, *const u8, usize);
syscall!(sys_unlinkat, SYSCALL_UNLINKAT, isize, *const u8, usize);
syscall!(sys_symlinkat, SYSCALL_SYMLINKAT, *const u8, isize, *const u8);
syscall!(sys_readlinkat, SYSCALL_READLINKAT, isize, *const u8, *mut u8, usize);
syscall!(sys_newfstatat, SYSCALL_NEWFSTATAT, isize, *const u8, *mut u8, usize);
syscall!(sys_fstatfs, SYSCALL_FSTATFS, usize, *mut u8);
syscall!(sys_statfs, SYSCALL_STATFS, *const u8, *mut u8);
syscall!(sys_mkdirat, SYSCALL_MKDIRAT, isize, *const u8, usize);
syscall!(sys_renameat2, SYSCALL_RENAMEAT2, isize, *const u8, isize, *const u8, usize);

syscall!(sys_setxattr, SYSCALL_SETXATTR, *const u8, *const u8, *const u8, usize, usize);
syscall!(sys_lsetxattr, SYSCALL_LSETXATTR, *const u8, *const u8, *const u8, usize, usize);
syscall!(sys_fsetxattr, SYSCALL_FSETXATTR, usize, *const u8, *const u8, usize, usize);
syscall!(sys_getxattr, SYSCALL_GETXATTR, *const u8, *const u8, *mut u8, usize);
syscall!(sys_lgetxattr, SYSCALL_LGETXATTR, *const u8, *const u8, *mut u8, usize);
syscall!(sys_fgetxattr, SYSCALL_FGETXATTR, usize, *const u8, *mut u8, usize);
syscall!(sys_listxattr, SYSCALL_LISTXATTR, *const u8, *mut u8, usize);
syscall!(sys_llistxattr, SYSCALL_LLISTXATTR, *const u8, *mut u8, usize);
syscall!(sys_flistxattr, SYSCALL_FLISTXATTR, usize, *mut u8, usize);
syscall!(sys_removexattr, SYSCALL_REMOVEXATTR, *const u8, *const u8);
syscall!(sys_lremovexattr, SYSCALL_LREMOVEXATTR, *const u8, *const u8);
syscall!(sys_fremovexattr, SYSCALL_FREMOVEXATTR, usize, *const u8);
syscall!(sys_getdents64, SYSCALL_GETDENTS64, usize, *mut u8, usize);
syscall!(sys_truncate, SYSCALL_TRUNCATE, *const u8, usize);
syscall!(sys_ftruncate, SYSCALL_FTRUNCATE, usize, usize);
syscall!(sys_pipe2, SYSCALL_PIPE2, *mut u32, usize);
syscall!(sys_dup, SYSCALL_DUP, usize);
syscall!(sys_dup3, SYSCALL_DUP3, usize, usize, usize);
syscall!(sys_fcntl, SYSCALL_FCNTL, usize, usize, usize);
syscall!(sys_ioctl, SYSCALL_IOCTL, usize, usize, usize);
syscall!(sys_pread64, SYSCALL_PREAD64, usize, *mut u8, usize, usize);
syscall!(sys_pwrite64, SYSCALL_PWRITE64, usize, *const u8, usize, usize);
syscall!(sys_readv, SYSCALL_READV, usize, usize, usize);
syscall!(sys_writev, SYSCALL_WRITEV, usize, usize, usize);
syscall!(sys_pselect6, SYSCALL_PSELECT6, usize, usize, usize, usize, usize, usize);
syscall!(sys_ppoll, SYSCALL_PPOLL, *mut PollFd, usize, *const TimeSpec, *const usize, usize);
syscall!(sys_fchdir, SYSCALL_FCHDIR, usize);
syscall!(sys_getrlimit, SYSCALL_GETRLIMIT, usize, usize);
syscall!(sys_setrlimit, SYSCALL_SETRLIMIT, usize, usize);
syscall!(sys_getrusage, SYSCALL_GETRUSAGE, usize, usize);
syscall!(sys_fsync, SYSCALL_FSYNC, usize);
syscall!(sys_utimensat, SYSCALL_UTIMENSAT, isize, *const u8, *const u8, usize);
syscall!(sys_set_tid_address, SYSCALL_SET_TID_ADDRESS, usize);
syscall!(sys_futex, SYSCALL_FUTEX, usize, usize, usize, usize, usize, usize);
syscall!(sys_sigaltstack, SYSCALL_SIGALTSTACK, usize, usize);
syscall!(sys_rt_sigaction, SYSCALL_RT_SIGACTION, usize, usize, usize, usize);
syscall!(sys_rt_sigprocmask, SYSCALL_RT_SIGPROCMASK, usize, usize, usize, usize);
syscall!(sys_setpriority, SYSCALL_SETPRIORITY, i32, u32, i32);
syscall!(sys_getpriority, SYSCALL_GETPRIORITY, i32, u32);
syscall!(sys_uname, SYSCALL_UNAME, usize);
syscall!(sys_brk, SYSCALL_BRK, usize);
syscall!(sys_munmap, SYSCALL_MUNMAP, usize, usize);
syscall!(sys_mmap, SYSCALL_MMAP, usize, usize, usize, usize, usize, usize);
syscall!(sys_mprotect, SYSCALL_MPROTECT, usize, usize, usize);
syscall!(sys_prlimit64, SYSCALL_PRLIMIT64, usize, usize, usize, usize);
syscall!(sys_madvise, SYSCALL_MADVISE, usize, usize, usize);
syscall!(sys_umask, SYSCALL_UMASK, usize);
syscall!(sys_eventfd2, SYSCALL_EVENTFD2, usize, usize);
syscall!(sys_epoll_create1, SYSCALL_EPOLL_CREATE1, usize);
syscall!(sys_epoll_ctl, SYSCALL_EPOLL_CTL, usize, usize, usize, usize);
syscall!(sys_nanosleep, SYSCALL_NANOSLEEP, *mut u8, *mut u8);

#[cfg(target_arch = "x86_64")]
pub fn sys_clone(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    tls: usize,
    child_tid: usize,
) -> isize {
    // x86_64 raw clone 需要把 child_tid 放在 tls 前面。
    syscall(SYSCALL_CLONE, [flags, stack, parent_tid, child_tid, tls, 0])
}

#[cfg(target_arch = "riscv64")]
pub fn sys_clone(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    tls: usize,
    child_tid: usize,
) -> isize {
    // riscv64 raw clone 保持 Linux ABI 原始参数顺序。
    syscall(SYSCALL_CLONE, [flags, stack, parent_tid, tls, child_tid, 0])
}
