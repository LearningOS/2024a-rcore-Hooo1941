//! Semaphore

use crate::sync::UPSafeCell;
use crate::task::{
    block_current_and_run_next, current_task, wakeup_task, TaskControlBlock, TaskStatus,
};
use alloc::{collections::VecDeque, sync::Arc};

/// semaphore structure
pub struct Semaphore {
    /// semaphore inner
    pub inner: UPSafeCell<SemaphoreInner>,
}

pub struct SemaphoreInner {
    pub count: isize,
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(res_count: usize) -> Self {
        trace!("kernel: Semaphore::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(SemaphoreInner {
                    count: res_count as isize,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    /// up operation of semaphore
    pub fn up(&self, sem_id: usize) {
        trace!("kernel: Semaphore::up");
        let mut inner = self.inner.exclusive_access();
        inner.count += 1;
        if inner.count <= 0 {
            if let Some(task) = inner.wait_queue.pop_front() {
                let process = task.process.upgrade().unwrap();
                let mut process = process.inner_exclusive_access();
                let tid = process
                    .tasks
                    .iter()
                    .position(|x| {
                        x.clone().is_some_and(|x| {
                            let task_trap_cx =
                                task.inner_exclusive_access().get_trap_cx() as *const _ as usize;
                            let x_trap_cx =
                                x.inner_exclusive_access().get_trap_cx() as *const _ as usize;
                            task_trap_cx == x_trap_cx
                        })
                    })
                    .unwrap();
                process.need_list[tid][sem_id] -= 1;
                process.allocation_semaphore_list[tid][sem_id] += 1;
                wakeup_task(task);
            }
        }
    }

    /// down operation of semaphore
    pub fn down(&self, sem_id: usize) {
        trace!("kernel: Semaphore::down");
        let mut inner = self.inner.exclusive_access();
        inner.count -= 1;
        if inner.count < 0 {
            let task = current_task().unwrap();
            let process = task.process.upgrade().unwrap();
            let mut process = process.inner_exclusive_access();
            let tid = process
                .tasks
                .iter()
                .position(|x| {
                    x.clone().is_some_and(|x| {
                        x.inner_exclusive_access().task_status == TaskStatus::Running
                    })
                })
                .unwrap();
            process.need_list[tid][sem_id] += 1;
            drop(process);
            inner.wait_queue.push_back(task);
            drop(inner);
            block_current_and_run_next();
        } else {
            let task = current_task().unwrap();
            let process = task.process.upgrade().unwrap();
            let mut process = process.inner_exclusive_access();
            let tid = process
                .tasks
                .iter()
                .position(|x| {
                    x.clone().is_some_and(|x| {
                        x.inner_exclusive_access().task_status == TaskStatus::Running
                    })
                })
                .unwrap();
            process.allocation_semaphore_list[tid][sem_id] += 1;
        }
    }
}
