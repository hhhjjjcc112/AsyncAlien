use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::sync::atomic::AtomicUsize;

use arch::sfence_vma_all;
use config::FRAME_BITS;
use config::{FRAME_SIZE, KERNEL_HEAP_SIZE, TRAMPOLINE};
#[cfg(target_arch = "x86_64")]
use config::{LOW_PHYS_MAP_BASE, LOW_PHYS_MAP_SIZE, PERCPU_MIRROR_BASE};
use ksync::RwLock;
use log::info;
use page_table::MappingFlags;
use platform::{config::DEVICE_SPACE, MemIf, Platform, println};
use ptable::{PhysPage, VmArea, VmAreaEqual, VmAreaType, VmSpace};
use spin::Lazy;

use super::{alloc_frame_trackers, AlienResult};
use crate::frame::{FrameTracker, VmmPageAllocator};

pub static KERNEL_SPACE: Lazy<Arc<RwLock<VmSpace<VmmPageAllocator>>>> =
    Lazy::new(|| Arc::new(RwLock::new(VmSpace::<VmmPageAllocator>::new())));

unsafe extern "C" {
    fn stext();
    fn srodata();
    fn sdata();
    fn sbss();
    fn sheap();
    fn sinit();
    fn einit();

    #[cfg(target_arch = "x86_64")]
    fn _percpu_start();
    #[cfg(target_arch = "x86_64")]
    fn _percpu_end();

    // fn kernel_eh_frame();
    // fn kernel_eh_frame_end();
    // fn kernel_eh_frame_hdr();
    // fn kernel_eh_frame_hdr_end();
}

#[cfg(target_arch = "x86_64")]
fn map_percpu_mirror(kernel_space: &mut VmSpace<VmmPageAllocator>) {
    let percpu_start = _percpu_start as *const () as usize;
    let percpu_end = _percpu_end as *const () as usize;
    if percpu_end <= percpu_start {
        return;
    }
    let size = (percpu_end - percpu_start + FRAME_SIZE - 1) & !(FRAME_SIZE - 1);
    let pages = size / FRAME_SIZE;

    let mut frames: Vec<Box<dyn PhysPage>> = Vec::with_capacity(pages);
    for i in 0..pages {
        let paddr = percpu_start + i * FRAME_SIZE;
        frames.push(Box::new(FrameTracker::new(paddr >> FRAME_BITS, 1, false)));
    }

    let percpu_area = VmArea::new(
        PERCPU_MIRROR_BASE..PERCPU_MIRROR_BASE + size,
        MappingFlags::READ | MappingFlags::WRITE,
        frames,
    );
    kernel_space.map(VmAreaType::VmArea(percpu_area)).unwrap();
}

#[cfg(target_arch = "x86_64")]
fn map_low_phys_window(kernel_space: &mut VmSpace<VmmPageAllocator>) {
    let pages = LOW_PHYS_MAP_SIZE / FRAME_SIZE;
    let mut frames: Vec<Box<dyn PhysPage>> = Vec::with_capacity(pages);
    for page in 0..pages {
        frames.push(Box::new(FrameTracker::new(page, 1, false)));
    }

    // 供 ACPI/AML 访问低端物理内存（如 EBDA/BDA），避免和空指针地址重叠。
    let low_phys_area = VmArea::new(
        LOW_PHYS_MAP_BASE..LOW_PHYS_MAP_BASE + LOW_PHYS_MAP_SIZE,
        MappingFlags::READ | MappingFlags::WRITE,
        frames,
    );
    kernel_space.map(VmAreaType::VmArea(low_phys_area)).unwrap();
}

#[cfg(target_arch = "x86_64")]
fn map_acpi_reserved_tail(kernel_space: &mut VmSpace<VmmPageAllocator>) {
    const ACPI_TAIL_MAP_SIZE: usize = 0x20_0000;
    const PCI_ECAM_BASE: usize = 0xb000_0000;

    let max_ram_end = Platform::phys_ram_ranges()
        .iter()
        .map(|(start, size)| start.saturating_add(*size))
        .max()
        .unwrap_or(0);
    if max_ram_end == 0 {
        return;
    }

    let start = (max_ram_end + FRAME_SIZE - 1) & !(FRAME_SIZE - 1);
    if start >= PCI_ECAM_BASE {
        return;
    }
    let mut end = start.saturating_add(ACPI_TAIL_MAP_SIZE);
    if end > PCI_ECAM_BASE {
        end = PCI_ECAM_BASE;
    }
    end &= !(FRAME_SIZE - 1);
    if end <= start {
        return;
    }

    let pages = (end - start) / FRAME_SIZE;
    let mut frames: Vec<Box<dyn PhysPage>> = Vec::with_capacity(pages);
    for page in 0..pages {
        let paddr = start + page * FRAME_SIZE;
        frames.push(Box::new(FrameTracker::new(paddr >> FRAME_BITS, 1, false)));
    }

    // QEMU 常把 ACPI reclaim/NVS 放在可用内存尾部上方，这里补一段只读写映射供 ACPI 解析。
    let tail_area = VmArea::new(start..end, MappingFlags::READ | MappingFlags::WRITE, frames);
    kernel_space.map(VmAreaType::VmArea(tail_area)).unwrap();
}

