//! Miscellaneous platform interface
//!
//! Provides boot information and machine descriptors.

use core::{fmt::Debug, ops::Range};

/// Platform call return value
#[derive(Debug, Copy, Clone, Default)]
pub struct PlatformCallRet {
    /// Error code (0 = success)
    pub error: isize,
    /// Return value
    pub value: isize,
}

impl PlatformCallRet {
    /// Create a success return
    pub const fn success(value: isize) -> Self {
        Self { error: 0, value }
    }

    /// Create an error return
    pub const fn error(error: isize) -> Self {
        Self { error, value: 0 }
    }

    /// Check if the call succeeded
    pub const fn is_success(&self) -> bool {
        self.error == 0
    }
}

/// Machine information trait
///
/// Common interface for machine descriptors across platforms.
pub trait MachineInfo: Clone + Debug + Send + Sync {
    /// Get memory start address
    fn memory_start(&self) -> usize;

    /// Get memory size
    fn memory_size(&self) -> usize;

    /// Get memory range
    fn memory_range(&self) -> Range<usize> {
        self.memory_start()..self.memory_start() + self.memory_size()
    }

    /// Get number of CPUs
    fn cpu_count(&self) -> usize;

    /// Get initrd range (if any)
    fn initrd(&self) -> Option<Range<usize>>;

    /// Get boot arguments (if any)
    fn bootargs(&self) -> Option<&str>;
}

/// Miscellaneous platform operations trait
///
/// Provides boot/init related operations that don't fit other categories.
pub trait MiscIf {
    /// Associated machine info type
    type MachineInfo: MachineInfo;

    /// Initialize boot information
    /// - On RISC-V: parse DTB (device tree blob)
    /// - On x86-64: parse Multiboot info
    fn init_boot_info(ptr: usize);

    /// Get boot information pointer (DTB or Multiboot address)
    fn boot_info_ptr() -> usize;

    /// Get basic machine information
    fn machine_info() -> Self::MachineInfo;
}
