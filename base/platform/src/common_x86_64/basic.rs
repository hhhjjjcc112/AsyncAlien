//! Machine information for x86-64
//!
//! Provides machine configuration based on static config and runtime detection.

use core::{fmt::Debug, ops::Range};

use multiboot::information::{MemoryManagement, Module, Multiboot, PAddr};

use super::boot::PHYS_VIRT_OFFSET;

const BOOTARGS_MAX: usize = 255;

/// Machine information structure
/// 
/// For x86-64, PLIC and CLINT are replaced with APIC concepts,
/// but we keep the fields for interface compatibility.
#[derive(Clone)]
pub struct MachineInfo {
    /// Machine model name
    pub model: [u8; 32],
    /// Number of CPUs
    pub smp: usize,
    /// Physical memory range
    pub memory: Range<usize>,
    /// Interrupt controller range (Local APIC on x86)
    pub plic: Range<usize>,
    /// Timer controller range (not used on x86, APIC timer is integrated)
    pub clint: Range<usize>,
    /// Initrd range (if loaded by bootloader)
    pub initrd: Option<Range<usize>>,
    /// Boot arguments
    pub bootargs: Option<[u8; 255]>,
    /// Boot arguments length
    pub bootargs_len: usize,
}

impl Debug for MachineInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let index = self.model.iter().position(|&x| x == 0).unwrap_or(32);
        let model = core::str::from_utf8(&self.model[..index]).unwrap_or("x86_64");
        writeln!(f, "Machine: {}", model)?;
        writeln!(f, "SMP:     {} CPUs", self.smp)?;
        writeln!(f, "Memory:  {:#x}..{:#x}", self.memory.start, self.memory.end)?;
        writeln!(f, "APIC:    {:#x}..{:#x}", self.plic.start, self.plic.end)?;
        if let Some(ref initrd) = self.initrd {
            writeln!(f, "Initrd:  {:#x}..{:#x}", initrd.start, initrd.end)?;
        }
        if let Some(ref args) = self.bootargs {
            let bootargs = core::str::from_utf8(&args[..self.bootargs_len]).unwrap_or("");
            if !bootargs.is_empty() {
                writeln!(f, "Bootargs: {}", bootargs)?;
            }
        }
        Ok(())
    }
}

/// Create machine info from boot information (Multiboot pointer)
pub fn machine_info_from_boot_info(multiboot_ptr: usize) -> MachineInfo {
    // Initialize memory regions from Multiboot
    super::mem::init_from_multiboot(multiboot_ptr);
    let acpi_info = super::acpi::device_info();
    
    // Get CPU count from CPUID
    let smp = get_cpu_count();
    
    // Build machine info
    let mut model = [0u8; 32];
    let name = b"qemu-x86_64-pc";
    model[..name.len()].copy_from_slice(name);
    
    let (initrd, bootargs, bootargs_len) = parse_multiboot_extras(multiboot_ptr);

    MachineInfo {
        model,
        smp,
        memory: super::mem::memory_range(),
        // Local APIC address from ACPI MADT.
        plic: acpi_info.lapic_base..(acpi_info.lapic_base + 0x1000),
        // IO APIC address from ACPI MADT (kept in CLINT slot for compatibility).
        clint: acpi_info.ioapic_base..(acpi_info.ioapic_base + 0x1000),
        initrd,
        bootargs,
        bootargs_len,
    }
}

struct BootInfoMemHelper;

impl MemoryManagement for BootInfoMemHelper {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let vaddr = addr as usize + PHYS_VIRT_OFFSET as usize;
        Some(unsafe { core::slice::from_raw_parts(vaddr as *const u8, size) })
    }

    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, _addr: PAddr) {}
}

fn pick_initrd_module<'a>(modules: impl Iterator<Item = Module<'a>>) -> Option<Range<usize>> {
    let mut first: Option<Range<usize>> = None;
    for m in modules {
        let range = m.start as usize..m.end as usize;
        if first.is_none() {
            first = Some(range.clone());
        }
        if let Some(name) = m.string {
            let lower = name.as_bytes();
            if lower.windows(6).any(|w| w.eq_ignore_ascii_case(b"initrd"))
                || lower.windows(4).any(|w| w.eq_ignore_ascii_case(b"cpio"))
            {
                return Some(range);
            }
        }
    }
    first
}

fn parse_multiboot_extras(multiboot_ptr: usize) -> (Option<Range<usize>>, Option<[u8; BOOTARGS_MAX]>, usize) {
    let mut mm = BootInfoMemHelper;
    let Some(info) = (unsafe { Multiboot::from_ptr(multiboot_ptr as PAddr, &mut mm) }) else {
        return (None, None, 0);
    };

    let initrd = info.modules().and_then(pick_initrd_module);

    let (bootargs, bootargs_len) = if let Some(cmdline) = info.command_line() {
        let bytes = cmdline.as_bytes();
        let len = bytes.len().min(BOOTARGS_MAX);
        let mut arr = [0u8; BOOTARGS_MAX];
        arr[..len].copy_from_slice(&bytes[..len]);
        (Some(arr), len)
    } else {
        (None, 0)
    };

    if let Some(ref initrd) = initrd {
        // 记录 initrd 区间，供内存系统早期搬运。
        println!("Initrd from multiboot: {:#x}..{:#x}", initrd.start, initrd.end);
    }

    (initrd, bootargs, bootargs_len)
}

/// Get number of logical CPUs from CPUID
fn get_cpu_count() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(1, |finfo| {
            let count = finfo.max_logical_processor_ids() as usize;
            if count == 0 { 1 } else { count }
        })
}
