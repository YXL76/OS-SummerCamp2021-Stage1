mod pipe;
mod stdio;

use crate::mm::UserBuffer;

pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}
