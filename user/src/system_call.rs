pub(crate) mod io;
pub(crate) mod process;

use core::arch::asm;

fn system_call(id: usize, args: [usize; 7]) -> isize{
    let result: isize;
    unsafe {
        asm!("ecall",
            inlateout("a0") args[0] => result,
            in("a1") args[1],
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a6") args[6],
            in("a7") id
        );
    }
    result
}