pub fn kernel_info() -> usize {
    let heap_start = sheap as *const () as usize;
    let heap_end = heap_start + KERNEL_HEAP_SIZE;
    println!(
        "kernel text:          {:#x}-{:#x}",
        stext as *const () as usize, srodata as *const () as usize
    );
    println!(
        "kernel rodata:        {:#x}-{:#x}",
        srodata as *const () as usize, sdata as *const () as usize
    );
    println!(
        "kernel init_array:    {:#x}-{:#x}",
        sinit as *const () as usize, einit as *const () as usize
    );
    println!(
        "kernel data:          {:#x}-{:#x}",
        sdata as *const () as usize, sbss as *const () as usize
    );
    println!(
        "kernel bss:           {:#x}-{:#x}",
        sbss as *const () as usize, sheap as *const () as usize
    );
    // println!("kernel eh_frame:      {:#x}-{:#x}", kernel_eh_frame as usize, kernel_eh_frame_end as usize);
    // println!("kernel eh_frame_hdr:  {:#x}-{:#x}", kernel_eh_frame_hdr as usize, kernel_eh_frame_hdr_end as usize);
    println!("kernel heap:          {:#x}-{:#x}", heap_start, heap_end);
    for &(start, size) in Platform::alloc_ranges() {
        println!("kernel alloc range:   {:#x}-{:#x}", start, start + size);
    }
    sheap as *const () as usize
}

static KERNEL_MAP_MAX: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "memory_self_test")]
#[path = "../tests/memory_self_test.rs"]
mod memory_self_test;

#[cfg(feature = "memory_self_test")]
pub use memory_self_test::{verify_kernel_page_table_activated, verify_kernel_page_table_mappings};

pub fn build_kernel_address_space() {
    kernel_info();
    let mut kernel_space = KERNEL_SPACE.write();
    let heap_start = sheap as *const () as usize;
    let heap_end = heap_start + KERNEL_HEAP_SIZE;
    let text_area = VmAreaEqual::new(
        stext as *const () as _..srodata as *const () as _,
        MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::WRITE,
    );
    let rodata_area = VmAreaEqual::new(srodata as *const () as _..sdata as *const () as _, MappingFlags::READ);
    let sdata_area = VmAreaEqual::new(
        sdata as *const () as _..sbss as *const () as _,
        MappingFlags::READ | MappingFlags::WRITE,
    );
    let sbss_area = VmAreaEqual::new(
        sbss as *const () as _..sheap as *const () as _,
        MappingFlags::READ | MappingFlags::WRITE,
    );
    let heap_area = VmAreaEqual::new(
        heap_start..heap_end,
        MappingFlags::READ | MappingFlags::WRITE,
    );

    let trampoline_area = VmArea::new(
        TRAMPOLINE..(TRAMPOLINE + FRAME_SIZE),
        MappingFlags::READ | MappingFlags::EXECUTE,
        vec![Box::new(FrameTracker::create_trampoline())],
    );
    kernel_space
        .map(VmAreaType::VmAreaEqual(text_area))
        .unwrap();
    kernel_space
        .map(VmAreaType::VmAreaEqual(rodata_area))
        .unwrap();
    kernel_space
        .map(VmAreaType::VmAreaEqual(sdata_area))
        .unwrap();
    kernel_space
        .map(VmAreaType::VmAreaEqual(sbss_area))
        .unwrap();
    kernel_space
        .map(VmAreaType::VmAreaEqual(heap_area))
        .unwrap();
    kernel_space
        .map(VmAreaType::VmArea(trampoline_area))
        .unwrap();

    #[cfg(target_arch = "x86_64")]
    {
        map_percpu_mirror(&mut kernel_space);
        map_low_phys_window(&mut kernel_space);
        map_acpi_reserved_tail(&mut kernel_space);
    }

    let mut map_max = heap_end;
    for &(start, size) in Platform::alloc_ranges() {
        if size == 0 {
            continue;
        }
        let end = start + size;
        kernel_space
            .map(VmAreaType::VmAreaEqual(VmAreaEqual::new(
                start..end,
                MappingFlags::READ | MappingFlags::WRITE,
            )))
            .unwrap();
        if end > map_max {
            map_max = end;
        }
    }

    for pair in DEVICE_SPACE {
        let io_area = VmAreaEqual::new(
            pair.1..pair.1 + pair.2,
            MappingFlags::READ | MappingFlags::WRITE,
        );
        kernel_space.map(VmAreaType::VmAreaEqual(io_area)).unwrap();
        println!("map {}: {:#x?}-{:#x?}", pair.0, pair.1, pair.1 + pair.2);
        map_max = map_max.max(pair.1 + pair.2);
    }
    KERNEL_MAP_MAX.store(map_max, core::sync::atomic::Ordering::SeqCst);
}

