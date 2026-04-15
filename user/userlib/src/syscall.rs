use crate::{arch, syscall};
use crate::time::TimeSpec;
use pconst::{io::PollFd, signal::SignalNumber, task::CloneFlags, *};
fn syscall(id: usize, args: [usize; 6]) -> isize {
    arch::syscall(id, args)
}

#[cfg(target_arch = "x86_64")]
syscall!(sys_poll, SYSCALL_POLL, *mut PollFd, usize, i32);
#[cfg(target_arch = "riscv64")]
syscall!(
    sys_ppoll,
    SYSCALL_PPOLL,
    *mut PollFd,
    usize,
    *const TimeSpec,
    *const usize,
    usize
);

#[cfg(target_arch = "riscv64")]
pub fn sys_poll(fds: *mut PollFd, nfds: usize, timeout_ms: i32) -> isize {
    if timeout_ms < 0 {
        return sys_ppoll(fds, nfds, core::ptr::null(), core::ptr::null(), 0);
    }

    let timeout = TimeSpec::new((timeout_ms as usize) / 1000, ((timeout_ms as usize) % 1000) * 1_000_000);
    sys_ppoll(fds, nfds, &timeout, core::ptr::null(), 0)
}

syscall!(sys_read, SYSCALL_READ, usize, *mut u8, usize);
syscall!(sys_write, SYSCALL_WRITE, usize, *const u8, usize);
syscall!(sys_exit, SYSCALL_EXIT, i32);
syscall!(sys_yield, SYSCALL_SCHED_YIELD);
syscall!(sys_getpid, SYSCALL_GETPID);
syscall!(sys_gettid, SYSCALL_GETTID);
syscall!(sys_setpgid, SYSCALL_SETPGID, usize, usize);
syscall!(sys_getpgid, SYSCALL_GETPGID, usize);
#[cfg(target_arch = "x86_64")]
syscall!(sys_getpgrp, SYSCALL_GETPGRP);
#[cfg(target_arch = "riscv64")]
pub fn sys_getpgrp() -> isize {
    sys_getpgid(0)
}
syscall!(sys_getsid, SYSCALL_GETSID, usize);
syscall!(sys_setsid, SYSCALL_SETSID);
syscall!(sys_gettimeofday, SYSCALL_GETTIMEOFDAY, *mut u8, *mut u8);
pub fn sys_get_time(tv: *mut u8) -> isize {
    sys_gettimeofday(tv, core::ptr::null_mut())
}
#[cfg(target_arch = "x86_64")]
pub fn sys_clone(flags: usize, stack: usize, parent_tid: usize, tls: usize, child_tid: usize) -> isize {
    syscall(SYSCALL_CLONE, [flags, stack, parent_tid, child_tid, tls, 0])
}

#[cfg(target_arch = "riscv64")]
pub fn sys_clone(flags: usize, stack: usize, parent_tid: usize, tls: usize, child_tid: usize) -> isize {
    syscall(SYSCALL_CLONE, [flags, stack, parent_tid, tls, child_tid, 0])
}

pub fn sys_fork() -> isize {
    sys_clone(SignalNumber::SIGCHLD as usize, 0, 0, 0, 0)
}

pub fn sys_vfork() -> isize {
    sys_clone(
        (CloneFlags::CLONE_VFORK | CloneFlags::CLONE_VM).bits() as usize
            | SignalNumber::SIGCHLD as usize,
        0,
        0,
        0,
        0,
    )
}

syscall!(sys_getrandom, SYSCALL_GETRANDOM, *mut u8, usize, usize);
syscall!(
    sys_execve,
    SYSCALL_EXECVE,
    *const u8,
    *const usize,
    *const usize
);
syscall!(sys_waitpid, SYSCALL_WAIT4, isize, *mut i32, u32);
syscall!(sys_waitid, SYSCALL_WAITID, usize, usize, *mut u8, usize, usize);

