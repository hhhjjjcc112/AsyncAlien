use super::common::{dispatch_syscall, with_current_trap_frame};

/// RISC-V 系统调用异常处理。
///
/// `ecall` 返回前需要手动前移 `sepc`，避免回到同一条指令重复陷入。
pub fn syscall_exception_handler() {
    with_current_trap_frame(|cx| {
        cx.update_sepc(cx.sepc() + 4);
        dispatch_syscall(cx);
    });
}
