use core::arch::global_asm;

use x86_64::registers::control::{Cr0Flags, Cr4Flags, EferFlags};

use crate::{rust_main, rust_secondary_main};

// (bits 1, 16: memory information, address fields in header)
const MULTIBOOT_HEADER_FLAGS: usize = 0x0001_0002;
// 固定的魔数
const MULTIBOOT_HEADER_MAGIC: usize = 0x1BAD_B002;
pub(crate) const MULTIBOOT_BOOTLOADER_MAGIC: usize = 0x2BAD_B002;
// CR0寄存器，用于启用保护模式和分页，并设置FPU相关标志
const CR0: u64 = Cr0Flags::PROTECTED_MODE_ENABLE.bits()
    | Cr0Flags::PAGING.bits()
    | Cr0Flags::MONITOR_COPROCESSOR.bits()
    | Cr0Flags::NUMERIC_ERROR.bits()
    | Cr0Flags::WRITE_PROTECT.bits();
// CR4寄存器，用于设置物理地址扩展和分页
const CR4: u64 = Cr4Flags::PHYSICAL_ADDRESS_EXTENSION.bits()
    | Cr4Flags::PAGE_GLOBAL.bits();
// EFER寄存器，用于启用长模式和NXE功能
const EFER: u64 = EferFlags::LONG_MODE_ENABLE.bits() | EferFlags::NO_EXECUTE_ENABLE.bits();
// 启动栈
pub const BOOT_STACK_SIZE: usize = 0x40000; 
#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];
// 物理内存与虚拟内存的偏移量
pub const PHYS_VIRT_OFFSET: u64 = 0xffff_8000_0000_0000;

global_asm!(
    include_str!("multiboot.S"),
    mb_magic = const MULTIBOOT_BOOTLOADER_MAGIC,
    mb_hdr_magic = const MULTIBOOT_HEADER_MAGIC,
    mb_hdr_flags = const MULTIBOOT_HEADER_FLAGS,
    entry = sym main_entry,
    entry_secondary = sym crate::rust_secondary_main,
    phys_virt_offset = const PHYS_VIRT_OFFSET,
    boot_stack_size = const BOOT_STACK_SIZE,
    boot_stack = sym BOOT_STACK,
    cr0 = const CR0,
    cr4 = const CR4,
    efer_msr = const x86::msr::IA32_EFER,
    efer = const EFER,
);

fn current_cpu_id() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(0, |finfo| finfo.initial_local_apic_id() as usize)
}

#[unsafe(no_mangle)]
fn main_entry(magic: usize, mbi: usize) {
    if magic == MULTIBOOT_BOOTLOADER_MAGIC {
        rust_main(current_cpu_id(), mbi);
    }
}

#[unsafe(no_mangle)]
fn secondary_entry(magic: usize) {
    if magic == MULTIBOOT_BOOTLOADER_MAGIC {
        rust_secondary_main(current_cpu_id());
    }
}