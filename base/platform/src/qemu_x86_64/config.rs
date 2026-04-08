//! QEMU x86_64 平台配置。

/// TSC 频率（Hz，运行期会重新校准）。
pub const CLOCK_FREQ: usize = 4_000_000_000;

/// 定时器中断向量。
pub const TIMER_IRQ: u8 = 0xf0;

/// APIC timer 模式开关：默认 one-shot。
/// 若改为 `false`，将启用 periodic 模式（需配合 trap 分支）。
pub const APIC_TIMER_ONESHOT: bool = true;

/// APIC 伪中断向量。
pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;

/// APIC 错误中断向量。
pub const APIC_ERROR_VECTOR: u8 = 0xf2;

/// 是否使用静态 ACPI 设备表（不做运行期 ACPI 表探测）。
pub const STATIC_ACPI: bool = true;

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

/// 平台内部的静态 ACPI 兼容设备描述。
pub const DEVICE_SPACE: &[(&str, usize, usize)] = &[
    ("local_apic", 0xfee0_0000, 0x1000),
    ("io_apic", 0xfec0_0000, 0x1000),
    ("hpet", 0xfed0_0000, 0x1000),
    ("pci_ecam", 0xb000_0000, 0x1000_0000),
];

