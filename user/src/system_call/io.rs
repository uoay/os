use common::{io::FileDescriptor, system_call::WRITE};

use super::system_call;

pub(crate) fn write(file_descriptor: FileDescriptor, buffer: &[u8]) -> isize {
    system_call(
        WRITE,
        [file_descriptor.0, buffer.as_ptr() as usize, buffer.len(), 0, 0, 0, 0],
    )
}