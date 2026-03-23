use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};

static BREAKPOINT_HIT: AtomicBool = AtomicBool::new(false);

pub fn record_breakpoint(rip: usize) {
    BREAKPOINT_HIT.store(true, Ordering::Release);
    println!("[trap_self_test] hit breakpoint at RIP={:#x}", rip);
}

pub fn run() {
    BREAKPOINT_HIT.store(false, Ordering::Release);
    println!("[trap_self_test] trigger breakpoint");
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
    assert!(
        BREAKPOINT_HIT.load(Ordering::Acquire),
        "trap_self_test: breakpoint handler was not reached"
    );
    println!("[trap_self_test] breakpoint self-test passed");
}