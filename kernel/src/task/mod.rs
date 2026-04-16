mod processor;
mod resource;
#[cfg(target_arch = "riscv64")]
mod riscv64;
mod scheduler;
#[cfg(target_arch = "x86_64")]
mod x86_64;

use alloc::{sync::Arc, vec::Vec};

use arch::cpu_id;
use basic::sync::Once;
use config::CPU_NUM;
use interface::{SchedulerDomain, TaskDomain};
use ksync::Mutex;
pub use processor::{current_task, current_tid, init_current_tid};
pub use scheduler::{
    exit_now, get_task_priority, is_task_exit, remove_task, set_task_priority, wait_now,
    wake_up_wait_task, yield_now,
};
use task_meta::{TaskMeta, TaskStatus};

#[cfg(target_arch = "riscv64")]
pub use self::riscv64::switch;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::{
    current_user_fs_base, current_user_gs_base, set_current_user_fs_base, set_current_user_gs_base,
    switch,
};
use crate::{
    error::AlienResult,
    task::{resource::TaskMetaExt, scheduler::TASK_WAIT_QUEUE},
};

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
    // 启动初期兜底：确保已创建的初始任务进入调度队列。
    let wait_tids: Vec<usize> = TASK_WAIT_QUEUE.lock().keys().copied().collect();
    println!("run_task bootstrap wake {} tasks", wait_tids.len());
    for tid in wait_tids {
        scheduler::wake_up_wait_task(tid);
    }
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
