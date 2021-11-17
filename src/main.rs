#![no_std]
#![no_main]
#![feature(global_asm, asm, panic_info_message)]

mod init;
mod io;
mod sbi;

global_asm!(include_str!("asm/boot.S"));

#[no_mangle]
extern "C" fn kstart() -> ! {
    init::init();
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
