use common::io::STDOUT;

use crate::print;

pub(crate) fn write(target: usize, buffer: *const u8, length: usize) -> isize {
    if target == STDOUT.0 {
        let slice = unsafe {
            core::slice::from_raw_parts(buffer, length)
        };
        let str = core::str::from_utf8(slice).unwrap();
        print!("{}", str);
        length as isize
    } else {
        panic!("Unsupported target in write");
    }
}
