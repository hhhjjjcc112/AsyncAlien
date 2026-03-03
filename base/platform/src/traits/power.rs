//! Power management interface
//!
//! Provides system power control and SMP boot capabilities.

use super::PlatformCallRet;

/// Power management trait
///
/// Platform implementations provide power control and multi-core boot.
pub trait PowerIf {
    /// System shutdown (never returns)
    fn system_off() -> !;

    /// System reboot (never returns)
    fn system_reboot() -> ! {
        // Default: just shutdown
        Self::system_off()
    }

    /// Bootstrap a secondary CPU core
    ///
    /// Arguments:
    /// - `cpu_id`: The logical CPU ID (0, 1, ..., N-1)
    /// - `start_addr`: The physical address where the CPU should start executing
    /// - `opaque`: Optional argument passed to the secondary CPU
    ///
    /// Returns: PlatformCallRet with error code and value
    ///
    /// On RISC-V: Uses SBI HSM hart_start
    /// On x86-64: Uses INIT-SIPI-SIPI sequence via APIC
    fn cpu_boot(cpu_id: usize, start_addr: usize, opaque: usize) -> PlatformCallRet;

    /// Get the number of CPU cores available
    fn cpu_num() -> usize;

    /// Get the current CPU ID
    fn current_cpu_id() -> usize;

    /// Halt the current CPU (wait for interrupt)
    fn cpu_halt();

    /// Remote instruction cache fence
    ///
    /// Flush instruction cache on specified CPUs.
    /// On RISC-V: FENCE.I via SBI RFENCE
    /// On x86-64: No-op (x86 has coherent I-cache)
    fn remote_fence_i(cpu_mask: usize, cpu_mask_base: usize) -> PlatformCallRet {
        PlatformCallRet::success(0)
    }
}
