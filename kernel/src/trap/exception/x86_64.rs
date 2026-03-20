use super::common::{dispatch_syscall, with_current_trap_frame};

/// x86_64 系统调用异常处理。
///
/// 当前用于 `syscall` 路径（LSTAR 入口）统一分发，
/// 返回路径统一走 `trap_return -> iretq`，不单独走 `sysretq`。
pub fn syscall_exception_handler() {
    with_current_trap_frame(dispatch_syscall);
}
