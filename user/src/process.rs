use crate::system_call;

pub fn exit(exit_code: i32) {
    system_call::process::exit(exit_code);
}
