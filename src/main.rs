#![no_std]
#![no_main]
#![feature(global_asm, asm, panic_info_message)]

mod device;
mod init;
mod io;
mod mem;
mod sbi;

global_asm!(include_str!("asm/boot.S"));

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
    print!("(T_T) PANIC: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information");
    }
    abort();
}
