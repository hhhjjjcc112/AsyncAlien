mod processor;
mod resource;
mod scheduler;

use alloc::sync::Arc;
use core::arch::global_asm;
#[cfg(target_arch = "x86_64")]
use x86_64::registers::model_specific::Msr;

use arch::cpu_id;
use basic::{sync::Once, task::TaskContext};
use config::CPU_NUM;
use interface::{SchedulerDomain, TaskDomain};
use ksync::Mutex;
pub use processor::current_tid;
pub use scheduler::{
    exit_now, get_task_priority, is_task_exit, remove_task, set_task_priority, wait_now,
    wake_up_wait_task, yield_now,
};
use task_meta::{TaskMeta, TaskStatus};

use crate::{
    error::AlienResult,
    task::{processor::current_task, resource::TaskMetaExt, scheduler::TASK_WAIT_QUEUE},
};

// Architecture-specific task switch assembly
#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("switch_riscv.asm"));
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("switch_x86_64.asm"));

unsafe extern "C" {
    fn __switch(now: *mut TaskContext, next: *const TaskContext);
}

#[inline(always)]
#[cfg(target_arch = "riscv64")]
pub fn switch(now: *mut TaskContext, next: *const TaskContext) {
    unsafe {
        __switch(now, next);
    }
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn switch(now: *mut TaskContext, next: *const TaskContext) {
    unsafe {
        const IA32_FS_BASE: u32 = 0xC000_0100;
        const IA32_KERNEL_GS_BASE: u32 = 0xC000_0102;

        // FP/SIMD 状态在 Rust 路径保存恢复，汇编只处理通用寄存器。
        (*now).save_fp_simd();
        (*next).restore_fp_simd();

        // 任务切换前保存当前任务的 TLS 相关基址。
        (*now).set_fs_base(Msr::new(IA32_FS_BASE).read() as usize);
        (*now).set_gs_base(Msr::new(IA32_KERNEL_GS_BASE).read() as usize);

        // 在任务切换处统一更新 rsp0，避免分散到 trap 返回路径。
        crate::trap::write_tss_rsp0((*next).kstack_top());

        // 切到下一个任务前恢复其 TLS 相关基址。
        Msr::new(IA32_FS_BASE).write((*next).fs_base() as u64);
        Msr::new(IA32_KERNEL_GS_BASE).write((*next).gs_base() as u64);

        __switch(now, next);
    }
}

pub static TASK_DOMAIN: Once<Arc<dyn TaskDomain>> = Once::new();
#[macro_export]
macro_rules! task_domain {
    () => {
        basic::sync::OnceGet::get_must(&$crate::task::TASK_DOMAIN)
    };
}

pub fn register_scheduler_domain(scheduler_domain: Arc<dyn SchedulerDomain>) {
    scheduler::set_scheduler(scheduler_domain);
}

pub fn register_task_domain(task_domain: Arc<dyn TaskDomain>) {
    TASK_DOMAIN.call_once(|| task_domain);
}

pub fn run_task() {
    processor::cpu_loop();
}

pub fn add_one_task(task_meta: TaskMeta, is_kthread: bool) -> AlienResult<usize> {
    let mut task_meta_ext = TaskMetaExt::new(task_meta, is_kthread);
    let kstack_top = task_meta_ext.kstack.top();

    task_meta_ext.set_status(TaskStatus::Waiting);
    let tid = task_meta_ext.tid();
    let task = Arc::new(Mutex::new(task_meta_ext));
    TASK_WAIT_QUEUE.lock().insert(tid, task);

    Ok(kstack_top.as_usize())
}

pub fn synchronize_rcu() {
    let task = current_task();
    if task.is_none() {
        return;
    }
    let task = task.expect("no current task");
    let mut guard = task.lock();
    let old_cpus_allowed = guard.scheduling_info.as_ref().unwrap().cpus_allowed;
    guard.scheduling_info.as_mut().unwrap().cpus_allowed = (1 << CPU_NUM) - 1;
    // println!("set cpus_allowed to {}", (1 << CPU_NUM) - 1);
    drop(guard);
    loop {
        let mut guard = task.lock();
        let current_cpu = cpu_id();
        let mut cpus_allowed = guard.scheduling_info.as_ref().unwrap().cpus_allowed;
        cpus_allowed &= !(1 << current_cpu);
        if cpus_allowed == CPU_OK {
            // println!("synchronize_rcu done");
            guard.scheduling_info.as_mut().unwrap().cpus_allowed = old_cpus_allowed;
            break;
        }
        guard.scheduling_info.as_mut().unwrap().cpus_allowed = cpus_allowed;
        // println!("synchronize_rcu cpus_allowed: {}", cpus_allowed);
        drop(guard);
        yield_now();
    }
}

#[cfg(plat_vf2)]
const CPU_OK: usize = 1;

#[cfg(not(plat_vf2))]
const CPU_OK: usize = 0;
