#![allow(dead_code)]

use core::{arch::asm, fmt, mem::transmute};

#[derive(Default)]
struct SbiFunction(i32, i32); // SbiFunction(fid, eid)

// Base Extension
const GET_SPEC_VERSION: SbiFunction = SbiFunction(0, 0);
const GET_IMPL_VERSION: SbiFunction = SbiFunction(1, 0);
const PROBE_EXTENSION: SbiFunction = SbiFunction(2, 0);
const GET_MVENDORID: SbiFunction = SbiFunction(3, 0);
const GET_MARCHID: SbiFunction = SbiFunction(4, 0);
const GET_MIMPID: SbiFunction = SbiFunction(5, 0);

// IPI Extension
const SEND_IPI: SbiFunction = SbiFunction(0, 0x735049);

// Debug Console Extension
const DEBUG_CONSOLE_WRITE: SbiFunction = SbiFunction(0, 0x4442434E);
const DEBUG_CONSOLE_READ: SbiFunction = SbiFunction(1, 0x4442434E);
const DEBUG_CONSOLE_WRITE_BYTE: SbiFunction = SbiFunction(2, 0x4442434E);

// System Reset Extension
const SYSTEM_RESET: SbiFunction = SbiFunction(0, 0x53525354);

// Timer Extension
const SET_TIMER: SbiFunction = SbiFunction(0, 0x54494D45);

// RFENCE Extension
const REMOTE_FENCE_I: SbiFunction = SbiFunction(0, 0x54494D45);
const REMOTE_SFENCE_VMA: SbiFunction = SbiFunction(1, 0x54494D45);
const REMOTE_SFENCE_VMA_ASID: SbiFunction = SbiFunction(2, 0x54494D45);
const REMOTE_HFENCE_GVMA_VMID: SbiFunction = SbiFunction(3, 0x54494D45);
const REMOTE_HFENCE_GVMA: SbiFunction = SbiFunction(4, 0x54494D45);
const REMOTE_HFENCE_VVMA_ASID: SbiFunction = SbiFunction(5, 0x54494D45);
const REMOTE_HFENCE_VVMA: SbiFunction = SbiFunction(6, 0x54494D45);

#[repr(isize)]
pub enum SbiError {
    NoShmem = -9,
    AlreadyStopped,
    AlreadyStarted,
    AlreadyAvailable,
    InvalidAddress,
    Denied,
    InvalidParam,
    NotSupport,
    Failed,
}

impl fmt::Debug for SbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed => write!(f, "Failed"),
            Self::NotSupport => write!(f, "Not support"),
            Self::InvalidParam => write!(f, "Invalid parameter(s)"),
            Self::Denied => write!(f, "Denied or not allowed"),
            Self::InvalidAddress => write!(f, "Invalid address(s)"),
            Self::AlreadyAvailable => write!(f, "Already available"),
            Self::AlreadyStarted => write!(f, "Already started"),
            Self::AlreadyStopped => write!(f, "Already stopped"),
            Self::NoShmem => write!(f, "Shared memory not available"),
        }
    }
}

pub fn system_reset() {
    sbi_call(SYSTEM_RESET, [0; 6]);
}

pub fn debug_console_write(num_bytes: usize, base_addr_lo: usize, base_addr_hi: usize) {
    sbi_call(DEBUG_CONSOLE_WRITE, [num_bytes, base_addr_lo, base_addr_hi, 0, 0, 0]);
}

pub fn debug_console_write_byte(c: char) {
    sbi_call(DEBUG_CONSOLE_WRITE_BYTE, [c as usize, 0, 0, 0, 0, 0]);
}

pub fn console_getchar() -> Result<isize, SbiError> {
    let (error_code, result) = sbi_call(DEBUG_CONSOLE_READ,  [0; 6]);
    if error_code != 0 {
        Err(unsafe {
            transmute(error_code)
        })
    } else {
        Ok(result)
    }
}

#[inline(always)]
fn sbi_call(function: SbiFunction, args: [usize; 6]) -> (isize, isize) {
    let error_code: isize;
    let value: isize;
    unsafe {
        asm!("ecall",
            inlateout("a0") args[0] => error_code,
            inlateout("a1") args[1] => value,
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a6") function.0,
            in("a7") function.1,
        );
    }
    (error_code, value)
}
