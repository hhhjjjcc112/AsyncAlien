use core::arch::asm;

#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mov rdi, rsp
            call _start_rust",
        )
    }
}

pub(crate) fn syscall(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        // x86_64 syscall ABI：rax=号，rdi/rsi/rdx/r10/r8/r9=参数，rcx/r11 会被硬件覆盖。
        asm!(
            "syscall",
            inlateout("rax") id as isize => ret,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("r10") args[3],
            in("r8") args[4],
            in("r9") args[5],
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }
    ret
}
