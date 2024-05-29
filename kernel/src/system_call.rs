mod io;
mod process;

use common::system_call::*;

pub fn dispatch_system_call(id: usize, args: [usize; 7]) -> isize {
    match id {
        WRITE => crate::system_call::io::write(args[0], args[1] as *const u8, args[2]),
        EXIT => crate::system_call::process::exit(args[0] as i32),
        _ => panic!("Unsupported system call id: {}", id),
    }
}
