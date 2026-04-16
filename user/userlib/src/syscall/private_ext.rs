use pconst::*;

use super::syscall;
use crate::syscall;

// AsyncAlien 私有扩展 syscall。
syscall!(sys_list, SYSCALL_LIST, *const u8);
syscall!(sys_create_global_bucket, SYSCALL_CREATE_GLOBAL_BUCKET, *const u8);
syscall!(sys_execute_user_func, SYSCALL_EXECUTE_USER_FUNC, *const u8, *const u8, usize, usize);
syscall!(sys_show_dbfs, SYSCALL_SHOW_DBFS);
syscall!(sys_dbfs_execute_operate, SYSCALL_EXECUTE_OPERATE, *const u8, *const u8);
syscall!(sys_framebuffer, SYSCALL_FRAMEBUFFER);
syscall!(sys_framebuffer_flush, SYSCALL_FRAMEBUFFER_FLUSH);
syscall!(sys_event, SYSCALL_EVENT_GET, *mut u64, usize);
syscall!(__system_shutdown, SYSCALL_SYSTEM_SHUTDOWN);
syscall!(sys_register_domain, SYSCALL_LOAD_DOMAIN, usize, u8, *const u8, usize);
syscall!(sys_update_domain, SYSCALL_REPLACE_DOMAIN, *const u8, usize, *const u8, usize, u8);
syscall!(sys_out_mask, 2003);
