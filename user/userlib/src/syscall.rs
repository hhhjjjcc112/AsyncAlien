use crate::{arch, syscall};

#[cfg(target_arch = "x86_64")]
mod nr {
    pub const SYSCALL_READ: usize = 0;
    pub const SYSCALL_WRITE: usize = 1;
    pub const SYSCALL_OPENAT: usize = 257;
    pub const SYSCALL_CLOSE: usize = 3;
    pub const SYSCALL_GETCWD: usize = 79;
    pub const SYSCALL_CHDIR: usize = 80;
    pub const SYSCALL_MKDIR: usize = 83;
    pub const SYSCALL_LINKAT: usize = 265;
    pub const SYSCALL_UNLINKAT: usize = 263;
    pub const SYSCALL_SYMLINKAT: usize = 266;
    pub const SYSCALL_READLINKAT: usize = 267;
    pub const SYSCALL_FSTATAT: usize = 262;
    pub const SYSCALL_FSTATFS: usize = 138;
    pub const SYSCALL_STATFS: usize = 137;
    pub const SYSCALL_MKDIRAT: usize = 258;
    pub const SYSCALL_RENAMEAT: usize = 264;
    pub const SYSCALL_LSEEK: usize = 8;
    pub const SYSCALL_FSTAT: usize = 5;
    pub const SYSCALL_GETDENTS: usize = 217;
    pub const SYSCALL_SETXATTR: usize = 188;
    pub const SYSCALL_LSETXATTR: usize = 189;
    pub const SYSCALL_FSETXATTR: usize = 190;
    pub const SYSCALL_GETXATTR: usize = 191;
    pub const SYSCALL_LGETXATTR: usize = 192;
    pub const SYSCALL_FGETXATTR: usize = 193;
    pub const SYSCALL_LISTXATTR: usize = 194;
    pub const SYSCALL_LLISTXATTR: usize = 195;
    pub const SYSCALL_FLISTXATTR: usize = 196;
    pub const SYSCALL_REMOVEXATTR: usize = 197;
    pub const SYSCALL_LREMOVEXATTR: usize = 198;
    pub const SYSCALL_FREMOVEXATTR: usize = 199;
    pub const SYSCALL_TRUNCATE: usize = 76;
    pub const SYSCALL_FTRUNCATE: usize = 77;
    pub const SYSCALL_PIPE: usize = 293;
    pub const SYSCALL_DUP: usize = 32;
    pub const SYSCALL_DUP3: usize = 292;
    pub const SYSCALL_BRK: usize = 12;
    pub const SYSCALL_MMAP: usize = 9;
    pub const SYSCALL_MUNMAP: usize = 11;
    pub const SYSCALL_EXIT: usize = 60;
    pub const SYSCALL_YIELD: usize = 24;
    pub const SYSCALL_GET_TIME: usize = 96;
    pub const SYSCALL_GETPID: usize = 39;
    pub const SYSCALL_GETTID: usize = 186;
    pub const SYSCALL_FORK: usize = 57;
    pub const SYSCALL_EXEC: usize = 59;
    pub const SYSCALL_WAITPID: usize = 61;
    pub const SYSCALL_WAITID: usize = 247;
    pub const SYSCALL_SOCKET: usize = 41;
    pub const SYSCALL_SOCKET_PAIR: usize = 53;
    pub const SYSCALL_BIND: usize = 49;
    pub const SYSCALL_LISTEN: usize = 50;
    pub const SYSCALL_ACCEPT: usize = 43;
    pub const SYSCALL_CONNECT: usize = 42;
    pub const SYSCALL_GET_SOCKNAME: usize = 51;
    pub const SYSCALL_GET_PEERNAME: usize = 52;
    pub const SYSCALL_SENDTO: usize = 44;
    pub const SYSCALL_RECVFROM: usize = 45;
    pub const SYSCALL_SET_SOCKOPT: usize = 54;
    pub const SYSCALL_GET_SOCKOPT: usize = 55;
    pub const SYSCALL_SHUTDOWN: usize = 48;
    pub const SYSCALL_MOUNT: usize = 165;
    pub const SYSCALL_NANO_SLEEP: usize = 35;
}

