use crate::syscall::sys_getrandom;
pub use pconst::{GRND_NONBLOCK, GRND_RANDOM};

pub fn getrandom(buf: &mut [u8]) -> isize {
    getrandom_with_flags(buf, 0)
}

pub fn getrandom_with_flags(buf: &mut [u8], flags: usize) -> isize {
    sys_getrandom(buf.as_mut_ptr(), buf.len(), flags)
}