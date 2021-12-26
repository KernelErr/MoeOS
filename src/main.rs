#![no_std]
#![no_main]
#![feature(panic_info_message)]
use core::arch::{asm, global_asm};

mod device;
mod init;
mod io;
mod mem;
mod sbi;
mod trap;
mod timer;
mod tests;

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/mem.S"));
global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
extern "C" fn kstart() -> ! {
    // a0 contains a unique per-hart ID
    // a1 contains a pointer to the device tree
    // We should save the value ASAP
    let a0: usize;
    let a1: usize;
    unsafe {
        asm!(
            "",
            out("x10") a0,
            out("x11") a1,
        );
    }

    clear_bss();
    init::init(a0, a1);

    #[cfg(debug_assertions)]
    tests::start();

    abort();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(p) = info.location() {
        error!(
            "PANIC: line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        error!("PANIC: no information");
    }
    abort();
}
