use riscv::register::{
    sstatus::{self, Sstatus},
    scause::{self, Scause, Trap, Interrupt, Exception},
    sscratch,
    stvec,
    stval,
    sepc,
};
use crate::{info, error};
use crate::timer::timer_next_triger;


#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub spec: usize,
}

pub fn init() {
    sscratch::write(0);
    unsafe {
        extern "C" {
            fn _traps();
        }
        stvec::write(_traps as usize, stvec::TrapMode::Direct);
        sstatus::set_sie();
    }
    info!("Trap handler ready");
}

#[no_mangle]
extern "C" fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.spec += 4;
            cx.x[10] = 0;
        },
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            error!("{:?} va = {:#x} instruction = {:#x}", scause.cause(), stval::read(), sepc::read());
            panic!("page fault!");
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("IllegalInstruction");
        },
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer_next_triger();
        }
        _ => {
            panic!("Unhandled trap: {:?} stval = {:#x}", scause.cause(), stval);
        }
    }
    cx
}