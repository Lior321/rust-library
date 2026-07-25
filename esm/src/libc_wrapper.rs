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
    unsafe fn epoll_create1(flags: c_int) -> RawFd;

    unsafe fn epoll_ctl(
        epoll_fd: c_int,
        operation: c_int,
        fd: c_int,
        event: *mut epoll_event,
    ) -> c_int;

    unsafe fn epoll_wait(
        epoll_fd: c_int,
        events: *mut epoll_event,
        n: c_int,
        timeout: c_int,
    ) -> c_int;
}

pub(crate) fn epoll_create() -> Result<RawFd, Error> {
    let fd: RawFd = unsafe { epoll_create1(0) };
    if fd.is_negative() {
        return Err(Error::last_os_error());
    }

    Ok(fd)
}

pub(crate) fn epoll_add(
    epoll_fd: &RawFd,
    file: &RawFd,
    mode: EventType,
    event_data: epoll_data_t,
) -> Result<(), Error> {
    let event: epoll_event = epoll_event {
        events: mode as u32,
        data: event_data,
    };

    let ptr = Box::into_raw(Box::new(event));
    if 0 != unsafe { epoll_ctl(epoll_fd.clone(), EPOLL_CTL_ADD, file.clone(), ptr) } {
        return Err(Error::last_os_error());
    }

    Ok(())
}

pub(crate) fn epoll_remove(epoll_fd: &RawFd, file: &RawFd) -> Result<(), Error> {
    if 0 != unsafe { epoll_ctl(epoll_fd.clone(), EPOLL_CTL_DEL, file.clone(), null_mut()) } {
        return Err(Error::last_os_error());
    }

    Ok(())
}

pub(crate) fn epoll_wait_single_event(epoll_fd: &RawFd) -> Result<epoll_data_t, Error> {
    let mut callback: Box<epoll_event> = Box::new(epoll_event {
        events: 0,
        data: epoll_data_t { ptr: null_mut() },
    });

    loop {
        let result = unsafe { epoll_wait(epoll_fd.clone(), callback.as_mut(), 1, -1) };
        if 0 > result {
            // 4 is EINTR which means we simply got a signal
            // could have been also timeout, but we run with unlimited timeout by feature
            if Error::last_os_error().raw_os_error().unwrap() == 4 {
                println!("shitty signal");
                continue;
            }

            return Err(Error::last_os_error());
        }

        return Ok(callback.data);
    }
}
