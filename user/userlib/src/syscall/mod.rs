use crate::arch;

mod linux_common;
mod private_ext;
#[cfg(target_arch = "riscv64")]
mod linux_riscv64;
#[cfg(target_arch = "x86_64")]
mod linux_x86_64;

fn syscall(id: usize, args: [usize; 6]) -> isize {
    arch::syscall(id, args)
}

pub use linux_common::*;
#[cfg(target_arch = "riscv64")]
pub use linux_riscv64::*;
#[cfg(target_arch = "x86_64")]
pub use linux_x86_64::*;
pub use private_ext::*;
