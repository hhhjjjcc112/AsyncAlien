//! QEMU x86_64 平台配置。

/// TSC 频率（Hz，运行期会重新校准）。
pub const CLOCK_FREQ: usize = 4_000_000_000;

/// 定时器中断向量。
pub const TIMER_IRQ: u8 = 0xf0;

/// APIC 伪中断向量。
pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;

/// APIC 错误中断向量。
pub const APIC_ERROR_VECTOR: u8 = 0xf2;

/// IPI 中断向量。
pub const IPI_IRQ: u8 = 0xf3;

/// 线性映射的物理-虚拟偏移。
pub const PHYS_VIRT_OFFSET: usize = 0xffff_8000_0000_0000;

/// 内核基址物理地址。
pub const KERNEL_BASE_PADDR: usize = 0x20_0000;

/// 内核基址虚拟地址。
pub const KERNEL_BASE_VADDR: usize = PHYS_VIRT_OFFSET + KERNEL_BASE_PADDR;

/// 启动栈大小（256K）。
pub const BOOT_STACK_SIZE: usize = 0x40000;

/// MMIO 区间，格式为 `(base_paddr, size)`。
pub const MMIO_RANGES: &[(usize, usize)] = &[
    (0xb000_0000, 0x1000_0000), // PCI ECAM 配置空间
    (0xfe00_0000, 0xc0_0000),   // PCI 设备内存
    (0xfec0_0000, 0x1000),      // IO APIC
    (0xfed0_0000, 0x1000),      // HPET
    (0xfee0_0000, 0x1000),      // Local APIC
];

/// PCI ECAM 基址（理想情况下应来自 ACPI MCFG）。
pub const PCI_ECAM_BASE: usize = 0xb000_0000;

/// PCI 总线结束号。
pub const PCI_BUS_END: u8 = 0xff;

/// VirtIO MMIO 区间（x86 走 PCI，通常为空）。
pub const VIRTIO_MMIO_RANGES: &[(usize, usize)] = &[];

/// 保留内存区间（低 1MiB）。
pub const RESERVED_MEMORY: &[(usize, usize)] = &[
    (0, 0x100000), // 低 1MiB 预留给传统设备
];

/// 为兼容 RISC-V 接口保留的设备空间描述。
pub const DEVICE_SPACE: &[(&str, usize, usize)] = &[
    ("local_apic", 0xfee0_0000, 0x1000),
    ("io_apic", 0xfec0_0000, 0x1000),
    ("hpet", 0xfed0_0000, 0x1000),
    ("pci_ecam", 0xb000_0000, 0x1000_0000),
];

/// ACPI 动态发现的设备区间。
/// 若 ACPI 尚未初始化，则回退到静态设备表。
pub fn device_space_dynamic() -> heapless::Vec<(&'static str, usize, usize), 16> {
    let mut out = heapless::Vec::new();
    let list = crate::common_x86_64::acpi::device_list();
    if list.entries.is_empty() {
        for (name, base, size) in DEVICE_SPACE {
            let _ = out.push((*name, *base, *size));
        }
        return out;
    }

    for entry in list.entries.iter() {
        let _ = out.push((entry.name, entry.base, entry.size));
    }
    out
}
