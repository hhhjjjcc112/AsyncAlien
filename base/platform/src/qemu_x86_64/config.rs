//! QEMU x86-64 platform configuration
//!
//! Static device configuration for x86 PC platform.

/// TSC frequency in Hz (estimated, will be calibrated at runtime)
pub const CLOCK_FREQ: usize = 4_000_000_000;

/// Timer interrupt vector
pub const TIMER_IRQ: u8 = 0xf0;

/// APIC spurious vector
pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;

/// APIC error vector  
pub const APIC_ERROR_VECTOR: u8 = 0xf2;

/// IPI interrupt vector
pub const IPI_IRQ: u8 = 0xf3;

/// Physical to virtual offset for linear mapping
pub const PHYS_VIRT_OFFSET: usize = 0xffff_8000_0000_0000;

/// Kernel base physical address
pub const KERNEL_BASE_PADDR: usize = 0x20_0000;

/// Kernel base virtual address
pub const KERNEL_BASE_VADDR: usize = PHYS_VIRT_OFFSET + KERNEL_BASE_PADDR;

/// Boot stack size (256K)
pub const BOOT_STACK_SIZE: usize = 0x40000;

/// MMIO ranges with format (base_paddr, size)
/// These should be identity-mapped or mapped with PHYS_VIRT_OFFSET
pub const MMIO_RANGES: &[(usize, usize)] = &[
    (0xb000_0000, 0x1000_0000), // PCI ECAM config space
    (0xfe00_0000, 0xc0_0000),   // PCI device memory
    (0xfec0_0000, 0x1000),      // IO APIC
    (0xfed0_0000, 0x1000),      // HPET
    (0xfee0_0000, 0x1000),      // Local APIC
];

/// PCI ECAM base address (should read from ACPI MCFG table)
pub const PCI_ECAM_BASE: usize = 0xb000_0000;

/// End PCI bus number
pub const PCI_BUS_END: u8 = 0xff;

/// VirtIO MMIO ranges (not used on x86, using PCI instead)
pub const VIRTIO_MMIO_RANGES: &[(usize, usize)] = &[];

/// Reserved memory regions (lower 1MiB)
pub const RESERVED_MEMORY: &[(usize, usize)] = &[
    (0, 0x100000), // Lower 1MiB reserved for legacy devices
];

/// Device space for compatibility with RISC-V interface
pub const DEVICE_SPACE: &[(&str, usize, usize)] = &[
    ("local_apic", 0xfee0_0000, 0x1000),
    ("io_apic", 0xfec0_0000, 0x1000),
    ("hpet", 0xfed0_0000, 0x1000),
    ("pci_ecam", 0xb000_0000, 0x1000_0000),
];
