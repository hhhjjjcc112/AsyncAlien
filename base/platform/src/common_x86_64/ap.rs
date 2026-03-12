//! x86_64 从核启动流程。

use core::{arch::global_asm, time::Duration};

use config::CPU_NUM;

use crate::common_x86_64::{
    apic::{get_local_apic, is_x2apic},
    boot::{BOOT_STACK_SIZE, PHYS_VIRT_OFFSET},
    time::busy_wait,
};

/// AP 启动页索引（0x6000）。
const AP_START_PAGE_IDX: usize = 6;
/// AP 启动页物理地址。
const AP_START_PAGE_ADDR: usize = AP_START_PAGE_IDX * 0x1000;

/// 支持的最大 CPU 数。
const MAX_CPUS: usize = 32;

/// AP 启动栈区。
#[unsafe(link_section = ".bss.stack")]
static mut AP_STARTUP_STACKS: [u8; BOOT_STACK_SIZE * MAX_CPUS] = [0; BOOT_STACK_SIZE * MAX_CPUS];

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
fn setup_startup_page(stack_top: usize) {
    let start_page = unsafe {
        core::slice::from_raw_parts_mut(
            (AP_START_PAGE_ADDR + PHYS_VIRT_OFFSET as usize) as *mut u64,
            0x1000 / 8,
        )
    };

    // 将 AP 启动代码拷到低地址页。
    unsafe {
        core::ptr::copy_nonoverlapping(
            ap_start as *const u8,
            start_page.as_mut_ptr() as *mut u8,
            ap_end as *const () as usize - ap_start as *const () as usize,
        )
    }

    // 在页尾写入 AP 入口和栈顶。
    start_page[0x1000 / 8 - 2] = stack_top as u64;
    start_page[0x1000 / 8 - 1] = ap_entry32 as *const () as usize as u64;
}

/// 获取逻辑 CPU 数量。
pub fn cpu_num() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(1, |finfo| finfo.max_logical_processor_ids() as usize)
}

/// 启动全部 AP。
pub fn start_aps() {
    let num_cpus = cpu_num().min(MAX_CPUS).min(CPU_NUM);
    log::info!("Starting {} APs...", num_cpus.saturating_sub(1));

    let apic = unsafe { get_local_apic() };

    for cpu_id in 1..num_cpus {
        #[allow(static_mut_refs)]
        let stack_top =
            unsafe { AP_STARTUP_STACKS.as_ptr() } as usize + cpu_id * BOOT_STACK_SIZE;
        setup_startup_page(stack_top);

        log::debug!("Starting CPU {}", cpu_id);

        let target_apic_id = if is_x2apic() {
            cpu_id as u32
        } else {
            (cpu_id << 24) as u32
        };

        unsafe {
            // 发送 INIT IPI。
            apic.send_init_ipi(target_apic_id);
            busy_wait(Duration::from_millis(10)); // 10ms

            // 发送 SIPI（两次提高可靠性）。
            apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id);
            busy_wait(Duration::from_micros(200)); // 200us
            apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id);
        }

        // 等待 AP 拉起。
        busy_wait(Duration::from_millis(10));
    }
}

/// 启动指定从核。
pub fn start_secondary_cpu(cpu_id: usize, _start_addr: usize, _opaque: usize) -> (isize, isize) {
    if cpu_id >= MAX_CPUS || cpu_id >= CPU_NUM {
        return (-1, 0);
    }

    #[allow(static_mut_refs)]
    let stack_top = unsafe { AP_STARTUP_STACKS.as_ptr() } as usize + cpu_id * BOOT_STACK_SIZE;
    setup_startup_page(stack_top);

    let apic = unsafe { get_local_apic() };
    let target_apic_id = if is_x2apic() {
        cpu_id as u32
    } else {
        (cpu_id << 24) as u32
    };

    unsafe {
        apic.send_init_ipi(target_apic_id);
        busy_wait(Duration::from_millis(10));
        apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id);
        busy_wait(Duration::from_micros(200));
        apic.send_sipi(AP_START_PAGE_IDX as u8, target_apic_id);
    }

    (0, 0)
}
