#[macro_use]
extern crate syscall;

fn write(fd: usize, buf: &[u8]) -> isize {
    unsafe {
        syscall!(WRITE, fd, buf.as_ptr(), buf.len()) as isize
    }
}

fn exit(code: usize) {
    unsafe {
        syscall!(EXIT, code);
    }
}

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        syscall::platform::setup_vsyscall(argc, argv);
    }

    if write(1, "Hello, vsyscall!\n".as_bytes()) < 0 {
        exit(1);
    }
    exit(0);
    0
}
