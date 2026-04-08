#![no_std]
#![no_main]

extern crate alloc;

use Mstd::fs::{
    close, linkat, open, readlinkat, symlinkat, write, LinkFlags, OpenFlags, AT_FDCWD,
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

    if linkat(AT_FDCWD, SRC, AT_FDCWD as usize, HARD, LinkFlags::empty()) < 0 {
        println!("[sys_link] linkat failed");
        return false;
    }

    if symlinkat(SRC, AT_FDCWD, SOFT) < 0 {
        println!("[sys_link] symlinkat failed");
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

    true
}
