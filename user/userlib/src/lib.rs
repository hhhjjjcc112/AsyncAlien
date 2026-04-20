#![no_std]
#![feature(linkage)]
#![allow(unused)]
#![allow(non_snake_case)]
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{heap::init_heap, process::exit, syscall::__system_shutdown};

mod arch;

pub mod common;
pub mod fs;
mod heap;
pub mod io;
pub mod ipc;
mod macros;
pub mod memory;
mod panic;
pub mod random;
pub mod process;
pub mod pthread;
pub mod socket;
mod sys;
mod syscall;
pub mod thread;
pub mod time;
pub mod vdso;

pub mod domain;
#[cfg(feature = "gui")]
pub mod gui;
pub mod sync;

#[unsafe(no_mangle)]
fn _start_rust(argc_ptr: usize) {
    init_heap();

    let (argc, argv) = parse_start_stack(argc_ptr);
    vdso::init_from_stack(argc_ptr);
    exit(unsafe { main(argc, argv) });
}

fn parse_start_stack(argc_ptr: usize) -> (usize, Vec<String>) {
    let argc = unsafe { (argc_ptr as *const usize).read_volatile() };
    let argv = argc_ptr + core::mem::size_of::<usize>();

    let argv = parse_args(argc, argv);
    (argc, argv)
}

fn parse_args(argc: usize, argv: usize) -> Vec<String> {
    let mut args = Vec::new();
    for i in 0..argc {
        let arg = unsafe { *(argv as *const *const u8).add(i) };
        let len = unsafe { common::strlen(arg) };
        let arg = unsafe {
            let slice = core::slice::from_raw_parts(arg, len);
            core::str::from_utf8_unchecked(slice)
        };
        args.push(arg.to_string());
    }
    args
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main(argc: usize, argv: Vec<String>) -> i32 {
    panic!("Cannot find main!");
}

pub fn system_shutdown() -> ! {
    __system_shutdown();
    panic!("Shutdown failed!");
}
