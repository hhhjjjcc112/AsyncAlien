#![no_std]
#![no_main]

use crate::ap::start_aps;

#[macro_use]
extern crate time;

mod boot;
mod ap;
mod apic;
mod console;
mod time;
mod lang_items;

fn rust_main(cpu_id: usize, multiboot_info_addr: usize) {
    // 在主CPU上初始化
    console::init_console();
    apic::init_primary_apic();
    time::init_time();
    
    println!("Hello, x86_64 world!");
    println!("Current CPU ID: {}", cpu_id);
    println!("Multiboot Info Address: {:#x}", multiboot_info_addr);

    start_aps();
}

fn rust_secondary_main(cpu_id: usize) {
    println!("Hello from secondary CPU {}!", cpu_id);
}
