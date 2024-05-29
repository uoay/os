use crate::batch::APP_MANAGER;

pub(crate) fn exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    unsafe {
        APP_MANAGER.run_next_app();
    };
}