// virtio-mmio-net
syscall!(sys_socket, SYSCALL_SOCKET, usize, usize, usize);
syscall!(
    sys_socket_pair,
    SYSCALL_SOCKETPAIR,
    usize,
    usize,
    usize,
    *mut usize
);
syscall!(sys_bind, SYSCALL_BIND, usize, *const usize, usize);
syscall!(sys_listen, SYSCALL_LISTEN, usize, usize);
syscall!(sys_accept, SYSCALL_ACCEPT, usize, *mut usize, *mut usize);
syscall!(sys_connect, SYSCALL_CONNECT, usize, *const usize, usize);
syscall!(
    sys_getsockname,
    SYSCALL_GETSOCKNAME,
    usize,
    *mut usize,
    *mut usize
);
syscall!(
    sys_getpeername,
    SYSCALL_GETPEERNAME,
    usize,
    *mut usize,
    *mut usize
);

syscall!(
    sys_sendto,
    SYSCALL_SENDTO,
    usize,
    *const u8,
    usize,
    usize,
    *const usize,
    usize
);
syscall!(
    sys_recvfrom,
    SYSCALL_RECVFROM,
    usize,
    *mut u8,
    usize,
    usize,
    *mut usize,
    *mut usize
);
syscall!(
    sys_setsockopt,
    SYSCALL_SETSOCKOPT,
    usize,
    usize,
    usize,
    *const u8,
    usize
);
syscall!(
    sys_getsockopt,
    SYSCALL_GETSOCKOPT,
    usize,
    usize,
    usize,
    *mut u8,
    *mut usize
);
syscall!(sys_shutdown, SYSCALL_SHUTDOWN, usize, usize);

syscall!(sys_list, SYSCALL_LIST, *const u8);
syscall!(sys_openat, SYSCALL_OPENAT, isize, *const u8, usize, usize);
syscall!(sys_close, SYSCALL_CLOSE, usize);
syscall!(sys_get_cwd, SYSCALL_GETCWD, *mut u8, usize);
syscall!(sys_chdir, SYSCALL_CHDIR, *const u8);
#[cfg(target_arch = "x86_64")]
pub fn sys_mkdir(path: *const u8, mode: usize) -> isize {
    syscall(SYSCALL_MKDIR, [path as usize, mode, 0, 0, 0, 0])
}

#[cfg(target_arch = "riscv64")]
pub fn sys_mkdir(path: *const u8, mode: usize) -> isize {
    syscall(
        SYSCALL_MKDIRAT,
        [AT_FDCWD as usize, path as usize, mode, 0, 0, 0],
    )
}

syscall!(sys_nanosleep, SYSCALL_NANOSLEEP, *mut u8, *mut u8);

syscall!(
    sys_create_global_bucket,
    SYSCALL_CREATE_GLOBAL_BUCKET,
    *const u8
);
syscall!(
    sys_execute_user_func,
    SYSCALL_EXECUTE_USER_FUNC,
    *const u8,
    *const u8,
    usize,
    usize
);
syscall!(sys_show_dbfs, SYSCALL_SHOW_DBFS);
syscall!(
    sys_dbfs_execute_operate,
    SYSCALL_EXECUTE_OPERATE,
    *const u8,
    *const u8
);
syscall!(
    sys_mount,
    SYSCALL_MOUNT,
    *const u8,
    *const u8,
    *const u8,
    usize,
    *const u8
);
syscall!(sys_lseek, SYSCALL_LSEEK, usize, isize, usize);
syscall!(sys_fstat, SYSCALL_FSTAT, usize, *mut u8);
syscall!(
    sys_linkat,
    SYSCALL_LINKAT,
    isize,
    *const u8,
    usize,
    *const u8,
    usize
);
syscall!(sys_unlinkat, SYSCALL_UNLINKAT, isize, *const u8, usize);
syscall!(
    sys_symlinkat,
    SYSCALL_SYMLINKAT,
    *const u8,
    isize,
    *const u8
);
syscall!(
    sys_readlinkat,
    SYSCALL_READLINKAT,
    isize,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_fstatat,
    SYSCALL_NEWFSTATAT,
    isize,
    *const u8,
    *mut u8,
    usize
);
syscall!(sys_fstatfs, SYSCALL_FSTATFS, usize, *mut u8);
syscall!(sys_statfs, SYSCALL_STATFS, *const u8, *mut u8);
syscall!(sys_mkdirat, SYSCALL_MKDIRAT, isize, *const u8, usize);
syscall!(
    sys_renameat,
    SYSCALL_RENAMEAT,
    isize,
    *const u8,
    isize,
    *const u8
);

