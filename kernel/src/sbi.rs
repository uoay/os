#![allow(dead_code)]

use core::{arch::asm, fmt, mem::transmute};

#[derive(Default)]
pub struct SbiCall(i32, i32); // SbiFunction(fid, eid)

// Base Extension
const GET_SPEC_VERSION: SbiCall = SbiCall(0, 0);
const GET_IMPL_VERSION: SbiCall = SbiCall(1, 0);
const PROBE_EXTENSION: SbiCall = SbiCall(2, 0);
const GET_MVENDORID: SbiCall = SbiCall(3, 0);
const GET_MARCHID: SbiCall = SbiCall(4, 0);
const GET_MIMPID: SbiCall = SbiCall(5, 0);

// IPI Extension
const SEND_IPI: SbiCall = SbiCall(0, 0x735049);

// Debug Console Extension
const DEBUG_CONSOLE_WRITE: SbiCall = SbiCall(0, 0x4442434E);
const DEBUG_CONSOLE_READ: SbiCall = SbiCall(1, 0x4442434E);
const DEBUG_CONSOLE_WRITE_BYTE: SbiCall = SbiCall(2, 0x4442434E);

// System Reset Extension
const SYSTEM_RESET: SbiCall = SbiCall(0, 0x53525354);

// Timer Extension
const SET_TIMER: SbiCall = SbiCall(0, 0x54494D45);

// RFENCE Extension
const REMOTE_FENCE_I: SbiCall = SbiCall(0, 0x54494D45);
const REMOTE_SFENCE_VMA: SbiCall = SbiCall(1, 0x54494D45);
const REMOTE_SFENCE_VMA_ASID: SbiCall = SbiCall(2, 0x54494D45);
const REMOTE_HFENCE_GVMA_VMID: SbiCall = SbiCall(3, 0x54494D45);
const REMOTE_HFENCE_GVMA: SbiCall = SbiCall(4, 0x54494D45);
const REMOTE_HFENCE_VVMA_ASID: SbiCall = SbiCall(5, 0x54494D45);
const REMOTE_HFENCE_VVMA: SbiCall = SbiCall(6, 0x54494D45);

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
fn sbi_call(function: SbiCall, args: [usize; 6]) -> (isize, isize) {
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
