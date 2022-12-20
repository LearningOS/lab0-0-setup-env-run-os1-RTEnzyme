// remove the std rather than using the core
#![no_std]
// remove the _start stage
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
mod lang_items;
mod sbi;
#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("hello world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss(); // given by linker.ld
        fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}