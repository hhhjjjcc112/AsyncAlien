use alloc::{
    collections::BTreeMap,
    sync::{Arc, Weak},
};

use basic::sync::Mutex;
use interface::SchedulerDomain;
use shared_heap::DBox;
use spin::Once;
use task_meta::{TaskSchedulingInfo, TaskStatus};

use super::{
    processor::{clear_task_last_cpu, schedule},
    resource::TaskMetaExt,
};
use crate::task::processor::current_task;
type Tid = usize;
static TASK_MAP: Mutex<BTreeMap<Tid, Arc<Mutex<TaskMetaExt>>>> = Mutex::new(BTreeMap::new());
static TASK_LOOKUP: Mutex<BTreeMap<Tid, Weak<Mutex<TaskMetaExt>>>> = Mutex::new(BTreeMap::new());
pub(super) static GLOBAL_SCHEDULER: Once<Arc<dyn SchedulerDomain>> = Once::new();

#[macro_export]
macro_rules! global_scheduler {
    () => {
        GLOBAL_SCHEDULER.get().unwrap()
    };
}

pub fn set_scheduler(scheduler: Arc<dyn SchedulerDomain>) {
    GLOBAL_SCHEDULER.call_once(|| scheduler);
}

pub fn add_task(task_meta: Arc<Mutex<TaskMetaExt>>) {
    let mut guard = task_meta.lock();
    let scheduling_info = guard.take_scheduling_info();
    let tid = scheduling_info.tid;
    drop(guard);
    TASK_LOOKUP.lock().insert(tid, Arc::downgrade(&task_meta));
    TASK_MAP.lock().insert(tid, task_meta);
    global_scheduler!().add_task(scheduling_info).unwrap();
}

pub fn find_task(tid: Tid) -> Option<Arc<Mutex<TaskMetaExt>>> {
    TASK_LOOKUP.lock().get(&tid).and_then(Weak::upgrade)
}

pub fn fetch_task() -> Option<Arc<Mutex<TaskMetaExt>>> {
    let info = DBox::new(TaskSchedulingInfo::default());
    assert_eq!(info.domain_id(), 0);
    let scheduling_info = global_scheduler!().fetch_task(info).unwrap();
    assert_eq!(scheduling_info.domain_id(), 0);
    if scheduling_info.tid != usize::MAX {
        let task = TASK_MAP.lock().remove(&scheduling_info.tid).unwrap();
        task.lock().set_sched_info(scheduling_info);
        return Some(task);
    }
    None
}
pub static TASK_WAIT_QUEUE: Mutex<BTreeMap<Tid, Arc<Mutex<TaskMetaExt>>>> =
    Mutex::new(BTreeMap::new());
pub static TASK_EXIT_QUEUE: Mutex<BTreeMap<Tid, Arc<Mutex<TaskMetaExt>>>> =
    Mutex::new(BTreeMap::new());

pub fn wait_now() {
    let task = current_task().unwrap();
    task.lock().set_status(TaskStatus::Waiting);
    let tid = task.lock().tid();
    println!("[kernel][sched] wait_now tid={}", tid);
    TASK_WAIT_QUEUE.lock().insert(tid, task);
    schedule();
}

pub fn wake_up_wait_task(tid: Tid) {
    let task = TASK_WAIT_QUEUE.lock().remove(&tid);
    if let Some(task) = task {
        println!("[kernel][sched] wake_up_wait_task tid={}", tid);
        // put the task into the global task queue
        task.lock().set_status(TaskStatus::Ready);
        add_task(task);
    } else {
        println!("[kernel][sched] wake_up_wait_task miss tid={}", tid);
    }
}

pub fn yield_now() {
    let Some(task) = current_task() else {
        return;
    };
    task.lock().set_status(TaskStatus::Ready);
    schedule();
}

pub fn exit_now() {
    let task = current_task().unwrap();
    let tid = task.lock().tid();
    task.lock().set_status(TaskStatus::Zombie);
    println!("[kernel][sched] exit_now tid={}", tid);
    TASK_EXIT_QUEUE.lock().insert(tid, task);
    schedule();
}

pub fn remove_task(tid: Tid) {
    TASK_LOOKUP.lock().remove(&tid);
    clear_task_last_cpu(tid);
    let task = TASK_EXIT_QUEUE.lock().remove(&tid).unwrap();
    let status = task.lock().status();
    assert_eq!(status, TaskStatus::Terminated);
    assert_eq!(Arc::strong_count(&task), 1);
}

pub fn is_task_exit(tid: Tid) -> bool {
    let guard = TASK_EXIT_QUEUE.lock();
    let task = guard.get(&tid);
    if let Some(task) = task {
        let status = task.lock().status();
        let ref_count = Arc::strong_count(task);
        return status == TaskStatus::Terminated && ref_count == 1;
    }
    false
}

pub fn set_task_priority(nice: i8) {
    let task = current_task().unwrap();
    let tid = task.lock().tid();
    task.lock().scheduling_info.as_mut().unwrap().set_nice(nice);
    println_color!(32, "tid: {}, set_task_priority: {:?}", tid, nice);
}

pub fn get_task_priority() -> i8 {
    let task = current_task().unwrap();
    let guard = task.lock();
    guard.scheduling_info.as_ref().unwrap().nice()
}
