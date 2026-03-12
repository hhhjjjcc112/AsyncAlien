//! 杂项平台接口（启动信息与机器信息）。

use core::{fmt::Debug, ops::Range};

/// 机器信息抽象。
pub trait MachineInfo: Clone + Debug + Send + Sync {
    /// 内存起始地址。
    fn memory_start(&self) -> usize;

    /// 内存总大小。
    fn memory_size(&self) -> usize;

    /// 内存地址区间。
    fn memory_range(&self) -> Range<usize> {
        self.memory_start()..self.memory_start() + self.memory_size()
    }

    /// CPU 数量。
    fn cpu_count(&self) -> usize;

    /// initrd 区间（若存在）。
    fn initrd(&self) -> Option<Range<usize>>;

    /// 启动参数（若存在）。
    fn bootargs(&self) -> Option<&str>;
}

/// 启动阶段相关杂项操作。
pub trait MiscIf {
    /// 对应的平台机器信息类型。
    type MachineInfo: MachineInfo;

    /// 解析并初始化启动信息。
    fn init_boot_info(ptr: usize);

    /// 返回启动信息指针（DTB 或 Multiboot）。
    fn boot_info_ptr() -> usize;

    /// 返回基础机器信息。
    fn machine_info() -> Self::MachineInfo;
}
