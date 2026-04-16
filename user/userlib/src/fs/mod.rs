use alloc::{
    format,
    string::{String, ToString},
};

pub use attr::*;
use bitflags::bitflags;
use pconst::time::TimeSpec;
pub use pconst::{
    AT_FDCWD,
    io::{
        FaccessatFlags, FaccessatMode, FileMode, FileStat as Stat, FsStat as StatFs, LinkFlags,
        OpenFlags, PollEvents, PollFd, StatFlags, UnlinkatFlags,
    },
};

use crate::syscall::*;

mod attr;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StatTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
bitflags! {
    #[derive(Default)]
     pub struct InodeMode:u32{
        const S_SYMLINK = 0120000;
        const S_DIR = 0040000;
        const S_FILE = 0100000;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Dirent64 {
    /// ino is an inode number
    pub ino: u64,
    /// off is an offset to next linux_dirent
    pub off: i64,
    /// reclen is the length of this linux_dirent
    pub reclen: u16,
    /// type is the file type
    pub type_: DirentType,
    /// name is the filename (null-terminated)
    pub name: [u8; 0],
}

impl Dirent64 {
    pub fn get_name(&self) -> &str {
        unsafe {
            let name = self.name.as_ptr();
            let name = core::ffi::CStr::from_ptr(name as _);
            name.to_str().unwrap()
        }
    }
    pub fn len(&self) -> usize {
        self.reclen as usize
    }
}

bitflags! {
    pub struct DirentType:u8{
        const DT_UNKNOWN = 0;
        const DT_FIFO = 1;
        const DT_CHR = 2;
        const DT_DIR = 4;
        const DT_BLK = 6;
        const DT_REG = 8;
        const DT_LNK = 10;
        const DT_SOCK = 12;
        const DT_WHT = 14;
    }
}

impl ToString for DirentType {
    fn to_string(&self) -> String {
        match *self {
            DirentType::DT_UNKNOWN => "unknown".to_string(),
            DirentType::DT_FIFO => "fifo".to_string(),
            DirentType::DT_CHR => "char".to_string(),
            DirentType::DT_DIR => "dir".to_string(),
            DirentType::DT_BLK => "block".to_string(),
            DirentType::DT_REG => "regular".to_string(),
            DirentType::DT_LNK => "link".to_string(),
            DirentType::DT_SOCK => "sock".to_string(),
            DirentType::DT_WHT => "whiteout".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf.as_mut_ptr(), buf.len())
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf.as_ptr(), buf.len())
}

pub fn readdir(fd: usize, buf: &mut [u8]) -> isize {
    sys_getdents64(fd, buf.as_mut_ptr(), buf.len())
}

pub fn poll(fds: &mut [PollFd], timeout_ms: i32) -> isize {
    // 统一走公共 ppoll；负超时表示无限等待。
    if timeout_ms < 0 {
        return sys_ppoll(
            fds.as_mut_ptr(),
            fds.len(),
            core::ptr::null(),
            core::ptr::null(),
            0,
        );
    }
    let timeout = TimeSpec::new(
        (timeout_ms as usize) / 1000,
        ((timeout_ms as usize) % 1000) * 1_000_000,
    );
    sys_ppoll(fds.as_mut_ptr(), fds.len(), &timeout, core::ptr::null(), 0)
}

pub fn list(path: &str) -> isize {
    if !path.ends_with('\0') {
        let mut p = String::from(path);
        p.push('\0');
        return sys_list(p.as_ptr());
    }
    sys_list(path.as_ptr())
}

pub fn open(name: &str, flag: OpenFlags) -> isize {
    // 用户态统一走 openat(AT_FDCWD, ...)。
    sys_openat(
        AT_FDCWD,
        name.as_ptr(),
        flag.bits() as usize,
        FileMode::FMODE_RDWR.bits() as usize,
    )
}

/// now we don't support mode
pub fn openat(fd: isize, name: &str, flag: OpenFlags, file_mode: FileMode) -> isize {
    sys_openat(
        fd,
        name.as_ptr(),
        flag.bits() as usize,
        file_mode.bits() as usize,
    )
}

pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

pub fn get_cwd(buf: &mut [u8]) -> Result<&str, IoError> {
    let len = sys_getcwd(buf.as_mut_ptr(), buf.len());
    if len == -1 {
        return Err(IoError::BufferTooSmall);
    } else {
        let res = buf.iter().enumerate().find(|&(_, &x)| x == 0);
        let len = if res.is_none() {
            buf.len()
        } else {
            res.unwrap().0
        };
        let s = core::str::from_utf8(&buf[..len as usize]).unwrap();
        Ok(s)
    }
}

pub fn chdir(path: &str) -> isize {
    sys_chdir(path.as_ptr())
}

pub fn mkdir(path: &str, mode: usize) -> isize {
    // 用户态统一走 mkdirat(AT_FDCWD, ...)。
    sys_mkdirat(AT_FDCWD, path.as_ptr(), mode)
}

pub fn seek(fd: usize, offset: isize, whence: usize) -> isize {
    sys_lseek(fd, offset, whence)
}

pub fn fstat(fd: usize, stat: &mut Stat) -> isize {
    sys_fstat(fd, stat as *mut Stat as *mut u8)
}

pub fn mount(source: &str, target: &str, fs_type: &str, flags: usize, data: &str) -> isize {
    sys_mount(
        source.as_ptr(),
        target.as_ptr(),
        fs_type.as_ptr(),
        flags,
        data.as_ptr(),
    )
}

pub fn linkat(
    old_fd: isize,
    old_path: &str,
    new_fd: usize,
    new_path: &str,
    flag: LinkFlags,
) -> isize {
    sys_linkat(
        old_fd,
        old_path.as_ptr(),
        new_fd,
        new_path.as_ptr(),
        flag.bits() as usize,
    )
}

pub fn unlinkat(fd: isize, path: &str, flag: usize) -> isize {
    sys_unlinkat(fd, path.as_ptr(), flag)
}

pub fn symlinkat(old_path: &str, new_fd: isize, new_path: &str) -> isize {
    sys_symlinkat(old_path.as_ptr(), new_fd, new_path.as_ptr())
}

pub fn readlinkat(fd: isize, path: &str, buf: &mut [u8]) -> isize {
    sys_readlinkat(fd, path.as_ptr(), buf.as_mut_ptr(), buf.len())
}

pub fn fstatat(fd: isize, path: &str, stat: &mut Stat, flag: StatFlags) -> isize {
    sys_newfstatat(
        fd,
        path.as_ptr(),
        stat as *mut Stat as *mut u8,
        flag.bits() as usize,
    )
}

pub fn statfs(path: &str, stat: &mut StatFs) -> isize {
    sys_statfs(path.as_ptr(), stat as *mut StatFs as *mut u8)
}

pub fn fstatfs(fd: usize, stat: &mut StatFs) -> isize {
    sys_fstatfs(fd, stat as *mut StatFs as *mut u8)
}

pub fn renameat(old_fd: isize, old_path: &str, new_fd: isize, new_path: &str) -> isize {
    // 无 flags 的 renameat 兼容为 renameat2(..., 0)。
    sys_renameat2(old_fd, old_path.as_ptr(), new_fd, new_path.as_ptr(), 0)
}

pub fn mkdirat(fd: isize, path: &str, mode: usize) -> isize {
    sys_mkdirat(fd, path.as_ptr(), mode)
}

#[derive(Debug)]
pub enum IoError {
    BufferTooSmall,
    FileNotFound,
    FileAlreadyExist,
}
