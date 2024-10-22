//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    // TASK_MANAGER.exclusive_access().fetch()
    fetch_task_stride()
}

/// Take a process out of the ready queue with min stride value
fn fetch_task_stride() -> Option<Arc<TaskControlBlock>> {
    let mut task_manager = TASK_MANAGER.exclusive_access();
    if task_manager.ready_queue.is_empty() {
        return None;
    }
    let mut min_stride = usize::MAX;
    let mut min_stride_id = 0;
    for id in 0..task_manager.ready_queue.len() {
        let task = task_manager.fetch().unwrap();
        let stride = task.inner_exclusive_access().stride;
        if stride < min_stride {
            min_stride = stride;
            min_stride_id = id;
        }
        task_manager.add(task);
    }
    task_manager.ready_queue.remove(min_stride_id)
}