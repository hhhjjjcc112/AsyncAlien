use alloc::sync::Arc;
use core::{
    hint::spin_loop,
    sync::atomic::{AtomicUsize, Ordering},
};

use basic::{
    arch::CpuLocal,
    sync::Mutex,
};
use config::CPU_NUM;
use spin::Lazy;
use task_meta::{TaskContext, TaskStatus};

use crate::task::{
    resource::TaskMetaExt,
    scheduler::{add_task, fetch_task},
};
use platform::percpu_impl::cpu_id;

static SCHEDULE_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);
static CURRENT_TID_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// 空闲 CPU 的 tid 哨兵值。
const NO_TID: usize = usize::MAX;

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

static CPUS: Lazy<[CpuLocal<Cpu>; CPU_NUM]> = Lazy::new(|| {
    core::array::from_fn(|_| CpuLocal::new(Cpu::empty()))
});

static CURRENT_TIDS: Lazy<[CpuLocal<usize>; CPU_NUM]> = Lazy::new(|| {
    core::array::from_fn(|_| CpuLocal::new(NO_TID))
});

#[inline(always)]
fn set_current_tid(tid: usize) {
    let trace_idx = CURRENT_TID_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if trace_idx < 16 {
        println!("[kernel][sched] set_current_tid cpu={} tid={}", cpu_id(), tid);
    }
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
        let _tid = match current_task {
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
            next_task.lock().set_status(TaskStatus::Running);
            let next_task_ctx_ptr = next_task.lock().get_context_raw_mut_ptr();
            let next_tid = next_task.lock().tid();
            let trace_idx = SCHEDULE_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
            if trace_idx < 16 {
                println!(
                    "[kernel][sched] cpu={} switch_to tid={}",
                    cpu_id(),
                    next_tid,
                );
            }
            cpu.set_current(next_task);
            set_current_tid(next_tid);
            let cpu_context = cpu.get_idle_task_cx_ptr();
            crate::task::switch(cpu_context, next_task_ctx_ptr)
        } else {
            spin_loop();
        }
    }
}
