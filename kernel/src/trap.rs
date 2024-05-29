use core::arch::global_asm;

use riscv::register::{scause::{self, Exception, Trap}, sstatus::{self, Sstatus, SPP}, stval, stvec};

use crate::{println, system_call::dispatch_system_call};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_stack_pointer(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn init(app_entry: usize, stack_pointer: usize) -> Self {
        unsafe {
            sstatus::set_spp(SPP::User);
        }
        let mut context = Self {
            x: [0; 32],
            sstatus: sstatus::read(),
            sepc: app_entry,
        };
        context.set_stack_pointer(stack_pointer);
        context
    }
}

global_asm!(include_str!("trap.asm"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("init");
}

#[no_mangle]
fn trap_handler(context: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            context.sepc += 4;
            context.x[10] = dispatch_system_call(
                context.x[17],
                [context.x[10], context.x[11],context.x[12],context.x[13],context.x[14],context.x[15], context.x[16]]
            ) as usize;
        },
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    context
}
