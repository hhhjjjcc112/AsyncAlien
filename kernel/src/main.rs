#![feature(box_into_inner)]
#![feature(allocator_api)]
#![feature(ptr_metadata)]
#![allow(clippy::declare_interior_mutable_const)]
#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]
mod panic;

// Two-layer cfg guards: architecture + platform.
#[cfg(not(any(target_arch = "riscv64", target_arch = "x86_64")))]
compile_error!("Unsupported architecture. Expected target_arch = riscv64 or x86_64");

#[cfg(not(any(plat_qemu_riscv, plat_vf2, plat_qemu_x86_64)))]
compile_error!("No valid platform selected! Use --cfg plat_qemu_riscv, --cfg plat_vf2, or --cfg plat_qemu_x86_64");

#[cfg(any(
    all(plat_qemu_riscv, plat_vf2),
    all(plat_qemu_riscv, plat_qemu_x86_64),
    all(plat_vf2, plat_qemu_x86_64)
))]
compile_error!("Multiple platforms selected! Select exactly one platform cfg");

#[cfg(all(target_arch = "x86_64", not(plat_qemu_x86_64)))]
compile_error!("ARCH x86_64 requires PLATFORM=plat_qemu_x86_64");

#[cfg(all(target_arch = "riscv64", not(any(plat_qemu_riscv, plat_vf2))))]
compile_error!("ARCH riscv64 requires PLATFORM=plat_qemu_riscv or plat_vf2");

#[macro_use]
extern crate platform;
#[macro_use]
extern crate log;
extern crate alloc;
mod bus;
mod domain;
mod domain_helper;
mod domain_loader;
mod domain_proxy;
mod error;
mod sync;
mod task;
mod timer;
mod trap;

use core::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, Ordering},
};

/// 多核启动标志
static STARTED: AtomicBool = AtomicBool::new(false);

/// Main entry point
/// 
/// `boot_cpu_id` is the CPU ID of the boot processor.
/// - RISC-V: This is the hart ID passed from bootloader
/// - x86-64: This is the BSP's Local APIC ID
#[unsafe(no_mangle)]
fn main(boot_cpu_id: usize) {
    if STARTED
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        println!("Boot CPU {}", boot_cpu_id);
        let machine_info = platform::platform_machine_info();
        println!("{:#?}", machine_info);
        mem::init_memory_system(machine_info.memory.end, true);
        arch::allow_access_user_memory();
        bus::init_with_boot_info().unwrap();
        trap::init_trap_subsystem();

        domain::load_domains().unwrap();
        STARTED.store(false, Ordering::Relaxed);
    } else {
        while STARTED.load(Ordering::Relaxed) {
            spin_loop();
        }
        mem::init_memory_system(0, false);
        arch::allow_access_user_memory();
        trap::init_trap_subsystem();
        println!("CPU {} start...", arch::cpu_id());
    }
    #[cfg(feature = "test")]
    panic::test_unwind();
    timer::set_next_trigger();
    println!("Begin run task...");
    task::run_task();
}

/// Secondary CPU entry point (for x86-64 APs)
/// 
/// Called from platform code when Application Processors (APs) are started.
/// This is the x86-64 equivalent of RISC-V hart startup.
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
extern "C" fn secondary_main(cpu_id: usize) {
    // Call main - it will handle the non-boot CPU path
    main(cpu_id);
}