syscall!(
    sys_setxattr,
    SYSCALL_SETXATTR,
    *const u8,
    *const u8,
    *const u8,
    usize,
    usize
);
syscall!(
    sys_lsetxattr,
    SYSCALL_LSETXATTR,
    *const u8,
    *const u8,
    *const u8,
    usize,
    usize
);
syscall!(
    sys_fsetxattr,
    SYSCALL_FSETXATTR,
    usize,
    *const u8,
    *const u8,
    usize,
    usize
);

syscall!(
    sys_getxattr,
    SYSCALL_GETXATTR,
    *const u8,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_lgetxattr,
    SYSCALL_LGETXATTR,
    *const u8,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_fgetxattr,
    SYSCALL_FGETXATTR,
    usize,
    *const u8,
    *mut u8,
    usize
);

syscall!(sys_listxattr, SYSCALL_LISTXATTR, *const u8, *mut u8, usize);
syscall!(
    sys_llistxattr,
    SYSCALL_LLISTXATTR,
    *const u8,
    *mut u8,
    usize
);
syscall!(sys_flistxattr, SYSCALL_FLISTXATTR, usize, *mut u8, usize);

syscall!(sys_removexattr, SYSCALL_REMOVEXATTR, *const u8, *const u8);
syscall!(sys_lremovexattr, SYSCALL_LREMOVEXATTR, *const u8, *const u8);
syscall!(sys_fremovexattr, SYSCALL_FREMOVEXATTR, usize, *const u8);
syscall!(sys_getdents, SYSCALL_GETDENTS64, usize, *mut u8, usize);

syscall!(sys_truncate, SYSCALL_TRUNCATE, *const u8, usize);
syscall!(sys_ftruncate, SYSCALL_FTRUNCATE, usize, usize);

// ipc
#[cfg(target_arch = "x86_64")]
syscall!(sys_pipe, SYSCALL_PIPE, *mut u32);
#[cfg(target_arch = "riscv64")]
syscall!(sys_pipe2, SYSCALL_PIPE2, *mut u32, usize);
syscall!(sys_dup, SYSCALL_DUP, usize);
syscall!(sys_dup3, SYSCALL_DUP3, usize, usize, usize);

// alloc
syscall!(sys_brk, SYSCALL_BRK, usize);

// memory
syscall!(
    sys_mmap,
    SYSCALL_MMAP,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize
);
syscall!(sys_munmap, SYSCALL_MUNMAP, usize, usize);
syscall!(sys_setpriority, SYSCALL_SETPRIORITY, i32, u32, i32);
syscall!(sys_getpriority, SYSCALL_GETPRIORITY, i32, u32);

// gui
syscall!(sys_framebuffer, SYSCALL_FRAMEBUFFER);
syscall!(sys_framebuffer_flush, SYSCALL_FRAMEBUFFER_FLUSH);
syscall!(sys_event, SYSCALL_EVENT_GET, *mut u64, usize);
syscall!(__system_shutdown, SYSCALL_SYSTEM_SHUTDOWN);

syscall!(sys_register_domain, SYSCALL_LOAD_DOMAIN, usize, u8, *const u8, usize);
syscall!(
    sys_update_domain,
    SYSCALL_REPLACE_DOMAIN,
    *const u8,
    usize,
    *const u8,
    usize,
    u8
);

syscall!(sys_out_mask, 2003);
