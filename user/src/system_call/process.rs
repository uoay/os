use common::system_call::EXIT;

use super::system_call;

pub(crate) fn exit(exit_code: i32) {
    system_call(EXIT, [exit_code as usize, 0, 0, 0, 0, 0, 0]);
}
