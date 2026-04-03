use crate::constants::EventType;
use std::ffi::{c_int, c_void};
use std::io::Error;
use std::os::fd::RawFd;
use std::ptr::null_mut;

#[repr(C)]
pub union epoll_data_t {
    pub ptr: *mut c_void,
    pub fd: i32,
    pub uint32: u32,
    pub uint64: u64,
}

#[repr(C, packed)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data_t,
}

pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
// pub const EPOLL_CTL_MOD: i32 = 3;

unsafe extern "C" {
    fn epoll_create1(flags: c_int) -> RawFd;

    fn epoll_ctl(epoll_fd: c_int, operation: c_int, fd: c_int, event: *mut epoll_event) -> c_int;

    fn epoll_wait(epoll_fd: c_int, events: *mut epoll_event, n: c_int, timeout: c_int) -> c_int;
}

pub(crate) fn epoll_create() -> Result<RawFd, Error> {
    let fd: RawFd = unsafe { epoll_create1(0) };
    if fd.is_negative() {
        println!("fuck");
        return Err(Error::last_os_error());
    }

    Ok(fd)
}

pub(crate) fn epoll_add(epoll_fd: RawFd, file: RawFd, mode: EventType) -> bool {
    let event: epoll_event = epoll_event {
        events: mode as u32,
        data: epoll_data_t {
            uint32: file as u32,
        },
    };

    let ptr = Box::into_raw(Box::new(event));
    0 == unsafe { epoll_ctl(epoll_fd, EPOLL_CTL_ADD, file, ptr) }
}

pub(crate) fn epoll_remove(epoll_fd: RawFd, file: RawFd) -> bool {
    0 == unsafe { epoll_ctl(epoll_fd, EPOLL_CTL_DEL, file, null_mut()) }
}

pub(crate) fn epoll_wait_single_event(epoll_fd: RawFd) -> Result<i32, Error> {
    let mut callback: Box<epoll_event> = Box::new(epoll_event {
        events: 0,
        data: epoll_data_t { ptr: null_mut() },
    });

    let result = unsafe { epoll_wait(epoll_fd, callback.as_mut(), 1, -1) };
    if 0 > result {
        return Err(Error::last_os_error());
    }

    unsafe { Ok(callback.data.uint32 as i32) }
}
