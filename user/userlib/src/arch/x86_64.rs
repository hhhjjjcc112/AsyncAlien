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

#[cfg(not(int80_syscall))]
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

#[cfg(int80_syscall)]
pub(crate) fn syscall(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        // int 0x80 走 legacy x86 约定：rax=号，rbx/rcx/rdx/rsi/rdi/rbp=参数。
        asm!(
            "push rbx",
            "push rbp",
            "mov rbx, {arg0}",
            "mov rcx, {arg1}",
            "mov rdx, {arg2}",
            "mov rsi, {arg3}",
            "mov rdi, {arg4}",
            "mov rbp, {arg5}",
            "int 0x80",
            "pop rbp",
            "pop rbx",
            arg0 = in(reg) args[0],
            arg1 = in(reg) args[1],
            arg2 = in(reg) args[2],
            arg3 = in(reg) args[3],
            arg4 = in(reg) args[4],
            arg5 = in(reg) args[5],
            inlateout("rax") id as isize => ret,
        );
    }
    ret
}
