use alloc::sync::Arc;
use core::hint::spin_loop;

use basic::{arch::CpuLocal, sync::Mutex};
use config::CPU_NUM;
use platform::percpu_impl::cpu_id;
use spin::Lazy;
use task_meta::{TaskContext, TaskStatus};

use crate::task::{
    resource::TaskMetaExt,
    scheduler::{add_task, fetch_task},
};
/// 空闲 CPU 的 tid 哨兵值。
const NO_TID: usize = usize::MAX;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn refresh_current_tss_rsp0(next_tid: usize) {
    let (_, trap_frame_virt_addr) = crate::task_domain!()
        .page_table_token_with_trap_frame_virt_addr()
        .unwrap_or_else(|err| {
            panic!(
                "x86_64 cpu_loop: failed to get trap frame for tid {}: {:?}",
                next_tid, err
            );
        });
    let rsp0 = trap_frame_virt_addr + crate::trap::X86TrapFrame::USER_CONTEXT_SIZE;
    crate::trap::write_tss_rsp0(rsp0);
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pub(crate) task: Option<Arc<Mutex<TaskMetaExt>>>,
    pub(crate) context: TaskContext,
}

impl Cpu {
    const fn empty() -> Self {
        Self {
            task: None,
            context: TaskContext::empty(),
        }
    }
    pub fn current(&self) -> Option<Arc<Mutex<TaskMetaExt>>> {
        self.task.as_ref().map(Arc::clone)
    }
    pub fn take_current(&mut self) -> Option<Arc<Mutex<TaskMetaExt>>> {
        self.task.take()
    }
    pub fn set_current(&mut self, task_meta: Arc<Mutex<TaskMetaExt>>) {
        self.task.replace(task_meta);
    }
    pub fn get_idle_task_cx_ptr(&self) -> *mut TaskContext {
        &self.context as *const TaskContext as *mut _
    }
}

static CPUS: Lazy<[CpuLocal<Cpu>; CPU_NUM]> =
    Lazy::new(|| core::array::from_fn(|_| CpuLocal::new(Cpu::empty())));

static CURRENT_TIDS: Lazy<[CpuLocal<usize>; CPU_NUM]> =
    Lazy::new(|| core::array::from_fn(|_| CpuLocal::new(NO_TID)));


#[inline(always)]
fn set_current_tid(tid: usize) {
    *CURRENT_TIDS[cpu_id()].as_mut() = tid;
}

/// 初始化当前 CPU 的 tid 哨兵。
pub fn init_current_tid() {
    set_current_tid(NO_TID);
}

pub fn current_cpu() -> &'static mut Cpu {
    CPUS[cpu_id()].as_mut()
}

pub fn current_task() -> Option<Arc<Mutex<TaskMetaExt>>> {
    CPUS[cpu_id()]
        .current()
        .or_else(|| current_tid().and_then(crate::task::scheduler::find_task))
}

pub fn current_tid() -> Option<usize> {
    let tid = *CURRENT_TIDS[cpu_id()].get();
    if tid == NO_TID {
        None
    } else {
        Some(tid)
    }
}

pub fn schedule() {
    let cpu = current_cpu();
    let current_task = current_task().unwrap();
    let task_context = current_task.lock().get_context_raw_mut_ptr();
    drop(current_task);
    let cpu_context = cpu.get_idle_task_cx_ptr();
    crate::task::switch(task_context, cpu_context);
}

pub fn cpu_loop() {
    loop {
        let cpu = current_cpu();
        let current_task = cpu.take_current();
        set_current_tid(NO_TID);
        match current_task {
            Some(task) => {
                let tid = task.lock().tid();
                let status = task.lock().status();
                match status {
                    TaskStatus::Ready => {
                        add_task(task);
                    }
                    TaskStatus::Zombie => {
                        task.lock().set_status(TaskStatus::Terminated);
                    }
                    _ => {}
                }
                Some(tid)
            }
            None => None,
        };
        if let Some(next_task) = fetch_task() {
            let mut next_guard = next_task.lock();
            next_guard.set_status(TaskStatus::Running);
            let next_tid = next_guard.tid();
            let next_task_ctx_ptr = next_guard.get_context_raw_mut_ptr();
            drop(next_guard);
            cpu.set_current(next_task);
            set_current_tid(next_tid);
            #[cfg(target_arch = "x86_64")]
            refresh_current_tss_rsp0(next_tid);
            let cpu_context = cpu.get_idle_task_cx_ptr();
            crate::task::switch(cpu_context, next_task_ctx_ptr)
        } else {
            spin_loop();
        }
    }
}
