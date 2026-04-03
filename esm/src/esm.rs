use crate::constants::EventType::EpollIn;
use crate::epoll_event::EpollEvent;
use crate::libc_wrapper::{epoll_add, epoll_create, epoll_remove, epoll_wait_single_event};
use std::collections::HashMap;
use std::io::Error;
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

    pub fn dispatch(&self) -> Option<bool> {
        let event = epoll_wait_single_event(self.epoll_fd);
        if event.is_err() {
            return None;
        }

        let event_fd = RawFd::from(event.unwrap());
        self.map[&event_fd].handle(event_fd)
    }

    pub fn dispatch_indefinitely(&self) {
        loop {
            let result = self.dispatch();

            if result.is_none() {
                panic!("Fatal error during event handling")
            }

            if !result.unwrap() {
                eprintln!("Handling of event failed, see previous logs")
            }
        }
    }
}
