use std::os::fd::RawFd;

pub enum EventResult {
    Success,
    Failed,
    Fatal,
}

pub trait EpollEvent {
    fn handle(&self, fd: RawFd) -> EventResult;
}
