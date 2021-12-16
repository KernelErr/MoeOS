// This module is specially for Nezha MQ board (temorarily)
use core::ptr;

pub fn blue_led_on() {
    let pd2_controller = 0x02000098 as *mut u32;
    let mut pd2_controller_val: u32;
    unsafe {
        pd2_controller_val = ptr::read(pd2_controller);
    }
    pd2_controller_val &= 0xf0ffffff;
    pd2_controller_val |= 0x01000000;
    unsafe {
        ptr::write_volatile(pd2_controller, pd2_controller_val);
    }
}

pub fn blue_led_off() {
    let pd2_controller = 0x02000098 as *mut u32;
    let mut pd2_controller_val: u32;
    unsafe {
        pd2_controller_val = ptr::read(pd2_controller);
    }
    pd2_controller_val &= 0xf0ffffff;
    pd2_controller_val |= 0x0f000000;
    unsafe {
        ptr::write_volatile(pd2_controller, pd2_controller_val);
    }
}
