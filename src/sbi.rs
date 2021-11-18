#[allow(dead_code)]
pub enum LegacyExt {
    SetTimer = 0,
    ConsolePutchar = 1,
    ConsoleGetchar = 2,
    ClearIPI = 3,
    SendIPI = 4,
    RemoteFENCEI = 5,
    RemoteSFENCEVMA = 6,
    RemoteSFENCEVMAASID = 7,
    Shutdown = 8,
}

impl From<LegacyExt> for usize {
    fn from(ext: LegacyExt) -> usize {
        ext as usize
    }
}

pub fn sbi_call(which: LegacyExt, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    let which: usize = which.into();
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
