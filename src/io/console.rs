use crate::sbi::{sbi_call, LegacyExt};
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
		use crate::print;
		print!("\r\n")
	});
	($fmt:expr) => ({
		use crate::print;
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

#[macro_export]
macro_rules! trace
{
	($fmt:expr) => ({
		use crate::print;
		print!(concat!("\u{1b}[35mTRACE\u{1b}[0m ", concat!($fmt, "\r\n")))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		print!(concat!("\u{1b}[35mTRACE\u{1b}[0m ", concat!($fmt, "\r\n")), $($args)+)
	});
}

#[macro_export]
macro_rules! debug
{
	($fmt:expr) => ({
		use crate::print;
		#[cfg(debug_assertions)]
		print!(concat!("\u{1b}[34mDEBUG\u{1b}[0m ", concat!($fmt, "\r\n")))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		#[cfg(debug_assertions)]
		print!(concat!("\u{1b}[34mDEBUG\u{1b}[0m ", concat!($fmt, "\r\n")), $($args)+)
	});
}

#[macro_export]
macro_rules! info
{
	($fmt:expr) => ({
		use crate::print;
		print!(concat!("\u{1b}[32mINFO\u{1b}[0m ", concat!($fmt, "\r\n")))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		print!(concat!("\u{1b}[32mINFO\u{1b}[0m ", concat!($fmt, "\r\n")), $($args)+)
	});
}

#[macro_export]
macro_rules! warn
{
	($fmt:expr) => ({
		use crate::print;
		print!(concat!("\u{1b}[33mWARN\u{1b}[0m ", concat!($fmt, "\r\n")))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		print!(concat!("\u{1b}[33mWARN\u{1b}[0m ", concat!($fmt, "\r\n")), $($args)+)
	});
}

#[macro_export]
macro_rules! error
{
	($fmt:expr) => ({
		use crate::print;
		print!(concat!("\u{1b}[31mERROR\u{1b}[0m ", concat!($fmt, "\r\n")))
	});
	($fmt:expr, $($args:tt)+) => ({
		use crate::print;
		print!(concat!("\u{1b}[31mERROR\u{1b}[0m ", concat!($fmt, "\r\n")), $($args)+)
	});
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        puts(s);
        Ok(())
    }
}

pub fn putchar(c: char) {
    sbi_call(LegacyExt::ConsolePutchar.into(), 0, c as usize, 0, 0);
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}

pub fn getchar() -> Option<char> {
    let c = sbi_call(LegacyExt::ConsoleGetchar.into(), 0, 0, 0, 0);
    if c == usize::MAX {
        None
    } else {
        Some(c as u8 as char)
    }
}
