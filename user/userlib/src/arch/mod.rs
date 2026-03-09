#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "riscv64")]
pub(crate) use riscv64::syscall;
#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64::syscall;

#[cfg(not(any(target_arch = "riscv64", target_arch = "x86_64")))]
compile_error!("userlib 目前仅支持 riscv64 与 x86_64");
