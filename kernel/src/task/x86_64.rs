use core::arch::global_asm;

use basic::task::TaskContext;
use x86_64::{
    VirtAddr,
    registers::model_specific::{FsBase, KernelGsBase},
};

use crate::{
    error::{AlienError, AlienResult},
    task::current_task,
};

global_asm!(include_str!("switch_x86_64.asm"));

unsafe extern "C" {
    fn __switch(now: *mut TaskContext, next: *const TaskContext);
}

#[inline(always)]
pub fn switch(now: *mut TaskContext, next: *const TaskContext) {
    unsafe {
        // x86_64 仅把线程私有 TLS 状态放进 TaskContext。
        (*now).save_fp_simd();
        (*next).restore_fp_simd();

        (*now).save_fsgs();
        (*next).restore_fsgs();

        __switch(now, next);
    }
}

#[inline]
pub fn set_current_user_fs_base(fs_base: usize) -> AlienResult<()> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    guard.task_context().set_fs_base(fs_base);
    FsBase::write(VirtAddr::new(fs_base as u64));
    Ok(())
}

#[inline]
pub fn current_user_fs_base() -> AlienResult<usize> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    Ok(guard.task_context().fs_base())
}

#[inline]
pub fn set_current_user_gs_base(gs_base: usize) -> AlienResult<()> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    guard.task_context().set_gs_base(gs_base);
    KernelGsBase::write(VirtAddr::new(gs_base as u64));
    Ok(())
}

#[inline]
pub fn current_user_gs_base() -> AlienResult<usize> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    Ok(guard.task_context().gs_base())
}
