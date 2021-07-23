use super::TaskControlBlock;
use super::__switch;
use super::{fetch_task, TaskStatus};
use crate::mm::MapPermission;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use lazy_static::lazy_static;

pub struct Processor {
    inner: RefCell<ProcessorInner>,
}

unsafe impl Sync for Processor {}

struct ProcessorInner {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx_ptr: usize,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ProcessorInner {
                current: None,
                idle_task_cx_ptr: 0,
            }),
        }
    }

    fn get_idle_task_cx_ptr2(&self) -> *const usize {
        let inner = self.inner.borrow();
        &inner.idle_task_cx_ptr as *const usize
    }

    pub fn run(&self) {
        loop {
            if let Some(task) = fetch_task() {
                let idle_task_cx_ptr2 = self.get_idle_task_cx_ptr2();
                // acquire
                let mut task_inner = task.acquire_inner_lock();
                let next_task_cx_ptr2 = task_inner.get_task_cx_ptr2();
                task_inner.task_status = TaskStatus::Running;
                task_inner.add_task_stride();
                drop(task_inner);
                // release
                self.inner.borrow_mut().current = Some(task);
                unsafe {
                    __switch(idle_task_cx_ptr2, next_task_cx_ptr2);
                }
            }
        }
    }
    pub fn take_current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.borrow_mut().current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner
            .borrow()
            .current
            .as_ref()
            .map(|task| Arc::clone(task))
    }
}

lazy_static! {
    pub static ref PROCESSOR: Processor = Processor::new();
}

pub fn run_tasks() {
    PROCESSOR.run();
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.acquire_inner_lock().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().acquire_inner_lock().get_trap_cx()
}

pub fn schedule(switched_task_cx_ptr2: *const usize) {
    let idle_task_cx_ptr2 = PROCESSOR.get_idle_task_cx_ptr2();
    unsafe {
        __switch(switched_task_cx_ptr2, idle_task_cx_ptr2);
    }
}

pub fn set_priority(prio: isize) -> isize {
    current_task()
        .unwrap()
        .borrow_mut()
        .acquire_inner_lock()
        .set_task_pass(prio as usize);
    prio
}

pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    if (port & !0x7 != 0) || (port & 0x7 == 0) || (start & 0x111 != 0) {
        return -1;
    }
    current_task()
        .unwrap()
        .borrow_mut()
        .acquire_inner_lock()
        .memory_set
        .insert_framed_area(
            start.into(),
            (start + len).into(),
            MapPermission::from_bits((port << 1) as u8).unwrap() | MapPermission::U,
        )
}

pub fn munmap(start: usize, len: usize) -> isize {
    if (start & 0x111 != 0) || (len & 0x111 != 0) {
        return -1;
    }
    current_task()
        .unwrap()
        .borrow_mut()
        .acquire_inner_lock()
        .memory_set
        .remove_framed_area(start.into(), (start + len).into())
}
