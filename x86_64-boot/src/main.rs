#![no_std]
#![no_main]

mod boot;
mod ap;
mod apic;
mod console;
mod lang_items;
mod timer;

fn rust_main(cpu_id: usize, mbi: usize) {
    println!("Hello, x86_64 world!");
    println!("Current CPU ID: {}", cpu_id);
    println!("Multiboot Info Address: {:#x}", mbi);
}

fn rust_secondary_main(cpu_id: usize) {
    core::hint::spin_loop();
}
