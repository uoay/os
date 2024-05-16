use core::arch::global_asm;

use riscv::register::{scause::{self, Exception, Trap}, sscratch, sstatus::Sstatus, stval, stvec};

use crate::println;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

global_asm!(include_str!("trap.asm"));

pub fn init() {
    sscratch::write(0);
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("init");
}

#[no_mangle]
fn trap_handler(context: &mut TrapContext) {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            context.sepc += 2;
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
}
