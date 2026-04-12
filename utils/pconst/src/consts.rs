use alloc::string::ToString;
use core::fmt::Display;

use int_enum::IntEnum;
use pod::Pod;

#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum LinuxErrno {
    EPERM = -1,
    ENOENT = -2,
    ESRCH = -3,
    EINTR = -4,
    EIO = -5,
    ENXIO = -6,
    E2BIG = -7,
    ENOEXEC = -8,
    EBADF = -9,
    ECHILD = -10,
    EAGAIN = -11,
    ENOMEM = -12,
    EACCES = -13,
    EFAULT = -14,
    ENOTBLK = -15,
    EBUSY = -16,
    EEXIST = -17,
    EXDEV = -18,
    ENODEV = -19,
    ENOTDIR = -20,
    EISDIR = -21,
    EINVAL = -22,
    ENFILE = -23,
    EMFILE = -24,
    ENOTTY = -25,
    ETXTBSY = -26,
    EFBIG = -27,
    ENOSPC = -28,
    ESPIPE = -29,
    EROFS = -30,
    EMLINK = -31,
    EPIPE = -32,
    EDOM = -33,
    ERANGE = -34,
    ENOSYS = -38,
    ELOOP = -40,
    EADDRINUSE = -98,
    /// 协议不被支持 Protocol not supported.
    EPROTONOSUPPORT = -92,
    EOPNOTSUPP = -94,
    EPFNOSUPPORT = -96,
    /// 不支持的地址
    EAFNOSUPPORT = -97,
    EADDRNOTAVAIL = -99,
    ENETDOWN = -100,
    ENETUNREACH = -101,
    ENETRESET = -102,
    ECONNABORTED = -103,
    ECONNRESET = -104,
    ENOBUFS = -105,
    EISCONN = -106,
    ENOTCONN = -107,
    /// 操作正在处理 Operation in progress.
    EINPROGRESS = -115,
    /// 拒绝连接
    ECONNREFUSED = -111,
    /// Address already in use
    EALREADY = -114,
    #[cfg(feature = "special_error")]
    DOMAINCRASH = -255,
    #[cfg(feature = "special_error")]
    EBLOCKING = -256,
}

