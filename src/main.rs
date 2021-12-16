#![no_std]
#![no_main]
#![feature(global_asm, asm, panic_info_message)]

mod device;
mod init;
mod io;
mod mem;
mod sbi;
mod tests;

use riscv::register::{scause, sepc, sscratch, stvec};

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/mem.S"));

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

    init::init(a0, a1);

    #[cfg(debug_assertions)]
    tests::start();

    abort();
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