/// 返回根页表物理地址。
pub fn kernel_page_table_root_paddr() -> usize {
    KERNEL_SPACE.read().root_paddr()
}

pub fn kernel_page_table_token() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        kernel_page_table_root_paddr()
    }
    #[cfg(target_arch = "riscv64")]
    {
        8usize << 60 | (kernel_page_table_root_paddr() >> FRAME_BITS)
    }
}

pub fn query_kernel_space(addr: usize) -> Option<usize> {
    let kernel_space = KERNEL_SPACE.read();
    kernel_space
        .query(addr)
        .ok()
        .map(|(phy_addr, _, _)| phy_addr.as_usize())
}

/// 内核栈布局（高地址到低地址）：
/// 跳板页 -> 保护页 -> 栈区 -> 保护页。
pub fn map_kstack_for_task(task_id: usize, pages: usize) -> AlienResult<usize> {
    let kstack_base = TRAMPOLINE - (task_id + 1) * (pages + 1) * FRAME_SIZE;
    let kstack_lower = kstack_base + FRAME_SIZE;
    let kstack_upper = kstack_lower + pages * FRAME_SIZE;
    let mut phy_frames: Vec<Box<dyn PhysPage>> = vec![];
    for _ in 0..pages {
        phy_frames.push(Box::new(crate::alloc_frame_trackers(1)));
    }
    let kstack_area = VmArea::new(
        kstack_lower..kstack_upper,
        MappingFlags::READ | MappingFlags::WRITE,
        phy_frames,
    );
    let mut kernel_space = KERNEL_SPACE.write();
    kernel_space.map(VmAreaType::VmArea(kstack_area)).unwrap();
    info!(
        "task {} kstack: {:#x?}-{:#x?}",
        task_id, kstack_lower, kstack_upper
    );
    Ok(kstack_upper)
}

pub fn unmap_kstack_for_task(task_id: usize, pages: usize) -> AlienResult<()> {
    let kstack_base = TRAMPOLINE - (task_id + 1) * (pages + 1) * FRAME_SIZE;
    let kstack_lower = kstack_base + FRAME_SIZE;
    let kstack_upper = kstack_lower + pages * FRAME_SIZE;
    let mut kernel_space = KERNEL_SPACE.write();
    kernel_space
        .unmap(kstack_lower)
        .unwrap_or_else(|_| panic!("unmap kstack failed, task_id:{}", task_id));
    info!(
        "unmap task {} kstack: {:#x?}-{:#x?}",
        task_id, kstack_lower, kstack_upper
    );
    Ok(())
}

#[derive(Debug)]
pub struct VirtDomainArea {
    start: usize,
    size: usize,
}

impl VirtDomainArea {
    pub(super) fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.start as *mut u8
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.start as *const u8, self.size) }
    }
    pub fn as_mut_slice(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.start as *mut u8, self.size) }
    }
    pub fn len(&self) -> usize {
        self.size
    }
}

pub fn map_domain_region(size: usize) -> VirtDomainArea {
    assert_eq!(size % FRAME_SIZE, 0);
    let virt_start = KERNEL_MAP_MAX.fetch_add(size, core::sync::atomic::Ordering::Relaxed);
    // 分配物理页并映射到内核虚拟地址。
    log::error!(
        "[alloc_free_module_region] virt_start: {:#x}, size: {:#x}",
        virt_start,
        size
    );
    let mut phy_frames: Vec<Box<dyn PhysPage>> = vec![];
    for _ in 0..size / FRAME_SIZE {
        let frame = Box::new(alloc_frame_trackers(1));
        phy_frames.push(frame);
    }
    let mut kernel_space = KERNEL_SPACE.write();
    let vm_area = VmArea::new(
        virt_start..virt_start + size,
        MappingFlags::READ | MappingFlags::WRITE,
        phy_frames,
    );
    kernel_space.map(VmAreaType::VmArea(vm_area)).unwrap();
    // 刷新 TLB。
    sfence_vma_all();
    VirtDomainArea::new(virt_start, size)
}

pub fn unmap_domain_area(area: VirtDomainArea) {
    let mut kernel_space = KERNEL_SPACE.write();
    kernel_space.unmap(area.start).unwrap();
    sfence_vma_all();
}

pub fn set_memory_x(virt_addr: usize, numpages: usize) -> AlienResult<()> {
    let mut kernel_space = KERNEL_SPACE.write();
    // kernel_space.set_flags(virt_addr, numpages, MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE).unwrap();
    let mut addr = virt_addr;
    for _ in 0..numpages {
        kernel_space
            .protect(
                addr..addr + FRAME_SIZE,
                MappingFlags::READ | MappingFlags::EXECUTE,
            )
            .unwrap();
        addr += FRAME_SIZE;
    }
    // 刷新 TLB。
    sfence_vma_all();
    Ok(())
}
