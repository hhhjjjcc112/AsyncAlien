//! Time management interface
//!
//! Provides time measurement and timer functionality abstraction.

/// Number of nanoseconds in a second
pub const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Time operations trait
///
/// Platform implementations provide time measurement using platform-specific
/// hardware (TSC on x86, cycle counter on RISC-V, etc.)
pub trait TimeIf {
    /// Returns the current clock time in hardware ticks
    fn current_ticks() -> u64;

    /// Returns the tick frequency in Hz
    fn tick_freq() -> u64;

    /// Converts hardware ticks to nanoseconds
    fn ticks_to_nanos(ticks: u64) -> u64 {
        ticks * NANOS_PER_SEC / Self::tick_freq()
    }

    /// Converts nanoseconds to hardware ticks
    fn nanos_to_ticks(nanos: u64) -> u64 {
        nanos * Self::tick_freq() / NANOS_PER_SEC
    }

    /// Returns epoch offset in nanoseconds (wall time offset to monotonic clock start)
    fn epochoffset_nanos() -> u64;

    /// Set a one-shot timer interrupt
    ///
    /// A timer interrupt will be triggered at the specified deadline.
    /// The unit depends on platform (cycles or absolute time).
    fn set_timer(deadline: u64);

    /// Returns nanoseconds elapsed since system boot (monotonic time)
    fn monotonic_time_nanos() -> u64 {
        Self::ticks_to_nanos(Self::current_ticks())
    }

    /// Returns nanoseconds elapsed since epoch (wall time)
    fn wall_time_nanos() -> u64 {
        Self::monotonic_time_nanos() + Self::epochoffset_nanos()
    }
}
