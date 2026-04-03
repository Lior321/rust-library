use std::os::fd::RawFd;

pub trait EpollEvent {
    fn handle(&self, fd: RawFd) -> Option<bool>;
}
