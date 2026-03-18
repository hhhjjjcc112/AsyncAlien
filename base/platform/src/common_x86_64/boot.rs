//! x86_64 启动支持。

use core::arch::global_asm;

use x86_64::registers::control::{Cr0Flags, Cr4Flags, EferFlags};

/// Multiboot 头标志（内存与地址字段）。
const MULTIBOOT_HEADER_FLAGS: usize = 0x0001_0002;
/// Multiboot 头魔数。
const MULTIBOOT_HEADER_MAGIC: usize = 0x1BAD_B002;
/// 引导器传入的 Multiboot 魔数。
pub(crate) const MULTIBOOT_BOOTLOADER_MAGIC: usize = 0x2BAD_B002;

/// CR0 位：保护模式、分页、FPU 相关。
const CR0: u64 = Cr0Flags::PROTECTED_MODE_ENABLE.bits()
    | Cr0Flags::PAGING.bits()
    | Cr0Flags::MONITOR_COPROCESSOR.bits()
    | Cr0Flags::NUMERIC_ERROR.bits()
    | Cr0Flags::WRITE_PROTECT.bits();

/// CR4 位：PAE 与全局页。
const CR4: u64 = Cr4Flags::PHYSICAL_ADDRESS_EXTENSION.bits() | Cr4Flags::PAGE_GLOBAL.bits();

/// EFER 位：长模式与 NX。
const EFER: u64 = EferFlags::LONG_MODE_ENABLE.bits() | EferFlags::NO_EXECUTE_ENABLE.bits();

/// 启动栈大小。
pub const BOOT_STACK_SIZE: usize = 0x40000;

/// 物理到虚拟地址偏移，当前使用恒等映射。
pub const PHYS_VIRT_OFFSET: u64 = 0;

/// 启动栈。
#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

// 引入启动汇编。
global_asm!(
    include_str!("multiboot.S"),
    mb_magic = const MULTIBOOT_BOOTLOADER_MAGIC,
    mb_hdr_magic = const MULTIBOOT_HEADER_MAGIC,
    mb_hdr_flags = const MULTIBOOT_HEADER_FLAGS,
    entry = sym main_entry,
    entry_secondary = sym secondary_entry,
    phys_virt_offset = const PHYS_VIRT_OFFSET,
    boot_stack_size = const BOOT_STACK_SIZE,
    boot_stack = sym BOOT_STACK,
    cr0 = const CR0,
    cr4 = const CR4,
    efer_msr = const x86::msr::IA32_EFER,
    efer = const EFER,
);

/// 汇编入口转入 Rust 主入口。
#[unsafe(no_mangle)]
fn main_entry(magic: usize, mbi: usize) {
    // 早期启动标记，确认已进入 Rust 入口。
    println!("[x86_boot] main_entry magic={:#x} mbi={:#x}", magic, mbi);
    if magic == MULTIBOOT_BOOTLOADER_MAGIC {
        // 进入平台初始化。
        unsafe { crate::main(arch::cpu_id_early(), mbi) };
    } else {
        println!("[x86_boot] invalid multiboot magic: {:#x}", magic);
    }
}

/// 从核汇编入口。
#[unsafe(no_mangle)]
fn secondary_entry(magic: usize) {
    println!("[x86_boot] ap boot");
    if magic == MULTIBOOT_BOOTLOADER_MAGIC {
        // 进入从核主函数。
        unsafe { crate::secondary_main(arch::cpu_id_early()) };
    } else {
        println!("[x86_boot] invalid multiboot magic: {:#x}", magic);
    }
}
