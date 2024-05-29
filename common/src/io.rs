pub struct FileDescriptor(pub usize);

pub const STDIN: FileDescriptor = FileDescriptor(0);
pub const STDOUT: FileDescriptor = FileDescriptor(1);
pub const STDERR: FileDescriptor = FileDescriptor(2);