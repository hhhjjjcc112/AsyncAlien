use core::arch::asm;

/// x86-64 trap frame pushed by CPU and trap handler
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct X86TrapFrame {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub rbp: usize,
    pub rbx: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rax: usize,
    pub vector: usize,
    pub error_code: usize,
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize,
}

impl X86TrapFrame {
    #[inline]
    pub fn is_user(&self) -> bool {
        (self.cs & 0x3) == 3
    }

    #[inline]
    pub fn is_kernel(&self) -> bool {
        (self.cs & 0x3) == 0
    }

    #[inline]
    pub fn fault_address() -> usize {
        let addr: usize;
        unsafe {
            asm!("mov {}, cr2", out(reg) addr, options(nomem, nostack, preserves_flags));
        }
        addr
    }
}
