//! 物理内存信息接口。

use core::ops::Range;

bitflags::bitflags! {
    /// 物理内存区域属性。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MemRegionFlags: usize {
        /// 可读。
        const READ = 1 << 0;
        /// 可写。
        const WRITE = 1 << 1;
        /// 可执行。
        const EXECUTE = 1 << 2;
        /// 设备内存（MMIO）。
        const DEVICE = 1 << 4;
        /// 不可缓存。
        const UNCACHED = 1 << 5;
        /// 保留区（不参与通用分配）。
        const RESERVED = 1 << 6;
        /// 可分配。
        const FREE = 1 << 7;
    }
}

impl Default for MemRegionFlags {
    fn default() -> Self {
        Self::READ | Self::WRITE | Self::FREE
    }
}

/// 原始区间格式：`(start, size)`。
pub type RawRange = (usize, usize);

/// 物理内存区域描述。
#[derive(Debug, Clone, Copy)]
pub struct PhysMemRegion {
    /// 起始物理地址。
    pub paddr: usize,
    /// 大小（字节）。
    pub size: usize,
    /// 区域属性。
    pub flags: MemRegionFlags,
    /// 名称（调试用）。
    pub name: &'static str,
}

impl PhysMemRegion {
    /// 构造 RAM 区域（可读可写可分配）。
    pub const fn new_ram(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::FREE),
            name,
        }
    }

    /// 构造 MMIO 区域（设备内存）。
    pub const fn new_mmio(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::DEVICE),
            name,
        }
    }

    /// 构造保留区域（不可分配）。
    pub const fn new_reserved(start: usize, size: usize, name: &'static str) -> Self {
        Self {
            paddr: start,
            size,
            flags: MemRegionFlags::READ.union(MemRegionFlags::WRITE).union(MemRegionFlags::RESERVED),
            name,
        }
    }

    /// 返回地址区间。
    pub const fn range(&self) -> Range<usize> {
        self.paddr..(self.paddr + self.size)
    }
}

/// 平台内存布局抽象。
pub trait MemIf {
    /// 物理地址到虚拟地址的线性映射偏移。
    const PHYS_VIRT_OFFSET: usize;

    /// 返回全部物理 RAM 区间。
    fn phys_ram_ranges() -> &'static [RawRange];

    /// 返回保留物理区间（内核、boot_info 等）。
    fn reserved_ranges() -> &'static [RawRange];

    /// 返回 MMIO 区间。
    fn mmio_ranges() -> &'static [RawRange];

    /// 返回页帧分配器使用的物理区间列表。
    fn alloc_ranges() -> &'static [RawRange];

    /// 物理地址转虚拟地址。
    fn phys_to_virt(paddr: usize) -> usize {
        paddr.wrapping_add(Self::PHYS_VIRT_OFFSET)
    }

    /// 虚拟地址转物理地址。
    fn virt_to_phys(vaddr: usize) -> usize {
        vaddr.wrapping_sub(Self::PHYS_VIRT_OFFSET)
    }

    /// 统计总 RAM 大小。
    fn total_ram_size() -> usize {
        Self::phys_ram_ranges().iter().map(|(_, size)| *size).sum()
    }
}
