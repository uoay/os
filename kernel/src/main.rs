#![no_std]
#![no_main]

mod io;
mod lang_items;
#[cfg(target_arch = "riscv64")]
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    println!("Hello World!");
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    });
}
