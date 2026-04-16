use core::arch::global_asm;

use basic::task::TaskContext;

global_asm!(include_str!("switch_riscv.asm"));

unsafe extern "C" {
    fn __switch(now: *mut TaskContext, next: *const TaskContext);
}

#[inline(always)]
pub fn switch(now: *mut TaskContext, next: *const TaskContext) {
    unsafe {
        __switch(now, next);
    }
}
