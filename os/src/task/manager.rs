use super::TaskControlBlock;
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use core::cmp::Reverse;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct TaskManager {
    ready_queue: BinaryHeap<Reverse<Arc<TaskControlBlock>>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }

    fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(Reverse(task));
    }

    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop().map(|Reverse(task)| task)
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().fetch()
}
