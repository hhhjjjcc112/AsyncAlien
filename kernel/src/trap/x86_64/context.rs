use basic::task::TrapFrame;
use x86_64::registers::control::Cr2;

pub type X86TrapFrame = TrapFrame;

pub trait X86TrapFrameExt {
    fn is_user(&self) -> bool;
}

impl X86TrapFrameExt for X86TrapFrame {
    #[inline]
    fn is_user(&self) -> bool {
        (self.cs & 0x3) == 3
    }

}

#[inline]
pub fn fault_address() -> usize {
    Cr2::read().as_u64() as usize
}
