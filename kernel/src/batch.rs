use core::arch::asm;

use crate::{println, trap::TrapContext};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_stack_pointer(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    fn push_context(&self, context: TrapContext) -> &'static mut TrapContext{
        let ptr = (self.get_stack_pointer() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ptr = context;
            ptr.as_mut().unwrap()
        }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_stack_pointer(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: KernelStack = KernelStack {data: [0; KERNEL_STACK_SIZE]};
static USER_STACK: UserStack = UserStack {data: [0; USER_STACK_SIZE]};

pub const MAX_APP_NUM: usize = 1;

pub static mut APP_MANAGER: AppManager = AppManager {
    num_app: 0,
    current_app: 0,
    app_start: [0; MAX_APP_NUM +1],
};

pub struct AppManager {
    pub num_app: usize,
    pub current_app: usize,
    pub app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!")
        }
        println!("[kernel] Loading app_{}", app_id);

        core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            APP_SIZE_LIMIT
        ).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            app_src.len()
        );
        app_dst.copy_from_slice(app_src);

        asm!("fence.i");
    }

    pub fn run_next_app(&mut self) -> ! {
        unsafe {
            self.load_app(self.current_app);
        }
        self.current_app += 1;
        extern "C" {
            fn __trapret(context_address: usize);
        }
        unsafe {
            __trapret(KERNEL_STACK.push_context(
                TrapContext::init(
                    APP_BASE_ADDRESS,
                    USER_STACK.get_stack_pointer()
                )
            ) as *const _ as usize);
        }
        panic!();
    }
}
