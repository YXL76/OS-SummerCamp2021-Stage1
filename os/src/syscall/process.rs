use crate::config::BIG_STRIDE;
use crate::mm::translated_byte_buffer;
use crate::task::{
    current_user_token, exit_current_and_run_next, set_priority, suspend_current_and_run_next,
};
use crate::timer::get_time_us;

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *const u8, _tz: usize) -> isize {
    let ts =
        translated_byte_buffer(current_user_token(), ts, 1)[0].as_ptr() as *const _ as *mut TimeVal;
    let usec = get_time_us();
    unsafe {
        (*ts).sec = usec / 1000000;
        (*ts).usec = usec % 1000000;
    }
    0
}

pub fn sys_set_priority(prio: isize) -> isize {
    if prio >= 2 && prio <= BIG_STRIDE {
        set_priority(prio)
    } else {
        -1
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}
