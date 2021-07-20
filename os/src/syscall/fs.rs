const FD_STDOUT: usize = 1;

use crate::loader::check_buf;
use crate::task::current_task;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !check_buf(current_task(), buf as usize, len) {
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            // panic!("Unsupported fd in sys_write!");
            -1
        }
    }
}
