mod cause;
mod entry;

pub use entry::{
    kernel_trap_vector, set_kernel_trap_entry, set_user_trap_entry, trap_return, user_trap_vector,
};

pub fn init_trap() {
    // RISC-V: 初始化并设置 stvec（含内核态和用户态入口）
    set_kernel_trap_entry();
}