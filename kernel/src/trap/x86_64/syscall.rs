use arch::{read_msr, write_msr, MSR_EFER, MSR_KERNEL_GS_BASE};
use core::arch::global_asm;

global_asm!(include_str!("syscall.asm"));

// x86_64 syscall 相关 MSR
const MSR_STAR: u32 = 0xC000_0081;
const MSR_LSTAR: u32 = 0xC000_0082;
const MSR_FMASK: u32 = 0xC000_0084;
const EFER_SCE: u64 = 1 << 0;

const GDT_KERNEL_CODE: u64 = 0x08;
const GDT_USER_CODE: u64 = 0x20;

unsafe extern "C" {
    fn syscall_entry();
}

#[unsafe(no_mangle)]
pub extern "C" fn x86_syscall_handler() {
    super::super::exception::syscall_exception_handler();
}

/// 处理当前 x86_64 的系统调用入口。
///
/// 现阶段用户态系统调用仍走 `int 0x80`，因此直接复用
/// x86_64 专用的 trap-frame 分发逻辑。
pub fn handle_legacy_syscall() {
    super::super::exception::syscall_exception_handler();
}

/// 初始化 x86_64 syscall 相关寄存器。
///
/// 当前用户态主路径为 syscall/sysretq，int 0x80 仅保留兼容入口。
pub fn init_syscall_registers() {
    let star = (GDT_USER_CODE << 48) | (GDT_KERNEL_CODE << 32);
    let lstar = syscall_entry as *const() as usize as u64;

    // 屏蔽 TF/IF/DF/IOPL/NT/AC，避免带入用户态标志位。
    let sfmask = (1u64 << 8)
        | (1u64 << 9)
        | (1u64 << 10)
        | (1u64 << 12)
        | (1u64 << 13)
        | (1u64 << 14)
        | (1u64 << 18);

    unsafe {
        write_msr(MSR_STAR, star);
        write_msr(MSR_LSTAR, lstar);
        write_msr(MSR_FMASK, sfmask);
        write_msr(MSR_KERNEL_GS_BASE, 0);

        let mut efer = read_msr(MSR_EFER);
        efer |= EFER_SCE;
        write_msr(MSR_EFER, efer);
    }
}
