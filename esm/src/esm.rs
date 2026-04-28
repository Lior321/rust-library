use crate::constants::EventType::EpollIn;
use crate::epoll_event::EpollEvent;
use crate::esm_error::EsmError;
use crate::libc_wrapper::{epoll_add, epoll_create, epoll_remove, epoll_wait_single_event};
use std::collections::HashMap;
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
    pub fn new() -> Result<ESM, EsmError> {
        Ok(ESM {
            epoll_fd: epoll_create()?,
            map: HashMap::new(),
        })
    }

    pub fn add_event(&mut self, fd: RawFd, callback: Box<dyn EpollEvent>) -> Result<(), EsmError> {
        if fd.is_negative() {
            return Err(EsmError::InvalidArgument(format!("fd {} is negative", fd)));
        }

        epoll_add(self.epoll_fd, fd, EpollIn)?;
        self.map.insert(fd, callback);
        Ok(())
    }

    pub fn remove_event(&mut self, file: RawFd) -> Result<(), EsmError> {
        epoll_remove(self.epoll_fd, file)?;
        self.map.remove(&file);
        Ok(())
    }

    pub fn dispatch(&mut self) -> Result<bool, EsmError> {
        let event = epoll_wait_single_event(self.epoll_fd)?;
        let event_fd = RawFd::from(event);
        Ok(self.map.get_mut(&event_fd).expect("weird").handle())
    }

    pub fn dispatch_indefinitely(&mut self) -> Result<(), EsmError> {
        loop {
            let result = self.dispatch()?;

            if !result {
                eprintln!("Handling of event failed, see previous logs")
            }
        }
    }
}
