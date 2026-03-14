use alloc::boxed::Box;
use core::{
    ops::{Deref, DerefMut},
    sync::atomic::AtomicUsize,
};

use config::{FRAME_BITS, FRAME_SIZE};
use heapless::Vec;
use ksync::Mutex;
use log::trace;
use memory_addr::{PhysAddr, VirtAddr};
use pager::{PageAllocator, PageAllocatorExt};
use platform::{println, RawRange};
use ptable::PhysPage;

#[cfg(target_arch = "x86_64")]
use page_table::{NotLeafPage, PagingIf, X64PTE, ENTRY_COUNT};
#[cfg(not(target_arch = "x86_64"))]
use page_table::{NotLeafPage, PagingIf, Rv64PTE, ENTRY_COUNT};

// 架构相关页表项类型别名。
#[cfg(target_arch = "x86_64")]
pub type ArchPTE = X64PTE;
#[cfg(not(target_arch = "x86_64"))]
pub type ArchPTE = Rv64PTE;

#[cfg(feature = "pager_bitmap")]
type InnerFrameAllocator = pager::Bitmap<0>;
#[cfg(feature = "pager_buddy")]
type InnerFrameAllocator = pager::Zone<12>;

const MAX_ALLOC_RANGES: usize = 16;

#[derive(Debug)]
struct AllocRangeEntry {
    start_paddr: usize,
    end_paddr: usize,
    allocator: InnerFrameAllocator,
}

struct MultiFrameAllocator {
    ranges: Vec<AllocRangeEntry, MAX_ALLOC_RANGES>,
}

impl MultiFrameAllocator {
    const fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn init_with_ranges(&mut self, ranges: &[RawRange]) {
        self.ranges.clear();
        for &(start, size) in ranges {
            let aligned_start = align_up(start, FRAME_SIZE);
            let aligned_end = align_down(start.saturating_add(size), FRAME_SIZE);
            if aligned_end <= aligned_start {
                continue;
            }

            let mut allocator = InnerFrameAllocator::new();
            allocator
                .init(aligned_start..aligned_end)
                .expect("init frame allocator range failed");
            self.ranges
                .push(AllocRangeEntry {
                    start_paddr: aligned_start,
                    end_paddr: aligned_end,
                    allocator,
                })
                .expect("too many frame allocator ranges");

            println!(
                "Frame range: {:#x}..{:#x}, pages={:#x}",
                aligned_start,
                aligned_end,
                (aligned_end - aligned_start) / FRAME_SIZE
            );
        }
        assert!(!self.ranges.is_empty(), "no usable frame allocator ranges");
    }

    fn alloc_pages(&mut self, pages: usize, align: usize) -> Option<usize> {
        for range in self.ranges.iter_mut() {
            if let Ok(start_page) = range.allocator.alloc_pages(pages, align) {
                return Some(start_page);
            }
        }
        None
    }

    fn free_pages(&mut self, start_page: usize, pages: usize) -> bool {
        let start = start_page << FRAME_BITS;
        let end = start.saturating_add(pages.saturating_mul(FRAME_SIZE));
        for range in self.ranges.iter_mut() {
            if start >= range.start_paddr && end <= range.end_paddr {
                return range.allocator.free_pages(start_page, pages).is_ok();
            }
        }
        false
    }
}

const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

static FRAME_ALLOCATOR: Mutex<MultiFrameAllocator> = Mutex::new(MultiFrameAllocator::new());

#[allow(dead_code)]
pub fn init_frame_allocator(start: usize, end: usize) {
    init_frame_allocator_ranges(core::slice::from_ref(&(start, end.saturating_sub(start))));
}

pub fn init_frame_allocator_ranges(ranges: &[RawRange]) {
    FRAME_ALLOCATOR.lock().init_with_ranges(ranges);
}

#[unsafe(no_mangle)]
pub fn alloc_frames(num: usize) -> *mut u8 {
    assert_eq!(num.next_power_of_two(), num);
    let start_page = FRAME_ALLOCATOR
        .lock()
        .alloc_pages(num, FRAME_SIZE)
        .unwrap_or_else(|| panic!("alloc {} frame failed", num));
    let start_addr = start_page << FRAME_BITS;
    start_addr as *mut u8
}

