pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!("ecall",
             inout("x10") arg0 => ret,
                in("x11") arg1,
                in("x12") arg2,
                in("x17") which
        );
    }
    ret
}
