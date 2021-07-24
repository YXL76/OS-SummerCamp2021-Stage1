use crate::fs::{make_pipe, open_file, OpenFlags};
use crate::mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer};
use crate::task::{current_task, TASK_MANAGER};
use alloc::sync::Arc;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if !inner.check_buf(buf as usize, len) || fd >= inner.fd_table.len() {
        return -1;
    }
    let token = inner.get_user_token();
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if !inner.check_buf(buf as usize, len) || fd >= inner.fd_table.len() {
        return -1;
    }
    let token = inner.get_user_token();
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let token = inner.get_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() || inner.fd_table[fd].is_none() {
        -1
    } else {
        inner.fd_table[fd].take();
        0
    }
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let token = inner.get_user_token();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}

pub fn sys_mail_read(buf: *mut u8, len: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if !inner.check_buf(buf as usize, len) {
        return -1;
    }
    let token = inner.get_user_token();
    inner.mailbox.read(
        UserBuffer::new(translated_byte_buffer(token, buf, len)),
        len,
    )
}

pub fn sys_mail_write(pid: usize, buf: *mut u8, len: usize) -> isize {
    let len = len.min(256);
    let current = current_task().unwrap();
    let mut inner = current.acquire_inner_lock();
    if !inner.check_buf(buf as usize, len) {
        return -1;
    }
    let token = inner.get_user_token();
    let sender = current.getpid();
    if sender == pid {
        inner.mailbox.write(
            sender,
            UserBuffer::new(translated_byte_buffer(token, buf, len)),
            len,
        )
    } else if let Some(task) = TASK_MANAGER.lock().find(pid) {
        let mut inner = task.acquire_inner_lock();
        inner.mailbox.write(
            sender,
            UserBuffer::new(translated_byte_buffer(token, buf, len)),
            len,
        )
    } else {
        -1
    }
}

pub fn sys_dup(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() || inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(Arc::clone(inner.fd_table[fd].as_ref().unwrap()));
    new_fd as isize
}
