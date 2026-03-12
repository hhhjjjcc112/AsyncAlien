//! 时间管理接口。

/// 每秒纳秒数。
pub const NANOS_PER_SEC: u64 = 1_000_000_000;

/// 时间操作抽象。
pub trait TimeIf {
    /// 读取当前硬件时钟 tick。
    fn current_ticks() -> u64;

    /// 返回 tick 频率（Hz）。
    fn tick_freq() -> u64;

    /// tick 转纳秒。
    fn ticks_to_nanos(ticks: u64) -> u64 {
        ticks * NANOS_PER_SEC / Self::tick_freq()
    }

    /// 纳秒转 tick。
    fn nanos_to_ticks(nanos: u64) -> u64 {
        nanos * Self::tick_freq() / NANOS_PER_SEC
    }

    /// 返回纪元偏移（纳秒）。
    fn epochoffset_nanos() -> u64;

    /// 设置单次定时中断。
    fn set_timer(deadline: u64);

    /// 返回开机以来的单调时间（纳秒）。
    fn monotonic_time_nanos() -> u64 {
        Self::ticks_to_nanos(Self::current_ticks())
    }

    /// 返回墙钟时间（纳秒）。
    fn wall_time_nanos() -> u64 {
        Self::monotonic_time_nanos() + Self::epochoffset_nanos()
    }
}
