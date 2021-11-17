use crate::io::console::{putchar, puts};
use crate::{println, print};

const BUILD_NAME: &'static str = env!("BUILD_NAME");
const BUILD_VERSION: &'static str = env!("BUILD_VERSION");
const BUILD_TIME: &'static str = env!("BUILD_TIME");
const BANNER: &str = include_str!("statics/banner.txt");

pub fn init() {
    print_banner();
    print_memory_info();
}

fn print_banner() {
    putchar('\n');
    puts(BANNER);
    println!("{} {} {}", BUILD_NAME, BUILD_VERSION, BUILD_TIME);
    putchar('\n');
}

fn print_memory_info() {
    println!("Memory information:");
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    putchar('\n');
}