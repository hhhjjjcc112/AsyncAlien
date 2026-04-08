#[cfg(target_arch = "x86_64")]
use arch::read_cr3;

use super::{kernel_page_table_root_paddr, KERNEL_SPACE};
use platform::MemIf;

#[inline]
fn mark_pass(stage: &str) {
    super::println!("[memory_test] pass: {}", stage);
}

#[inline]
fn assert_addr_mapped(addr: usize, tag: &str) {
    assert!(
        KERNEL_SPACE.read().query(addr).is_ok(),
        "kernel page table check failed: {} addr={:#x}",
        tag,
        addr
    );
}

#[inline]
fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn assert_range_mapped(start: usize, end: usize, tag: &str) {
    if end <= start {
        return;
    }
    // 采样检查首尾页，覆盖常见映射错误。
    assert_addr_mapped(start, tag);
    let last = align_down(end - 1, super::FRAME_SIZE);
    assert_addr_mapped(last, tag);
}

/// 内核页表建立后自检：验证关键区间映射是否可查询。
pub fn verify_kernel_page_table_mappings() {
    mark_pass("start verify_kernel_page_table_mappings");
    let text_start = super::stext as *const () as usize;
    let rodata_start = super::srodata as *const () as usize;
    let data_start = super::sdata as *const () as usize;
    let bss_start = super::sbss as *const () as usize;
    let heap_start = super::sheap as *const () as usize;
    let heap_end = heap_start + super::KERNEL_HEAP_SIZE;

    assert_range_mapped(text_start, rodata_start, "text");
    assert_range_mapped(rodata_start, data_start, "rodata");
    assert_range_mapped(data_start, bss_start, "data");
    assert_range_mapped(bss_start, heap_start, "bss");
    assert_range_mapped(heap_start, heap_end, "heap");
    assert_range_mapped(super::TRAMPOLINE, super::TRAMPOLINE + super::FRAME_SIZE, "trampoline");
    mark_pass("kernel sections and trampoline mapped");

    let mut alloc_range_count = 0usize;
    for &(start, size) in super::Platform::alloc_ranges() {
        assert_range_mapped(start, start + size, "alloc_range");
        alloc_range_count += 1;
    }
    super::println!("[memory_test] pass: alloc_ranges mapped, count={}", alloc_range_count);

    let mut device_count = 0usize;
    for &(_, start, size) in super::DEVICE_SPACE {
        assert_range_mapped(start, start + size, "device");
        device_count += 1;
    }
    super::println!("[memory_test] pass: device ranges mapped, count={}", device_count);

    let root = kernel_page_table_root_paddr();
    assert_eq!(root & (super::FRAME_SIZE - 1), 0, "root page table not aligned");
    mark_pass("root page table aligned");
    mark_pass("verify_kernel_page_table_mappings done");
}

/// 页表激活后自检。
pub fn verify_kernel_page_table_activated() {
    #[cfg(target_arch = "x86_64")]
    {
        // CR3 保存的是页表根物理地址。
        let cr3 = read_cr3();
        assert_eq!(
            cr3,
            kernel_page_table_root_paddr(),
            "CR3/root page table mismatch: cr3={:#x} root={:#x}",
            cr3,
            kernel_page_table_root_paddr()
        );
        mark_pass("cr3 matches kernel root page table");
    }

    mark_pass("verify_kernel_page_table_activated done");
}