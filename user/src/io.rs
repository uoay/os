use core::fmt::{self, Write};

use common::io::*;

use crate::system_call;

pub fn write(file_descriptor: FileDescriptor, buffer: &[u8]) {
    system_call::io::write(file_descriptor, buffer);
}

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        write(STDOUT, str.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::io::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::io::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
