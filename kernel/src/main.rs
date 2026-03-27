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
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};


/// 已完成从核初始化的数量。
static SECONDARY_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);
/// BSP 放行后，从核才进入调度。
static SECONDARY_RUN_RELEASED: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
fn main(boot_cpu_id: usize, boot_info_ptr: usize) {
    platform::clear_bss();
    platform::platform_init_percpu_primary(boot_cpu_id);
    platform::platform_init_primary(boot_cpu_id, boot_info_ptr);

    warn!("\n\n\n\n\n\ntest warning\n\n\n\n\n\n");

    mem::init_memory_system(true);
    trap::init_trap_subsystem();

    #[cfg(all(target_arch = "x86_64", feature = "trap_self_test"))]
    trap::run_trap_self_test();

    println!("Boot CPU {}", boot_cpu_id);
    let machine_info = platform::platform_machine_info();
    println!("{:#?}", machine_info);
    #[cfg(target_arch = "riscv64")]
    arch::allow_access_user_memory();
    bus::init_with_boot_info().unwrap();
    
    domain::load_domains().unwrap();

    platform::start_other_cpu(boot_cpu_id);

    let expected_secondary = machine_info.smp.saturating_sub(1);
    while SECONDARY_INIT_COUNT.load(Ordering::Acquire) < expected_secondary {
        spin_loop();
    }

    SECONDARY_RUN_RELEASED.store(true, Ordering::Release);

    #[cfg(feature = "test")]
    panic::test_unwind();
    timer::set_next_trigger();
    println!("Begin run task...");
    task::run_task();
}

/// 从核入口：由平台汇编从核入口直接调用。
#[unsafe(no_mangle)]
fn secondary_main(cpu_id: usize) {
    platform::platform_init_percpu_secondary(cpu_id);

    
    platform::platform_init_secondary(cpu_id);

    mem::init_memory_system(false);
    trap::init_trap_subsystem();
    #[cfg(target_arch = "riscv64")]
    arch::allow_access_user_memory();
    println!("CPU {} start...", cpu_id);

    SECONDARY_INIT_COUNT.fetch_add(1, Ordering::AcqRel);
    while !SECONDARY_RUN_RELEASED.load(Ordering::Acquire) {
        spin_loop();
    }

    timer::set_next_trigger();
    task::run_task();
}
