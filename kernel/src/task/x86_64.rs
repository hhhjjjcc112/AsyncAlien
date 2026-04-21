use core::arch::global_asm;

use basic::task::TaskContext;
use x86_64::{
    registers::model_specific::{FsBase, KernelGsBase},
    VirtAddr,
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
        // 任务迁移到别核时，FS/GS 必须跟随任务上下文恢复，不能沿用源核寄存器残值。
        (*now).save_fp_simd();
        (*next).restore_fp_simd();

        (*now).save_fsgs();
        (*next).restore_fsgs();

        __switch(now, next);
    }
}

// 更新fs寄存器和task上下文中的fs
#[inline]
pub fn set_current_user_fs_base(fs_base: usize) -> AlienResult<()> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    guard.task_context().set_fs_base(fs_base);
    FsBase::write(VirtAddr::new(fs_base as u64));
    Ok(())
}

// 直接读fs寄存器值就行
#[inline]
pub fn current_user_fs_base() -> AlienResult<usize> {
    Ok(FsBase::read().as_u64() as usize)
}

// 更新gs寄存器和task上下文中的gs
#[inline]
pub fn set_current_user_gs_base(gs_base: usize) -> AlienResult<()> {
    let task = current_task().ok_or(AlienError::EINVAL)?;
    let mut guard = task.lock();
    guard.task_context().set_gs_base(gs_base);
    // 因为有swapgs，用户态gs在内核态被换到KernelGsBase里了
    KernelGsBase::write(VirtAddr::new(gs_base as u64));
    Ok(())
}

// 直接读kernel_gs寄存器值就行
#[inline]
pub fn current_user_gs_base() -> AlienResult<usize> {
    Ok(KernelGsBase::read().as_u64() as usize)
}
