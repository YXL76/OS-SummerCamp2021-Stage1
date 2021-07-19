#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

use log::{debug, error, info, trace, warn};

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    console::init();
    error!("hello world");
    warn!("hello world");
    info!("hello world");
    debug!("hello world");
    trace!("hello world");

    let stext = stext as usize;
    let etext = etext as usize;
    let srodata = srodata as usize;
    let erodata = erodata as usize;
    let sdata = sdata as usize;
    let edata = edata as usize;
    let boot_stack = boot_stack as usize;
    let boot_stack_top = boot_stack_top as usize;
    let sbss = sbss as usize;
    let ebss = ebss as usize;

    info!(".text [{:#x}, {:#x})", stext, etext);
    info!(".rodata [{:#x}, {:#x})", srodata, erodata);
    info!(".data [{:#x}, {:#x})", sdata, edata);
    info!("boot_stack [{:#x}, {:#x})", boot_stack, boot_stack_top);
    info!(".bss [{:#x}, {:#x})", sbss, ebss);

    panic!("Shutdown machine!");
}
