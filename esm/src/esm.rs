use crate::constants::EventType::EpollIn;
use crate::epoll_event::{ESMActionResult, EpollEvent};
use crate::esm_error::EsmError;
use crate::libc_wrapper::{
    epoll_add, epoll_create, epoll_data_t, epoll_remove, epoll_wait_single_event,
};
use std::os::fd::RawFd;

pub struct ESM<T: EpollEvent> {
    epoll_fd: RawFd,
    events: Vec<Option<(RawFd, T)>>,
}

impl<T: EpollEvent> ESM<T> {
    pub fn new() -> Result<ESM<T>, EsmError> {
        Ok(ESM {
            epoll_fd: epoll_create()?,
            events: Vec::new(),
        })
    }

    pub fn add_event(&mut self, fd: RawFd, callback: T) -> Result<usize, EsmError> {
        if fd.is_negative() {
            return Err(EsmError::InvalidArgument(format!("fd {} is negative", fd)));
        }

        self.events.push(Some((fd, callback)));
        epoll_add(
            &self.epoll_fd,
            &fd,
            EpollIn,
            epoll_data_t {
                uint64: (self.events.len() - 1) as u64,
            },
        )?;
        Ok(self.events.len() - 1)
    }

    pub fn remove_event(&mut self, index: usize) -> Result<(), EsmError> {
        match &self.events[index] {
            Some((fd, _)) => {
                epoll_remove(&self.epoll_fd, &fd)?;
                self.events[index] = None;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn dispatch(&mut self) -> Result<ESMActionResult, EsmError> {
        let id = unsafe { epoll_wait_single_event(&self.epoll_fd)?.uint64 } as u64;
        match &mut self.events[id as usize] {
            None => Err(EsmError::InvalidIdentifier(id)),
            Some((_, event)) => Ok(event.handle()),
        }
    }

    pub fn dispatch_indefinitely(&mut self) -> Result<(), EsmError> {
        loop {
            let result: ESMActionResult = self.dispatch()?;

            match result {
                ESMActionResult::Failed => eprintln!("Handling of event failed, see previous logs"),
                ESMActionResult::Success => {}
            }
        }
    }
}
