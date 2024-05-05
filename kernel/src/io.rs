use core::fmt::{self, Write};

#[cfg(target_arch = "riscv64")]
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
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[allow(unused_variables)]
pub fn putchar(c: char) {
    #[cfg(target_arch = "riscv64")]
    sbi::debug_console_write_byte(c);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}
