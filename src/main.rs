#![no_std]
#![no_main]

mod io;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    println!("Hello World!");
    loop {}
}
