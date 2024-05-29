#![no_std]
#![no_main]

use core::arch::global_asm;
use os::{batch::{self, AppManager,APP_MANAGER}, trap};

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    trap::init();
    unsafe {
        extern "C" { fn _num_app(); }
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = num_app_ptr.read_volatile();
        let mut app_start: [usize; batch::MAX_APP_NUM + 1] = [0; batch::MAX_APP_NUM + 1];
        let app_start_raw: &[usize] =  core::slice::from_raw_parts(
            num_app_ptr.add(1), num_app + 1
        );
        app_start[..=num_app].copy_from_slice(app_start_raw);
        APP_MANAGER = AppManager {
            num_app,
            current_app: 0,
            app_start,
        };
        APP_MANAGER.run_next_app();
    };
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
