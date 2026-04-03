use std::ffi::{c_char, c_int };
use std::os::fd::RawFd;
use crate::epoll_event::{EventResult, EpollEvent};

pub mod esm;
pub mod epoll_event;
mod libc_wrapper;
mod constants;


struct File {
    pub fd: RawFd,
}

unsafe extern "C" {
    // We only declare the 2-argument version of open here because
    // we aren't creating a File (which would require the 3rd `mode` argument)
    fn open(pathname: *const c_char, flags: c_int) -> c_int;
}

impl EpollEvent for File {
    fn handle(&self, _fd: RawFd) -> EventResult {
        println!("handle_event");
        EventResult::Success
    }
}

const O_RDONLY: c_int = 0;



fn main() {
    let path = "/home/lior/Programming/rust/rust-epoll/ESM/test.txt\0";
    let c_path = path.as_ptr() as *const c_char;

    let mut esm = esm::ESM::new().unwrap();

    let fd = File {fd: unsafe{ open(c_path, O_RDONLY) }};

    if fd.fd < 0 {
        eprintln!("Failed to open File. fd returned: {}", fd.fd);
        return;
    }

    let callback = Box::new(fd);
    esm.add_event(callback.fd, callback);

    esm.dispatch_indefinitely().expect("Failed to dispatch");

    println!("Hello, world!");
}
