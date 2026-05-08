//! x86_64 从核启动流程。

use core::{
    arch::global_asm,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use config::{CPU_NUM, LOW_PHYS_MAP_BASE};

use crate::common_x86_64::{
    apic::{get_local_apic, is_x2apic},
    boot::BOOT_STACK_SIZE,
    time::busy_wait,
};

/// AP 启动页索引（0x8000）。
const AP_START_PAGE_IDX: usize = 8;
/// AP 启动页物理地址。
const AP_START_PAGE_ADDR: usize = AP_START_PAGE_IDX * 0x1000;
/// AP 启动页大小。
const AP_START_PAGE_SIZE: usize = 0x1000;
/// 等待从核进入早期入口的最长轮询次数。
const AP_BOOT_WAIT_RETRIES: usize = 200;

/// 支持的最大 CPU 数。
const MAX_CPUS: usize = 32;

/// AP 启动栈区。
#[unsafe(link_section = ".bss.stack")]
static mut AP_STARTUP_STACKS: [u8; BOOT_STACK_SIZE * MAX_CPUS] = [0; BOOT_STACK_SIZE * MAX_CPUS];
/// 从核进入 `secondary_entry` 的计数，BSP 依此判断是否可复写 trampoline 页。
static AP_EARLY_BOOT_COUNT: AtomicUsize = AtomicUsize::new(0);

// 引入 AP 启动汇编。
global_asm!(
    include_str!("ap_start.S"),
    start_page_paddr = const AP_START_PAGE_ADDR,
);

unsafe extern "C" {
    fn ap_start();
    fn ap_end();
    fn ap_entry32();
}

/// 设置 AP 启动页代码与栈顶。
fn setup_startup_page(_cpu_id: usize, stack_top: usize) {
    let start_page_vaddr = LOW_PHYS_MAP_BASE + AP_START_PAGE_ADDR;
    let start_page = unsafe {
        core::slice::from_raw_parts_mut(
            start_page_vaddr as *mut u64,
            AP_START_PAGE_SIZE / 8,
        )
    };
    let image_size = ap_end as *const () as usize - ap_start as *const () as usize;
    assert!(
        image_size <= AP_START_PAGE_SIZE,
        "AP trampoline too large: {} bytes",
        image_size
    );

    // 将 AP 启动代码拷到低地址页。
    unsafe {
        core::ptr::copy_nonoverlapping(
            ap_start as *const u8,
            start_page.as_mut_ptr() as *mut u8,
            image_size,
        )
    }

    // 在页尾写入 AP 入口和栈顶。
    start_page[AP_START_PAGE_SIZE / 8 - 2] = stack_top as u64;
    start_page[AP_START_PAGE_SIZE / 8 - 1] = ap_entry32 as *const () as usize as u64;
}

/// 从核进入 `secondary_entry` 后尽快上报，避免 BSP 复写仍在使用的 trampoline 页。
pub fn announce_secondary_cpu_started() {
    AP_EARLY_BOOT_COUNT.fetch_add(1, Ordering::AcqRel);
}

fn wait_for_secondary_early_boot(_cpu_id: usize, expected_count: usize) -> bool {
    for _ in 0..AP_BOOT_WAIT_RETRIES {
        if AP_EARLY_BOOT_COUNT.load(Ordering::Acquire) >= expected_count {
            return true;
        }
        busy_wait(Duration::from_millis(1));
    }
    false
}

pub fn boot_secondary_cpu(cpu_id: usize) -> bool {
    if cpu_id >= MAX_CPUS || cpu_id >= CPU_NUM {
        return false;
    }

    #[allow(static_mut_refs)]
    let stack_top = unsafe { AP_STARTUP_STACKS.as_ptr() } as usize + (cpu_id + 1) * BOOT_STACK_SIZE;
    setup_startup_page(cpu_id, stack_top);

    let mut guard = get_local_apic().expect("APIC not initialized");
    let apic = guard.as_mut().expect("APIC context missing");
    let target_apic_id = if is_x2apic() {
        cpu_id as u32
    } else {
        (cpu_id << 24) as u32
    };
    let expected_count = AP_EARLY_BOOT_COUNT.load(Ordering::Acquire) + 1;

    apic.send_init_ipi(target_apic_id).ok();
    busy_wait(Duration::from_millis(10));
    apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id).ok();
    busy_wait(Duration::from_micros(200));
    apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id).ok();

    wait_for_secondary_early_boot(cpu_id, expected_count)
}
