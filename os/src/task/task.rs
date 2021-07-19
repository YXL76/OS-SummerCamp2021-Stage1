use crate::config::BIG_STRIDE;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_cx_ptr: usize,
    pub task_status: TaskStatus,
    pub task_pass: isize,
    pub task_stride: isize,
    pub task_lastrun: usize,
    pub task_duration: usize,
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }

    pub fn set_task_pass(&mut self, prio: isize) {
        self.task_pass = BIG_STRIDE / prio;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
