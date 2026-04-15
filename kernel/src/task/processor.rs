use alloc::sync::Arc;
use core::hint::spin_loop;

use basic::{
    arch::{cpu_id, CpuLocal},
    sync::Mutex,
};
use config::CPU_NUM;
use task_meta::{TaskContext, TaskStatus};

use crate::task::{
    resource::TaskMetaExt,
    scheduler::{add_task, fetch_task},
};

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

const CPU_ONE: CpuLocal<Cpu> = CpuLocal::new(Cpu::empty());
static CPUS: [CpuLocal<Cpu>; CPU_NUM] = [CPU_ONE; CPU_NUM];

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct CurrentTid(usize);

impl CurrentTid {
    const fn new(tid: usize) -> Self {
        Self(tid)
    }
}

/// 当前 CPU 的 tid，跟随 percpu 存储。
#[percpu::def_percpu]
static CURRENT_TID: CurrentTid = CurrentTid::new(NO_TID);

#[inline(always)]
fn set_current_tid(tid: usize) {
    unsafe {
        CURRENT_TID.current_ref_mut_raw().0 = tid;
    }
}

/// 初始化当前 CPU 的 tid 哨兵。
pub fn init_current_tid() {
    set_current_tid(NO_TID);
}

pub fn current_cpu() -> &'static mut Cpu {
    CPUS[cpu_id()].as_mut()
}

pub fn current_task() -> Option<Arc<Mutex<TaskMetaExt>>> {
    CPUS[cpu_id()].current()
}

pub fn current_tid() -> Option<usize> {
    let tid = unsafe { CURRENT_TID.current_ref_raw().0 };
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
            cpu.set_current(next_task);
            set_current_tid(next_tid);
            let cpu_context = cpu.get_idle_task_cx_ptr();
            crate::task::switch(cpu_context, next_task_ctx_ptr)
        } else {
            spin_loop();
        }
    }
}
