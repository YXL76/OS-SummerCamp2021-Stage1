pub const USER_STACK_SIZE: usize = 4096;

pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

pub const MAX_APP_NUM: usize = 16;

pub const APP_BASE_ADDRESS: usize = 0x80400000;

pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const CLOCK_FREQ: usize = 12500000;

pub const BIG_STRIDE: isize = isize::MAX;

pub const DEFAULT_PRIO: isize = 16;