#[cfg(not(target_arch = "x86_64"))]
mod nr {
    pub const SYSCALL_READ: usize = pconst::SYSCALL_READ;
    pub const SYSCALL_WRITE: usize = pconst::SYSCALL_WRITE;
    pub const SYSCALL_OPENAT: usize = pconst::SYSCALL_OPENAT;
    pub const SYSCALL_CLOSE: usize = pconst::SYSCALL_CLOSE;
    pub const SYSCALL_GETCWD: usize = pconst::SYSCALL_GETCWD;
    pub const SYSCALL_CHDIR: usize = pconst::SYSCALL_CHDIR;
    pub const SYSCALL_MKDIR: usize = 83;
    pub const SYSCALL_LINKAT: usize = pconst::SYSCALL_LINKAT;
    pub const SYSCALL_UNLINKAT: usize = pconst::SYSCALL_UNLINKAT;
    pub const SYSCALL_SYMLINKAT: usize = 36;
    pub const SYSCALL_READLINKAT: usize = pconst::SYSCALL_READLINKAT;
    pub const SYSCALL_FSTATAT: usize = pconst::SYSCALL_FSTATAT;
    pub const SYSCALL_FSTATFS: usize = 44;
    pub const SYSCALL_STATFS: usize = pconst::SYSCALL_STATFS;
    pub const SYSCALL_MKDIRAT: usize = pconst::SYSCALL_MKDIRAT;
    pub const SYSCALL_RENAMEAT: usize = 38;
    pub const SYSCALL_LSEEK: usize = pconst::SYSCALL_LSEEK;
    pub const SYSCALL_FSTAT: usize = pconst::SYSCALL_FSTAT;
    pub const SYSCALL_GETDENTS: usize = pconst::SYSCALL_GETDENTS64;
    pub const SYSCALL_SETXATTR: usize = 5;
    pub const SYSCALL_LSETXATTR: usize = 6;
    pub const SYSCALL_FSETXATTR: usize = 7;
    pub const SYSCALL_GETXATTR: usize = 8;
    pub const SYSCALL_LGETXATTR: usize = 9;
    pub const SYSCALL_FGETXATTR: usize = 10;
    pub const SYSCALL_LISTXATTR: usize = 11;
    pub const SYSCALL_LLISTXATTR: usize = 12;
    pub const SYSCALL_FLISTXATTR: usize = 13;
    pub const SYSCALL_REMOVEXATTR: usize = 14;
    pub const SYSCALL_LREMOVEXATTR: usize = 15;
    pub const SYSCALL_FREMOVEXATTR: usize = 16;
    pub const SYSCALL_TRUNCATE: usize = 45;
    pub const SYSCALL_FTRUNCATE: usize = pconst::SYSCALL_FTRUNCATE;
    pub const SYSCALL_PIPE: usize = pconst::SYSCALL_PIPE2;
    pub const SYSCALL_DUP: usize = pconst::SYSCALL_DUP;
    pub const SYSCALL_DUP3: usize = pconst::SYSCALL_DUP3;
    pub const SYSCALL_BRK: usize = pconst::SYSCALL_BRK;
    pub const SYSCALL_MMAP: usize = pconst::SYSCALL_MMAP;
    pub const SYSCALL_MUNMAP: usize = pconst::SYSCALL_MUNMAP;
    pub const SYSCALL_EXIT: usize = pconst::SYSCALL_EXIT;
    pub const SYSCALL_YIELD: usize = pconst::SYSCALL_YIELD;
    pub const SYSCALL_GET_TIME: usize = pconst::SYSCALL_GET_TIME_OF_DAY;
    pub const SYSCALL_GETPID: usize = pconst::SYSCALL_GETPID;
    pub const SYSCALL_GETTID: usize = pconst::SYSCALL_GETTID;
    pub const SYSCALL_FORK: usize = pconst::SYSCALL_CLONE;
    pub const SYSCALL_EXEC: usize = pconst::SYSCALL_EXECVE;
    pub const SYSCALL_WAITPID: usize = pconst::SYSCALL_WAIT4;
    pub const SYSCALL_WAITID: usize = 95;
    pub const SYSCALL_SOCKET: usize = pconst::SYSCALL_SOCKET;
    pub const SYSCALL_SOCKET_PAIR: usize = pconst::SYSCALL_SOCKETPAIR;
    pub const SYSCALL_BIND: usize = pconst::SYSCALL_BIND;
    pub const SYSCALL_LISTEN: usize = pconst::SYSCALL_LISTEN;
    pub const SYSCALL_ACCEPT: usize = pconst::SYSCALL_ACCEPT;
    pub const SYSCALL_CONNECT: usize = pconst::SYSCALL_CONNECT;
    pub const SYSCALL_GET_SOCKNAME: usize = pconst::SYSCALL_GETSOCKNAME;
    pub const SYSCALL_GET_PEERNAME: usize = pconst::SYSCALL_GETPEERNAME;
    pub const SYSCALL_SENDTO: usize = pconst::SYSCALL_SENDTO;
    pub const SYSCALL_RECVFROM: usize = pconst::SYSCALL_RECVFROM;
    pub const SYSCALL_SET_SOCKOPT: usize = pconst::SYSCALL_SETSOCKOPT;
    pub const SYSCALL_GET_SOCKOPT: usize = pconst::SYSCALL_GETSOCKOPT;
    pub const SYSCALL_SHUTDOWN: usize = pconst::SYSCALL_SHUTDOWN;
    pub const SYSCALL_MOUNT: usize = pconst::SYSCALL_MOUNT;
    pub const SYSCALL_NANO_SLEEP: usize = pconst::SYSCALL_NANOSLEEP;
}

