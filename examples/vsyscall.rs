#![feature(phase)]

extern crate native;

#[phase(plugin, link)]
extern crate syscall;

unsafe fn find_auxv(argc: int, argv: *const *const u8) -> *const u8 {
    let mut ptr = argv.offset(argc + 1); 

    // skip env variables
    while *ptr != std::ptr::null() {
        ptr = ptr.offset(1)
    }

    ptr.offset(1) as *const u8
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    unsafe {
        syscall::platform::setup_vsyscall(find_auxv(argc, argv));
    }
    native::start(argc, argv, main)
}

fn write(fd: uint, buf: &[u8]) {
    unsafe {
        syscall!(WRITE, fd, buf.as_ptr(), buf.len());
    }
}

fn main() {
    write(1, "Hello, vsyscall!\n".as_bytes());
}
