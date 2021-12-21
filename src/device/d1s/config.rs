use crate::timer::CLOCK_FREQ;

pub fn init() {
    unsafe {
        CLOCK_FREQ = 24000000;
    }
}