// Alien 私有扩展号段。
const SYSCALL_LIST: usize = 1000;
const SYSCALL_CREATE_GLOBAL_BUCKET: usize = 1001;
const SYSCALL_EXECUTE_USER_FUNC: usize = 1002;
const SYSCALL_SHOW_DBFS: usize = 1003;
const SYSCALL_EXECUTE_OPERATE: usize = 1004;
const SYSCALL_FRAME_BUFFER: usize = 2000;
const SYSCALL_FRAME_FLUSH: usize = 2001;
const SYSCALL_EVENT: usize = 2002;
const SYSCALL_SYSTEMSHUTDOWN: usize = 2003;
fn syscall(id: usize, args: [usize; 6]) -> isize {
    arch::syscall(id, args)
}

syscall!(sys_read, nr::SYSCALL_READ, usize, *mut u8, usize);
syscall!(sys_write, nr::SYSCALL_WRITE, usize, *const u8, usize);
syscall!(sys_exit, nr::SYSCALL_EXIT, i32);
syscall!(sys_yield, nr::SYSCALL_YIELD);
syscall!(sys_getpid, nr::SYSCALL_GETPID);
syscall!(sys_gettid, nr::SYSCALL_GETTID);
syscall!(sys_get_time, nr::SYSCALL_GET_TIME, *mut u8);
syscall!(sys_fork, nr::SYSCALL_FORK);
syscall!(
    sys_execve,
    nr::SYSCALL_EXEC,
    *const u8,
    *const usize,
    *const usize
);
syscall!(sys_waitpid, nr::SYSCALL_WAITPID, isize, *mut i32, u32);
syscall!(sys_waitid, nr::SYSCALL_WAITID, usize, usize, *mut u8, usize, usize);

// virtio-mmio-net
syscall!(sys_socket, nr::SYSCALL_SOCKET, usize, usize, usize);
syscall!(
    sys_socket_pair,
    nr::SYSCALL_SOCKET_PAIR,
    usize,
    usize,
    usize,
    *const usize
);
syscall!(sys_bind, nr::SYSCALL_BIND, usize, *const usize, usize);
syscall!(sys_listen, nr::SYSCALL_LISTEN, usize, usize);
syscall!(sys_accept, nr::SYSCALL_ACCEPT, usize, *const usize, *mut usize);
syscall!(sys_connect, nr::SYSCALL_CONNECT, usize, *const usize, usize);
syscall!(
    sys_getsockname,
    nr::SYSCALL_GET_SOCKNAME,
    usize,
    *mut usize,
    *mut usize
);
syscall!(
    sys_getpeername,
    nr::SYSCALL_GET_PEERNAME,
    usize,
    *mut usize,
    *mut usize
);

syscall!(
    sys_sendto,
    nr::SYSCALL_SENDTO,
    usize,
    *const u8,
    usize,
    usize,
    *const usize,
    usize
);
syscall!(
    sys_recvfrom,
    nr::SYSCALL_RECVFROM,
    usize,
    *mut u8,
    usize,
    usize,
    *mut usize,
    *mut usize
);
syscall!(sys_setsockopt, nr::SYSCALL_SET_SOCKOPT);
syscall!(sys_getsockopt, nr::SYSCALL_GET_SOCKOPT);
syscall!(sys_shutdown, nr::SYSCALL_SHUTDOWN, usize, usize);

