use riscv::register::{time, sie};
use crate::sbi::set_timer;
use crate::info;

pub static mut CLOCK_FREQ: usize = 0;
const MICRO_PER_SEC: usize = 1_000_000;
const TICKS_PER_SEC: usize = 1000;

pub fn init() {
    unsafe {
        if CLOCK_FREQ == 0 {
            panic!("Clock frequency is zero!");
        }
    }
    unsafe {
        sie::set_stimer();
    }
    info!("Timer ready");
    timer_next_triger();
}

pub fn get_time_us() -> usize {
    unsafe {
        time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
    }
}

pub fn get_time() -> usize {
    time::read()
}

pub fn timer_next_triger() {
    unsafe {
        set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
    }
}