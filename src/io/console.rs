use crate::sbi::sbi_call;
use core::fmt::Write;

pub struct Stdout;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::io::console::Stdout, $($args)+);
	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        puts(s);
        Ok(())
    }
}

pub fn putchar(c: char) {
    sbi_call(1, c as usize, 0, 0);
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}
