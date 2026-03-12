//! 电源与多核启动接口。

/// 电源管理抽象。
pub trait PowerIf {
    /// 关机，不返回。
    fn shutdown() -> !;

    /// 重启，不返回。
    fn reboot() -> ! {
        // 默认退化为关机。
        Self::shutdown()
    }

    /// 启动从核。
    fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize);

    /// 返回可用 CPU 数量。
    fn cpu_count() -> usize;

    /// 返回当前 CPU ID。
    fn current_cpu_id() -> usize;

    /// 停机等待中断。
    fn halt();

    /// 刷新目标 CPU 的指令可见性（x86 上通常为空操作）。
    fn flush_cache(cpu_mask: usize, cpu_mask_base: usize);
}
