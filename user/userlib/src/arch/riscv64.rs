use core::arch::asm;

#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mv a0,sp
            call _start_rust",
        )
    }
}

pub(crate) fn syscall(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x17") id,
        );
    }
    ret
}
