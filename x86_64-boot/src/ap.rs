use core::arch::global_asm;

const AP_START_PAGE_IDX: usize = 6;
const AP_START_PAGE_ADDR: usize = AP_START_PAGE_IDX * 0x1000;

global_asm!(
    include_str!("ap_start.S"),
    start_page_paddr = const AP_START_PAGE_ADDR,
);

fn setup_startup_page(stack_top: usize) {
    
}