#[unsafe(no_mangle)]
pub fn free_frames(addr: *mut u8, num: usize) {
    assert_eq!(num.next_power_of_two(), num);
    let start = addr as usize >> FRAME_BITS;
    assert!(
        FRAME_ALLOCATOR.lock().free_pages(start, num),
        "free frame start:{:#x},num:{} failed",
        start,
        num
    );
}

#[derive(Debug)]
pub struct FrameTracker {
    start_page: usize,
    page_count: usize,
    dealloc: bool,
}

unsafe extern "C" {
    fn strampoline();
}

impl FrameTracker {
    pub fn new(start_page: usize, page_count: usize, dealloc: bool) -> Self {
        Self {
            start_page,
            page_count,
            dealloc,
        }
    }
    pub fn create_trampoline() -> Self {
        let trampoline_phy_addr = strampoline as *const () as usize;
        assert_eq!(trampoline_phy_addr % FRAME_SIZE, 0);
        Self {
            start_page: trampoline_phy_addr >> FRAME_BITS,
            page_count: 1,
            dealloc: false,
        }
    }

    pub fn start(&self) -> usize {
        self.start_page << FRAME_BITS
    }
}

impl PhysPage for FrameTracker {
    fn phys_addr(&self) -> PhysAddr {
        PhysAddr::from(self.start())
    }

    fn as_bytes(&self) -> &[u8] {
        self.deref()
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
    fn read_value_atomic(&self, offset: usize) -> usize {
        let ptr = self.start() + offset;
        unsafe {
            AtomicUsize::from_ptr(ptr as *mut usize).load(core::sync::atomic::Ordering::Relaxed)
        }
    }
    fn write_value_atomic(&mut self, offset: usize, value: usize) {
        let ptr = self.start() + offset;
        unsafe {
            AtomicUsize::from_ptr(ptr as *mut usize)
                .store(value, core::sync::atomic::Ordering::Relaxed)
        }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        if self.dealloc {
            trace!("drop frame tracker: {:#x?}", self);
            assert!(
                FRAME_ALLOCATOR
                    .lock()
                    .free_pages(self.start_page, self.page_count),
                "free frame start:{:#x},num:{} failed",
                self.start_page,
                self.page_count
            );
        }
    }
}

impl Deref for FrameTracker {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self.start() as *const u8, FRAME_SIZE * self.page_count)
        }
    }
}
impl DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self.start() as *mut u8, FRAME_SIZE * self.page_count)
        }
    }
}

pub fn alloc_frame_trackers(count: usize) -> FrameTracker {
    let frame = FRAME_ALLOCATOR
        .lock()
        .alloc_pages(count, FRAME_SIZE)
        .unwrap_or_else(|| panic!("alloc {} frame failed", count));
    trace!("alloc frame [{}] start page: {:#x}", count, frame);
    FrameTracker::new(frame, count, true)
}

pub struct VmmPageAllocator;

impl NotLeafPage<ArchPTE> for FrameTracker {
    fn phys_addr(&self) -> PhysAddr {
        PhysAddr::from(self.start_page << FRAME_BITS)
    }

    fn virt_addr(&self) -> VirtAddr {
        VirtAddr::from(self.start_page << FRAME_BITS)
    }

    fn zero(&self) {
        let ptr = self.start();
        unsafe {
            core::ptr::write_bytes(ptr as *mut u8, 0, self.page_count * FRAME_SIZE);
        }
    }

    fn as_pte_slice<'a>(&self) -> &'a [ArchPTE] {
        let ptr = self.start();
        unsafe { core::slice::from_raw_parts(ptr as _, ENTRY_COUNT) }
    }

    fn as_pte_mut_slice<'a>(&self) -> &'a mut [ArchPTE] {
        let ptr = self.start();
        unsafe { core::slice::from_raw_parts_mut(ptr as _, ENTRY_COUNT) }
    }
}

impl PagingIf<ArchPTE> for VmmPageAllocator {
    fn alloc_frame() -> Option<Box<dyn NotLeafPage<ArchPTE>>> {
        let frame = alloc_frame_trackers(1);
        Some(Box::new(frame))
    }
}
