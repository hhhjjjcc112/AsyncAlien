#![no_std]
#![no_main]

extern crate alloc;

use Mstd::fs::{
    close, fstat, ftruncate, open, statfs, truncate, write, OpenFlags, Stat, StatFs,
};
use Mstd::println;

const FILE_PATH_NUL: &str = "/tmp/sys_fs_case\0";
const SAMPLE: &[u8] = b"abcdef";

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_fs] start");
    if !run_case() {
        println!("[sys_fs] FAIL");
        return 1;
    }
    println!("[sys_fs] PASS");
    0
}

fn run_case() -> bool {
    let fd = open(FILE_PATH_NUL, OpenFlags::O_CREAT | OpenFlags::O_RDWR | OpenFlags::O_TRUNC);
    if fd < 0 {
        println!("[sys_fs] open failed: {}", fd);
        return false;
    }

    if write(fd as usize, SAMPLE) != SAMPLE.len() as isize {
        println!("[sys_fs] write failed");
        let _ = close(fd as usize);
        return false;
    }

    let mut stat = Stat::default();
    if fstat(fd as usize, &mut stat) < 0 || stat.st_size != SAMPLE.len() as u64 {
        println!("[sys_fs] fstat size mismatch: {}", stat.st_size);
        let _ = close(fd as usize);
        return false;
    }

    if ftruncate(fd as usize, 3) < 0 {
        println!("[sys_fs] ftruncate failed");
        let _ = close(fd as usize);
        return false;
    }

    if fstat(fd as usize, &mut stat) < 0 || stat.st_size != 3 {
        println!("[sys_fs] ftruncate size mismatch: {}", stat.st_size);
        let _ = close(fd as usize);
        return false;
    }

    if truncate(FILE_PATH_NUL, 1) < 0 {
        println!("[sys_fs] truncate failed");
        let _ = close(fd as usize);
        return false;
    }

    if fstat(fd as usize, &mut stat) < 0 || stat.st_size != 1 {
        println!("[sys_fs] truncate size mismatch: {}", stat.st_size);
        let _ = close(fd as usize);
        return false;
    }

    let mut fs = StatFs::default();
    if statfs("/tmp\0", &mut fs) < 0 {
        println!("[sys_fs] statfs failed");
        let _ = close(fd as usize);
        return false;
    }

    if fs.block_size == 0 || fs.total_blocks == 0 {
        println!("[sys_fs] statfs invalid fields");
        let _ = close(fd as usize);
        return false;
    }

    let mut fs2 = StatFs::default();
    if statfs(FILE_PATH_NUL, &mut fs2) < 0 {
        println!("[sys_fs] statfs on file failed");
        let _ = close(fd as usize);
        return false;
    }

    if fs2.block_size == 0 || fs2.total_blocks == 0 {
        println!("[sys_fs] statfs on file invalid fields");
        let _ = close(fd as usize);
        return false;
    }

    if close(fd as usize) < 0 {
        println!("[sys_fs] close failed");
        return false;
    }

    true
}
