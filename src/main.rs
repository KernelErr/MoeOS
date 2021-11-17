#![no_std]
#![no_main]
#![feature(global_asm, asm)]

global_asm!(include_str!("asm/boot.S"));

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    console_putchar(b'H');
    console_putchar(b'e');
    console_putchar(b'l');
    console_putchar(b'l');
    console_putchar(b'o');
    console_putchar(b',');
    console_putchar(b'w');
    console_putchar(b'o');
    console_putchar(b'r');
    console_putchar(b'l');
    console_putchar(b'd');
    console_putchar(b'!');
    console_putchar(b'\n');
    loop {}
}

pub fn console_putchar(ch: u8) {
    let ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!("ecall",
             inout("x10") arg0 => ret,
                in("x11") arg1,
                in("x12") arg2,
                in("x17") which
        );
    }
}

use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
