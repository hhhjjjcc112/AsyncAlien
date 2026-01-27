use core::{arch::global_asm, char::MAX, time::Duration};

use crate::{apic::{get_local_apic, is_x2apic}, boot::{BOOT_STACK_SIZE, PHYS_VIRT_OFFSET}, println, time::busy_wait};

const AP_START_PAGE_IDX: usize = 6;
const AP_START_PAGE_ADDR: usize = AP_START_PAGE_IDX * 0x1000;

const MAX_CPUS: usize = 32;
#[unsafe(link_section = ".bss.stack")]
static mut AP_STARTUP_STACKS: [u8; BOOT_STACK_SIZE * MAX_CPUS] = [0; BOOT_STACK_SIZE * MAX_CPUS];

global_asm!(
    include_str!("ap_start.S"),
    start_page_paddr = const AP_START_PAGE_ADDR,
);

fn setup_startup_page(stack_top: usize) {
    unsafe extern "C" {
        fn ap_start();
        fn ap_end();
        fn ap_entry32();
    }

    let start_page = unsafe {
        core::slice::from_raw_parts_mut(
            (AP_START_PAGE_ADDR + PHYS_VIRT_OFFSET as usize) as *mut u64,
            0x1000 / 8,
        )
    };

    unsafe {
        core::ptr::copy_nonoverlapping(
            ap_start as *const _,
            start_page.as_mut_ptr(),
            (ap_end as usize - ap_start as usize) / 8,
        )
    }

    // 设置AP入口点和栈顶
    start_page[0x1000 / 8 - 2] = stack_top as _;
    start_page[0x1000 / 8 - 1] = ap_entry32 as usize as _;
}

fn cpu_num() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(1, |finfo| finfo.max_logical_processor_ids() as usize)
}

pub fn start_aps() {
    let num_cpus = cpu_num();
    println!("Starting {} CPUs...", num_cpus);
    let apic = unsafe { get_local_apic() };

    for cpu_id in 1..num_cpus {
        #[allow(static_mut_refs)]
        let stack_top = unsafe { AP_STARTUP_STACKS.as_ptr() } as usize
            + cpu_id * BOOT_STACK_SIZE;
        setup_startup_page(stack_top);

        println!("Starting CPU {}", cpu_id);

        let cpu_id = if is_x2apic() { cpu_id } else { cpu_id << 24 };

        unsafe { 
            apic.send_init_ipi(cpu_id as u32);
            busy_wait(Duration::from_millis(10)); // 10ms
            apic.send_sipi(AP_START_PAGE_IDX as u8, cpu_id as u32);
            busy_wait(Duration::from_micros(200)); // 200us
            apic.send_sipi(AP_START_PAGE_IDX as u8, cpu_id as u32);
        }
    }
}