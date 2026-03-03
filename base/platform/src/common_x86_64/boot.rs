//! x86-64 Boot support
//!
//! Provides Multiboot boot entry and early initialization.

use core::arch::global_asm;

use x86_64::registers::control::{Cr0Flags, Cr4Flags, EferFlags};

/// Multiboot header flags (memory info, address fields)
const MULTIBOOT_HEADER_FLAGS: usize = 0x0001_0002;
/// Multiboot header magic
const MULTIBOOT_HEADER_MAGIC: usize = 0x1BAD_B002;
/// Multiboot bootloader magic (passed by bootloader)
pub(crate) const MULTIBOOT_BOOTLOADER_MAGIC: usize = 0x2BAD_B002;

/// CR0 flags: Protected mode, Paging, FPU settings
const CR0: u64 = Cr0Flags::PROTECTED_MODE_ENABLE.bits()
    | Cr0Flags::PAGING.bits()
    | Cr0Flags::MONITOR_COPROCESSOR.bits()
    | Cr0Flags::NUMERIC_ERROR.bits()
    | Cr0Flags::WRITE_PROTECT.bits();

/// CR4 flags: PAE, Page Global Enable
const CR4: u64 = Cr4Flags::PHYSICAL_ADDRESS_EXTENSION.bits() | Cr4Flags::PAGE_GLOBAL.bits();

/// EFER flags: Long Mode Enable, NX Enable
const EFER: u64 = EferFlags::LONG_MODE_ENABLE.bits() | EferFlags::NO_EXECUTE_ENABLE.bits();

/// Boot stack size
pub const BOOT_STACK_SIZE: usize = 0x40000;

/// Physical to virtual address offset
/// For now, use identity mapping (offset = 0)
pub const PHYS_VIRT_OFFSET: u64 = 0;

/// Boot stack
#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

// Include boot assembly
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

/// Get current CPU ID from APIC
pub fn current_cpu_id() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(0, |finfo| finfo.initial_local_apic_id() as usize)
}

/// Main entry point called from assembly
#[unsafe(no_mangle)]
fn main_entry(magic: usize, mbi: usize) {
    if magic == MULTIBOOT_BOOTLOADER_MAGIC {
        // Call platform initialization
        crate::platform_init_with_boot_info(current_cpu_id(), mbi);
    }
}

/// Secondary CPU entry point called from assembly
#[unsafe(no_mangle)]
fn secondary_entry(_magic: usize) {
    crate::common_x86_64::apic::init_secondary_apic();
    crate::common_x86_64::time::init_secondary_apic_timer();

    unsafe extern "C" {
        fn secondary_main(cpu_id: usize);
    }
    unsafe {
        secondary_main(current_cpu_id());
    }
}
