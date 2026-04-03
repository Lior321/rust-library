use crate::constants::EventType::EpollIn;
use crate::epoll_event::{EpollEvent, EventResult};
use crate::libc_wrapper::{epoll_add, epoll_create, epoll_remove, epoll_wait_single_event};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::os::fd::RawFd;

pub enum ESMActionResult {
    Success,
    Failed,
}

pub struct ESM {
    epoll_fd: RawFd,
    map: HashMap<RawFd, Box<dyn EpollEvent>>,
}

impl ESM {
    pub fn new() -> Result<ESM, Error> {
        Ok(ESM {
            epoll_fd: epoll_create()?,
            map: HashMap::new(),
        })
    }

    pub fn add_event(&mut self, fd: RawFd, callback: Box<dyn EpollEvent>) -> bool {
        if fd.is_negative() {
            return false;
        }

        let result = epoll_add(self.epoll_fd, fd, EpollIn);
        if result {
            self.map.insert(fd, callback);
        }

        result
    }

    pub fn remove_event(&mut self, file: RawFd) -> bool {
        let result = epoll_remove(self.epoll_fd, file);
        if result {
            self.map.remove(&file);
        }

        result
    }

    pub fn dispatch(&self) -> Result<EventResult, Error> {
        let callback = RawFd::from(epoll_wait_single_event(self.epoll_fd)?);
        Ok(self.map[&callback].handle(callback))
    }

    pub fn dispatch_indefinitely(&self) -> Result<EventResult, Error> {
        loop {
            let result = self.dispatch()?;

            match result {
                EventResult::Fatal => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Fatal error received during event handle",
                    ));
                }
                _ => {}
            }
        }
    }
}
