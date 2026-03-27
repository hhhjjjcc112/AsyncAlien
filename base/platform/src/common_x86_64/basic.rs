//! x86_64 机器信息。

use core::{fmt::Debug, ops::Range};

use multiboot::information::{MemoryManagement, Module, Multiboot, PAddr};

use super::boot::PHYS_VIRT_OFFSET;

const BOOTARGS_MAX: usize = 255;

/// 机器信息结构。
///
/// x86_64 下保留 plic/clint 字段，仅用于接口兼容。
#[derive(Clone)]
pub struct MachineInfo {
    /// 机器型号。
    pub model: [u8; 32],
    /// CPU 数量。
    pub smp: usize,
    /// 物理内存区间。
    pub memory: Range<usize>,
    /// 兼容字段：x86_64 下不承载真实设备发现语义。
    pub plic: Range<usize>,
    /// 兼容字段：x86_64 下不承载真实设备发现语义。
    pub clint: Range<usize>,
    /// initrd 区间（若由引导器加载）。
    pub initrd: Option<Range<usize>>,
    /// 启动参数。
    pub bootargs: Option<[u8; 255]>,
    /// 启动参数长度。
    pub bootargs_len: usize,
}

impl Debug for MachineInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let index = self.model.iter().position(|&x| x == 0).unwrap_or(32);
        let model = core::str::from_utf8(&self.model[..index]).unwrap_or("x86_64");
        writeln!(f, "Machine: {}", model)?;
        writeln!(f, "SMP:     {} CPUs", self.smp)?;
        writeln!(f, "Memory:  {:#x}..{:#x}", self.memory.start, self.memory.end)?;
        writeln!(
            f,
            "Compat:  plic={:#x}..{:#x}, clint={:#x}..{:#x}",
            self.plic.start,
            self.plic.end,
            self.clint.start,
            self.clint.end
        )?;
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

/// 从 boot_info（Multiboot 指针）构建机器信息。
pub fn machine_info_from_boot_info(multiboot_ptr: usize) -> MachineInfo {
    // 先根据 Multiboot 初始化内存区间。
    super::mem::init_from_multiboot(multiboot_ptr);
    
    // 从 CPUID 获取 CPU 数。
    let smp = get_cpu_count();
    
    // 组装机器信息。
    let mut model = [0u8; 32];
    let name = b"qemu-x86_64-pc";
    model[..name.len()].copy_from_slice(name);
    
    let (initrd, bootargs, bootargs_len) = parse_multiboot_extras(multiboot_ptr);

    MachineInfo {
        model,
        smp,
        memory: super::mem::memory_range(),
        // x86_64 下这两个字段仅为兼容占位，不作为设备发现结果。
        plic: 0..0,
        clint: 0..0,
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

/// 从 CPUID 获取逻辑 CPU 数。
fn get_cpu_count() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(1, |finfo| {
            let count = finfo.max_logical_processor_ids() as usize;
            if count == 0 { 1 } else { count }
        })
}
