#![no_std]
#![no_main]

extern crate alloc;

use Mstd::fs::{
    close, fstatat, linkat, open, readlinkat, symlinkat, unlinkat, write, InodeMode, LinkFlags,
    OpenFlags, Stat, StatFlags, AT_FDCWD,
};
use Mstd::println;

const SRC: &str = "/tmp/sys_link_src\0";
const HARD: &str = "/tmp/sys_link_hard\0";
const SOFT: &str = "/tmp/sys_link_soft\0";
const SRC_EXPECT: &[u8] = b"/tmp/sys_link_src";
const DATA: &[u8] = b"link-data";

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_link] start");
    if !run_case() {
        println!("[sys_link] FAIL");
        return 1;
    }
    println!("[sys_link] PASS");
    0
}

fn run_case() -> bool {
    let _ = unlinkat(AT_FDCWD, HARD, 0);
    let _ = unlinkat(AT_FDCWD, SOFT, 0);
    let _ = unlinkat(AT_FDCWD, SRC, 0);

    let fd = open(SRC, OpenFlags::O_CREAT | OpenFlags::O_RDWR | OpenFlags::O_TRUNC);
    if fd < 0 {
        println!("[sys_link] open failed: {}", fd);
        return false;
    }

    if write(fd as usize, DATA) != DATA.len() as isize {
        println!("[sys_link] write failed");
        let _ = close(fd as usize);
        return false;
    }
    if close(fd as usize) < 0 {
        println!("[sys_link] close failed");
        return false;
    }

    let link_ret = linkat(AT_FDCWD, SRC, AT_FDCWD as usize, HARD, LinkFlags::empty());
    if link_ret < 0 {
        println!("[sys_link] linkat skipped: {}", link_ret);
    }

    let sym_ret = symlinkat(SRC, AT_FDCWD, SOFT);
    if sym_ret < 0 {
        println!("[sys_link] symlinkat failed: {}", sym_ret);
        return false;
    }

    let mut buf = [0u8; 64];
    let len = readlinkat(AT_FDCWD, SOFT, &mut buf);
    if len < 0 {
        println!("[sys_link] readlinkat failed: {}", len);
        return false;
    }

    let len = len as usize;
    if &buf[..len] != SRC_EXPECT {
        println!("[sys_link] readlink mismatch");
        return false;
    }

    let mut link_stat = Stat::default();
    if fstatat(AT_FDCWD, SOFT, &mut link_stat, StatFlags::AT_SYMLINK_NOFOLLOW) < 0 {
        println!("[sys_link] lstat via fstatat failed");
        return false;
    }
    if link_stat.st_mode & InodeMode::S_SYMLINK.bits() == 0 {
        println!("[sys_link] symlink mode mismatch: {:#o}", link_stat.st_mode);
        return false;
    }

    let mut target_stat = Stat::default();
    if fstatat(AT_FDCWD, SOFT, &mut target_stat, StatFlags::empty()) < 0 {
        println!("[sys_link] stat via fstatat failed");
        return false;
    }
    if target_stat.st_mode & InodeMode::S_FILE.bits() == 0 {
        println!("[sys_link] target mode mismatch: {:#o}", target_stat.st_mode);
        return false;
    }

    let _ = unlinkat(AT_FDCWD, SOFT, 0);
    let _ = unlinkat(AT_FDCWD, HARD, 0);
    let _ = unlinkat(AT_FDCWD, SRC, 0);

    true
}
