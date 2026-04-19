use core::arch::global_asm;

use basic::task::TaskContext;
use percpu::read_percpu_reg;
use platform::percpu_impl::cpu_id;
use x86_64::{
    registers::model_specific::{FsBase, GsBase, KernelGsBase},
    VirtAddr,
};

use crate::{
    error::{AlienError, AlienResult},
    task::{current_task, current_tid},
};

global_asm!(include_str!("switch_x86_64.asm"));

unsafe extern "C" {
    fn __switch(now: *mut TaskContext, next: *const TaskContext);
    fn x86_read_tss_rsp0() -> usize;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct X86StateTrace {
    pub trap_frame_phy: usize,
    pub trap_cx_ptr: usize,
    pub user_cr3: usize,
    pub kernel_cr3: usize,
    pub kernel_sp: usize,
    pub rip: usize,
    pub rsp: usize,
    pub vector: usize,
    pub error: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct CurrentTaskTrace {
    tid: Option<usize>,
    ctx_kstack: usize,
    ctx_fs: usize,
    ctx_gs: usize,
}

impl X86StateTrace {
    #[inline]
    pub fn from_frame(
        frame: &basic::task::TrapFrame,
        trap_frame_phy: usize,
        trap_cx_ptr: usize,
        user_cr3: usize,
    ) -> Self {
        Self {
            trap_frame_phy,
            trap_cx_ptr,
            user_cr3,
            kernel_cr3: frame.kernel_page_table_token(),
            kernel_sp: frame.kernel_sp().as_usize(),
            rip: frame.rip,
            rsp: frame.rsp,
            vector: frame.vector,
            error: frame.error_code,
        }
    }
}

fn current_task_snapshot() -> CurrentTaskTrace {
    current_task()
        .map(|task| {
            let mut guard = task.lock();
            let tid = guard.tid();
            let ctx = guard.task_context();
            CurrentTaskTrace {
                tid: Some(tid),
                ctx_kstack: ctx.kstack_top(),
                ctx_fs: ctx.fs_base(),
                ctx_gs: ctx.gs_base(),
            }
        })
        .unwrap_or(CurrentTaskTrace {
            tid: current_tid(),
            ..CurrentTaskTrace::default()
        })
}

#[inline]
fn tss_rsp0() -> usize {
    unsafe { x86_read_tss_rsp0() }
}

#[inline]
pub fn should_trace_tid(tid: Option<usize>) -> bool {
    matches!(tid, Some(2))
}

#[inline]
pub fn should_trace_task(tid: usize) -> bool {
    should_trace_tid(Some(tid))
}

pub fn trace_current_state(label: &str, trace: X86StateTrace) {
    let task = current_task_snapshot();
    println!(
        "[x86 state] label={} cpu={} tid={:?} trap_frame_phy={:#x} trap_cx={:#x} rip={:#x} rsp={:#x} rsp0={:#x} user_cr3={:#x} kernel_cr3={:#x} kernel_sp={:#x} fs={:#x} gs={:#x} kgs={:#x} percpu={:#x} ctx_kstack={:#x} ctx_fs={:#x} ctx_gs={:#x} vec={:#x} err={:#x}",
        label,
        cpu_id(),
        task.tid,
        trace.trap_frame_phy,
        trace.trap_cx_ptr,
        trace.rip,
        trace.rsp,
        tss_rsp0(),
        trace.user_cr3,
        trace.kernel_cr3,
        trace.kernel_sp,
        FsBase::read().as_u64() as usize,
        GsBase::read().as_u64() as usize,
        KernelGsBase::read().as_u64() as usize,
        read_percpu_reg(),
        task.ctx_kstack,
        task.ctx_fs,
        task.ctx_gs,
        trace.vector,
        trace.error,
    );
}

pub fn trace_task_context_state(label: &str, tid: usize, ctx: &TaskContext) {
    println!(
        "[x86 sched] label={} cpu={} tid={} rsp0={:#x} fs={:#x} gs={:#x} kgs={:#x} percpu={:#x} ctx_kstack={:#x} ctx_fs={:#x} ctx_gs={:#x}",
        label,
        cpu_id(),
        tid,
        tss_rsp0(),
        FsBase::read().as_u64() as usize,
        GsBase::read().as_u64() as usize,
        KernelGsBase::read().as_u64() as usize,
        read_percpu_reg(),
        ctx.kstack_top(),
        ctx.fs_base(),
        ctx.gs_base(),
    );
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
