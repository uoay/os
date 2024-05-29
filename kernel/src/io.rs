use core::fmt::{self, Write};

use crate::sbi;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        puts(string);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub fn putchar(c: char) {
    sbi::debug_console_write_byte(c);
}

pub fn puts(string: &str) {
    for c in string.chars() {
        putchar(c);
    }
}
