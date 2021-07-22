mod context;
mod switch;
mod task;

use crate::loader::{get_app_data, get_num_app};
use crate::mm::MapPermission;
use crate::trap::TrapContext;
use alloc::vec::Vec;
use core::cell::RefCell;
use lazy_static::lazy_static;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        info!("init TASK_MANAGER");
        let num_app = get_num_app();
        info!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }
        TaskManager {
            num_app,
            inner: RefCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        let next_task_cx_ptr2 = {
            let mut inner = self.inner.borrow_mut();
            inner.tasks[0].task_status = TaskStatus::Running;
            inner.tasks[0].task_stride += inner.tasks[0].task_pass;
            inner.tasks[0].get_task_cx_ptr2()
        };

        let _unused: usize = 0;
        unsafe {
            __switch(&_unused as *const _, next_task_cx_ptr2);
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        (0..self.num_app)
            .filter(|&id| inner.tasks[id].task_status == TaskStatus::Ready)
            .min_by_key(|&id| inner.tasks[id].task_stride)
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.tasks[next].task_stride += inner.tasks[next].task_pass;
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            unsafe {
                __switch(current_task_cx_ptr2, next_task_cx_ptr2);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn set_priority(&self, prio: isize) -> isize {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].set_task_pass(prio);
        prio
    }

    fn mmap(&self, start: usize, len: usize, port: usize) -> isize {
        if (port & !0x7 != 0) || (port & 0x7 == 0) || (start & 0x111 != 0) {
            return -1;
        }
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].memory_set.insert_framed_area(
            start.into(),
            (start + len).into(),
            MapPermission::from_bits((port << 1) as u8).unwrap() | MapPermission::U,
        )
    }

    fn munmap(&self, start: usize, len: usize) -> isize {
        if (start & 0x111 != 0) || (len & 0x111 != 0) {
            return -1;
        }
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current]
            .memory_set
            .remove_framed_area(start.into(), (start + len).into())
    }

    fn check_buf(&self, addr: usize, len: usize) -> bool {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].memory_set.check_buf(addr, len)
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn set_priority(prio: isize) -> isize {
    TASK_MANAGER.set_priority(prio)
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    TASK_MANAGER.mmap(start, len, port)
}

pub fn munmap(start: usize, len: usize) -> isize {
    TASK_MANAGER.munmap(start, len)
}

pub fn check_buf(addr: usize, len: usize) -> bool {
    TASK_MANAGER.check_buf(addr, len)
}
