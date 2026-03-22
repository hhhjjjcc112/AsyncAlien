use alloc::sync::Arc;
use core::hint::spin_loop;

#[cfg(target_arch = "riscv64")]
use core::arch::asm;

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

#[cfg(target_arch = "x86_64")]
#[percpu::def_percpu]
static CURRENT_TID: usize = 0;

pub fn current_cpu() -> &'static mut Cpu {
    CPUS[cpu_id()].as_mut()
}

pub fn current_task() -> Option<Arc<Mutex<TaskMetaExt>>> {
    CPUS[cpu_id()].current()
}

pub fn current_tid() -> Option<usize> {
    current_tid_impl()
}

#[cfg(target_arch = "riscv64")]
fn current_tid_impl() -> Option<usize> {
    let mut tp: usize;
    unsafe {
        asm!(
            "mv {}, tp",
            out(reg) tp,
        )
    }
    let tid = tp >> 32;
    if tid == 0 {
        None
    } else {
        Some(tid)
    }
}

#[cfg(target_arch = "x86_64")]
fn current_tid_impl() -> Option<usize> {
    let tid = CURRENT_TID.read_current();
    if tid == 0 {
        None
    } else {
        Some(tid)
    }
}

pub fn take_current_task() -> Option<Arc<Mutex<TaskMetaExt>>> {
    CPUS[cpu_id()].as_mut().take_current()
}

/// Set thread pointer register
/// 
/// - RISC-V: Sets the `tp` register
/// - x86-64: Writes current tid into percpu
#[inline(always)]
#[cfg(target_arch = "riscv64")]
fn set_tp(tp: usize) {
    unsafe {
        asm!("mv tp, {}", in(reg) tp, options(nostack));
    }
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
fn set_tp(tp: usize) {
    CURRENT_TID.write_current(tp);
}

/// Create thread pointer value from task ID
/// 
/// Creates architecture-specific tp value.
/// - RISC-V: Upper 32 bits = TID, lower 32 bits = cpu_id
/// - x86-64: Equals TID
#[inline(always)]
#[cfg(target_arch = "riscv64")]
fn tp_from_tid(tid: usize) -> usize {
    let current_cpu = cpu_id(); // Get current CPU ID
    // tid:cpu_id format (32:32 bits)
    (tid << 32) | current_cpu
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
fn tp_from_tid(tid: usize) -> usize {
    tid
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
        let current_task = take_current_task();
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
            // log::warn!(
            //     "[tid: {:?}] switch to task {:?}",
            //     tid,
            //     next_task.lock().tid()
            // );
            let next_tid = next_task.lock().tid();
            cpu.set_current(next_task);
            set_tp(tp_from_tid(next_tid));
            let cpu_context = cpu.get_idle_task_cx_ptr();
            crate::task::switch(cpu_context, next_task_ctx_ptr)
        } else {
            spin_loop();
        }
    }
}