syscall!(sys_list, SYSCALL_LIST, *const u8);
syscall!(sys_openat, nr::SYSCALL_OPENAT, isize, *const u8, usize, usize);
syscall!(sys_close, nr::SYSCALL_CLOSE, usize);
syscall!(sys_get_cwd, nr::SYSCALL_GETCWD, *mut u8, usize);
syscall!(sys_chdir, nr::SYSCALL_CHDIR, *const u8);
syscall!(sys_mkdir, nr::SYSCALL_MKDIR, *const u8);
syscall!(sys_nanosleep, nr::SYSCALL_NANO_SLEEP, *mut u8, *mut u8);

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
    nr::SYSCALL_MOUNT,
    *const u8,
    *const u8,
    *const u8,
    usize,
    *const u8
);
syscall!(sys_lseek, nr::SYSCALL_LSEEK, usize, isize, usize);
syscall!(sys_fstat, nr::SYSCALL_FSTAT, usize, *mut u8);
syscall!(
    sys_linkat,
    nr::SYSCALL_LINKAT,
    isize,
    *const u8,
    usize,
    *const u8,
    usize
);
syscall!(sys_unlinkat, nr::SYSCALL_UNLINKAT, isize, *const u8, usize);
syscall!(
    sys_symlinkat,
    nr::SYSCALL_SYMLINKAT,
    *const u8,
    isize,
    *const u8
);
syscall!(
    sys_readlinkat,
    nr::SYSCALL_READLINKAT,
    isize,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_fstatat,
    nr::SYSCALL_FSTATAT,
    isize,
    *const u8,
    *mut u8,
    usize
);
syscall!(sys_fstatfs, nr::SYSCALL_FSTATFS, usize, *mut u8);
syscall!(sys_statfs, nr::SYSCALL_STATFS, *const u8, *mut u8);
syscall!(sys_mkdirat, nr::SYSCALL_MKDIRAT, isize, *const u8, usize);
syscall!(
    sys_renameat,
    nr::SYSCALL_RENAMEAT,
    isize,
    *const u8,
    isize,
    *const u8
);

syscall!(
    sys_setxattr,
    nr::SYSCALL_SETXATTR,
    *const u8,
    *const u8,
    *const u8,
    usize,
    usize
);
syscall!(
    sys_lsetxattr,
    nr::SYSCALL_LSETXATTR,
    *const u8,
    *const u8,
    *const u8,
    usize,
    usize
);
syscall!(
    sys_fsetxattr,
    nr::SYSCALL_FSETXATTR,
    usize,
    *const u8,
    *const u8,
    usize,
    usize
);

syscall!(
    sys_getxattr,
    nr::SYSCALL_GETXATTR,
    *const u8,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_lgetxattr,
    nr::SYSCALL_LGETXATTR,
    *const u8,
    *const u8,
    *mut u8,
    usize
);
syscall!(
    sys_fgetxattr,
    nr::SYSCALL_FGETXATTR,
    usize,
    *const u8,
    *mut u8,
    usize
);

syscall!(sys_listxattr, nr::SYSCALL_LISTXATTR, *const u8, *mut u8, usize);
syscall!(
    sys_llistxattr,
    nr::SYSCALL_LLISTXATTR,
    *const u8,
    *mut u8,
    usize
);
syscall!(sys_flistxattr, nr::SYSCALL_FLISTXATTR, usize, *mut u8, usize);

syscall!(sys_removexattr, nr::SYSCALL_REMOVEXATTR, *const u8, *const u8);
syscall!(sys_lremovexattr, nr::SYSCALL_LREMOVEXATTR, *const u8, *const u8);
syscall!(sys_fremovexattr, nr::SYSCALL_FREMOVEXATTR, usize, *const u8);
syscall!(sys_getdents, nr::SYSCALL_GETDENTS, usize, *mut u8, usize);

syscall!(sys_truncate, nr::SYSCALL_TRUNCATE, *const u8, usize);
syscall!(sys_ftruncate, nr::SYSCALL_FTRUNCATE, usize, usize);

// ipc
syscall!(sys_pipe, nr::SYSCALL_PIPE, *mut u32, usize);
syscall!(sys_dup, nr::SYSCALL_DUP, usize);
syscall!(sys_dup3, nr::SYSCALL_DUP3, usize, usize, usize);

// alloc
syscall!(sys_brk, nr::SYSCALL_BRK, usize);

// memory
syscall!(
    sys_mmap,
    nr::SYSCALL_MMAP,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize
);
syscall!(sys_munmap, nr::SYSCALL_MUNMAP, usize, usize);
syscall!(sys_setpriority, 140, i32, u32, i32);
syscall!(sys_getpriority, 141, i32, u32);

// gui
syscall!(sys_framebuffer, SYSCALL_FRAME_BUFFER);
syscall!(sys_framebuffer_flush, SYSCALL_FRAME_FLUSH);
syscall!(sys_event, SYSCALL_EVENT, *mut u64, usize);
syscall!(__system_shutdown, SYSCALL_SYSTEMSHUTDOWN);

syscall!(sys_register_domain, 888, usize, u8, *const u8, usize);
syscall!(
    sys_update_domain,
    889,
    *const u8,
    usize,
    *const u8,
    usize,
    u8
);

syscall!(sys_out_mask, 2003);