impl Display for LinuxErrno {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = match self {
            LinuxErrno::EPERM => "Operation not permitted".to_string(),
            LinuxErrno::ENOENT => "No such file or directory".to_string(),
            LinuxErrno::ESRCH => "No such process".to_string(),
            LinuxErrno::EINTR => "Interrupted system call".to_string(),
            LinuxErrno::EIO => "I/O error".to_string(),
            LinuxErrno::ENXIO => "No such device or address".to_string(),
            LinuxErrno::E2BIG => "Argument list too long".to_string(),
            LinuxErrno::ENOEXEC => "Exec format error".to_string(),
            LinuxErrno::EBADF => "Bad file number".to_string(),
            LinuxErrno::ECHILD => "No child processes".to_string(),
            LinuxErrno::EAGAIN => "Try again".to_string(),
            LinuxErrno::ENOMEM => "Out of memory".to_string(),
            LinuxErrno::EACCES => "Permission denied".to_string(),
            LinuxErrno::EFAULT => "Bad address".to_string(),
            LinuxErrno::ENOTBLK => "Block device required".to_string(),
            LinuxErrno::EBUSY => "Device or resource busy".to_string(),
            LinuxErrno::EEXIST => "File exists".to_string(),
            LinuxErrno::EXDEV => "Cross-device link".to_string(),
            LinuxErrno::ENODEV => "No such device".to_string(),
            LinuxErrno::ENOTDIR => "Not a directory".to_string(),
            LinuxErrno::EISDIR => "Is a directory".to_string(),
            LinuxErrno::EINVAL => "Invalid argument".to_string(),
            LinuxErrno::ENFILE => "File table overflow".to_string(),
            LinuxErrno::EMFILE => "Too many open files".to_string(),
            LinuxErrno::ENOTTY => "Not a typewriter".to_string(),
            LinuxErrno::ETXTBSY => "Text file busy".to_string(),
            LinuxErrno::EFBIG => "File too large".to_string(),
            LinuxErrno::ENOSPC => "No space left on device".to_string(),
            LinuxErrno::ESPIPE => "Illegal seek".to_string(),
            LinuxErrno::EROFS => "Read-only file system".to_string(),
            LinuxErrno::EMLINK => "Too many links".to_string(),
            LinuxErrno::EPIPE => "Broken pipe".to_string(),
            LinuxErrno::EDOM => "Math argument out of domain of func".to_string(),
            LinuxErrno::ERANGE => "Math result not representable".to_string(),
            LinuxErrno::ENOSYS => "Function not implemented".to_string(),
            LinuxErrno::ELOOP => "Too many symbolic links encountered".to_string(),
            LinuxErrno::EADDRINUSE => "Address already in use".to_string(),
            LinuxErrno::EPROTONOSUPPORT => "Protocol not supported".to_string(),
            LinuxErrno::EOPNOTSUPP => "Operation not supported on transport endpoint".to_string(),
            LinuxErrno::EPFNOSUPPORT => "Protocol family not supported".to_string(),
            LinuxErrno::EAFNOSUPPORT => "Address family not supported by protocol".to_string(),
            LinuxErrno::EADDRNOTAVAIL => "Cannot assign requested address".to_string(),
            LinuxErrno::ENETDOWN => "Network is down".to_string(),
            LinuxErrno::ENETUNREACH => "Network is unreachable".to_string(),
            LinuxErrno::ENETRESET => "Network dropped connection because of reset".to_string(),
            LinuxErrno::ECONNABORTED => "Software caused connection abort".to_string(),
            LinuxErrno::ECONNRESET => "Connection reset by peer".to_string(),
            LinuxErrno::ENOBUFS => "No buffer space available".to_string(),
            LinuxErrno::EISCONN => "Transport endpoint is already connected".to_string(),
            LinuxErrno::ENOTCONN => "Transport endpoint is not connected".to_string(),
            LinuxErrno::EINPROGRESS => "Connection already in progress".to_string(),
            LinuxErrno::ECONNREFUSED => "Connection refused".to_string(),
            #[cfg(feature = "special_error")]
            LinuxErrno::DOMAINCRASH => "Domain crash".to_string(),
            #[cfg(feature = "special_error")]
            LinuxErrno::EBLOCKING => "Blocking".to_string(),
            LinuxErrno::EALREADY => "Port already in use".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[cfg(feature = "trick")]
impl syscall_table::ToIsize for LinuxErrno {
    fn to_isize(self) -> isize {
        self as isize
    }
}

pub const GRND_NONBLOCK: usize = 0x0001;
pub const GRND_RANDOM: usize = 0x0002;
pub const PPOLL_FROM_POLL_SIGMASK: usize = usize::MAX;
pub const AT_FDCWD: isize = -100isize;
pub const SYSCALL_LIST: usize = 1000;
pub const SYSCALL_CREATE_GLOBAL_BUCKET: usize = 1001;
pub const SYSCALL_EXECUTE_USER_FUNC: usize = 1002;
pub const SYSCALL_SHOW_DBFS: usize = 1003;
pub const SYSCALL_EXECUTE_OPERATE: usize = 1004;
pub const SYSCALL_LOAD_DOMAIN: usize = 888;
pub const SYSCALL_REPLACE_DOMAIN: usize = 889;
pub const SYSCALL_FRAMEBUFFER: usize = 2000;
pub const SYSCALL_FRAMEBUFFER_FLUSH: usize = 2001;
pub const SYSCALL_EVENT_GET: usize = 2002;
pub const SYSCALL_DOMAIN_TEST: usize = 2003;
pub const SYSCALL_SYSTEM_SHUTDOWN: usize = 2004;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod)]
pub struct RLimit64 {
    /// 软上限
    pub rlim_cur: u64,
    /// 硬上限
    pub rlim_max: u64,
}

impl RLimit64 {
    pub fn new(cur: u64, max: u64) -> Self {
        Self {
            rlim_cur: cur,
            rlim_max: max,
        }
    }
}

impl Default for RLimit64 {
    fn default() -> Self {
        Self {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        }
    }
}

#[repr(usize)]
#[derive(Eq, PartialEq, Debug, Copy, Clone, IntEnum)]
pub enum PrLimitResType {
    RlimitStack = 3,
    RlimitNofile = 7,
    RlimitAs = 9,
}

pub fn syscall_name(id: usize) -> &'static str {
    match id {
        SYSCALL_SYSTEM_SHUTDOWN => "system_shutdown",
        SYSCALL_DOMAIN_TEST => "domain_test",
        SYSCALL_EVENT_GET => "event_get",
        SYSCALL_FRAMEBUFFER_FLUSH => "framebuffer_flush",
        SYSCALL_FRAMEBUFFER => "framebuffer",
        SYSCALL_EXECUTE_OPERATE => "execute_operate",
        SYSCALL_SHOW_DBFS => "show_dbfs",
        SYSCALL_EXECUTE_USER_FUNC => "execute_user_func",
        SYSCALL_CREATE_GLOBAL_BUCKET => "create_global_bucket",
        SYSCALL_LIST => "list",
        SYSCALL_REPLACE_DOMAIN => "replace_domain",
        SYSCALL_LOAD_DOMAIN => "load_domain",
        _ => "unknown",
    }
}
