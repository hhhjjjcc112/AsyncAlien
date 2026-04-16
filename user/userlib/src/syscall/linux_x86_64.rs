use pconst::{io::PollFd, *};

use super::syscall;
use crate::syscall;

// 仅 x86_64 文档存在的兼容 Linux syscall。
syscall!(sys_open, SYSCALL_OPEN, *const u8, usize, usize);
syscall!(sys_stat, SYSCALL_STAT, *const u8, *mut u8);
syscall!(sys_lstat, SYSCALL_LSTAT, *const u8, *mut u8);
syscall!(sys_access, SYSCALL_ACCESS, *const u8, usize);
syscall!(sys_pipe, SYSCALL_PIPE, *mut u32);
syscall!(sys_poll, SYSCALL_POLL, *mut PollFd, usize, i32);
syscall!(sys_select, SYSCALL_SELECT, usize, usize, usize, usize, usize);
syscall!(sys_mkdir, SYSCALL_MKDIR, *const u8, usize);
syscall!(sys_fork, SYSCALL_FORK);
syscall!(sys_vfork, SYSCALL_VFORK);
syscall!(sys_getpgrp, SYSCALL_GETPGRP);
syscall!(sys_renameat, SYSCALL_RENAMEAT, isize, *const u8, isize, *const u8);
syscall!(sys_arch_prctl, SYSCALL_ARCH_PRCTL, usize, usize);
