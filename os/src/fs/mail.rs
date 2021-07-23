use crate::mm::UserBuffer;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::{isize, usize};

const MAIL_SIZE: usize = 256;

const MAIL_MAX: usize = 16;

struct Mail {
    data: Vec<u8>,
    #[allow(dead_code)]
    sender: usize,
}

pub struct MailBox {
    mails: VecDeque<Mail>,
}

impl MailBox {
    pub fn new() -> Self {
        Self {
            mails: VecDeque::with_capacity(MAIL_SIZE),
        }
    }

    pub fn read(&mut self, buf: UserBuffer, len: usize) -> isize {
        if self.mails.len() == 0 {
            return -1;
        }
        if len == 0 {
            return 0;
        }
        let mail = self.mails.pop_front().unwrap();
        let mut buf = buf.into_iter();
        let mut read_size = 0;
        for src in mail.data {
            if let Some(dst) = buf.next() {
                unsafe {
                    *dst = src;
                }
                read_size += 1;
            } else {
                break;
            }
        }
        read_size
    }

    pub fn write(&mut self, sender: usize, buf: UserBuffer, len: usize) -> isize {
        if self.mails.len() >= MAIL_MAX {
            return -1;
        }
        if len == 0 {
            return 0;
        }
        let mut data = Vec::new();
        let mut write_size = 0;
        for i in buf {
            data.push(unsafe { *i });
            write_size += 1;
        }
        self.mails.push_back(Mail { data, sender });
        write_size
    }
}
