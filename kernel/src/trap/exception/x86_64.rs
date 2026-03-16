use super::common::{dispatch_syscall, with_current_trap_frame};

/// x86_64 系统调用异常处理。
///
/// 当前走 `int 0x80` 路径，CPU 压栈的 RIP 已经是下一条指令，
/// 因此这里不能像 RISC-V 一样额外前移返回地址。
pub fn syscall_exception_handler() {
    with_current_trap_frame(dispatch_syscall);
}
