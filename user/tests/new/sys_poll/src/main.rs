#![no_std]
#![no_main]

extern crate alloc;

use Mstd::{
    fs::{poll, read, write},
    ipc::pipe,
    println,
};
use pconst::{
    io::{PollEvents, PollFd},
};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[sys_poll] start");
    if !run_case() {
        println!("[sys_poll] FAIL");
        return 1;
    }
    println!("[sys_poll] PASS");
    0
}

fn run_case() -> bool {
    let mut sockets = [0u32; 2];
    let result = pipe(&mut sockets);
    if result < 0 {
        println!("[sys_poll] pipe failed: {}", result);
        return false;
    }

    let read_fd = sockets[0] as usize;
    let write_fd = sockets[1] as usize;

    let mut fds = [PollFd {
        fd: read_fd as i32,
        events: PollEvents::EPOLLIN,
        revents: PollEvents::empty(),
    }];
    let timeout_ms = 50;

    if poll(&mut fds, 0) != 0 {
        println!("[sys_poll] empty poll should return 0");
        return false;
    }
    if !fds[0].revents.is_empty() {
        println!("[sys_poll] empty poll unexpectedly set revents");
        return false;
    }
    println!("[sys_poll] empty poll ok");

    let payload = [0x5au8];
    if write(write_fd, &payload) != 1 {
        println!("[sys_poll] write failed");
        return false;
    }
    println!("[sys_poll] write ok");

    fds[0].revents = PollEvents::empty();
    if poll(&mut fds, 0) != 1 {
        println!("[sys_poll] ready poll should return 1");
        return false;
    }
    if !fds[0].revents.contains(PollEvents::EPOLLIN) {
        println!("[sys_poll] ready poll missing EPOLLIN");
        return false;
    }
    println!("[sys_poll] ready poll ok");

    let mut buffer = [0u8; 1];
    if read(read_fd, &mut buffer) != 1 || buffer[0] != 0x5a {
        println!("[sys_poll] read mismatch");
        return false;
    }
    println!("[sys_poll] read ok");

    fds[0].revents = PollEvents::empty();
    if poll(&mut fds, timeout_ms) != 0 {
        println!("[sys_poll] timeout poll should return 0");
        return false;
    }
    if !fds[0].revents.is_empty() {
        println!("[sys_poll] timeout poll unexpectedly set revents");
        return false;
    }
    println!("[sys_poll] timeout poll ok");

    fds[0].fd = i32::MAX;
    fds[0].revents = PollEvents::empty();
    if poll(&mut fds, 0) != 1 {
        println!("[sys_poll] invalid fd poll should report an error event");
        return false;
    }
    if !fds[0].revents.contains(PollEvents::EPOLLERR) {
        println!("[sys_poll] invalid fd poll missing EPOLLERR");
        return false;
    }
    println!("[sys_poll] invalid fd poll ok");

    true
}
