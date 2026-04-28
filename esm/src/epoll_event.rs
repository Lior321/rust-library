pub trait EpollEvent {
    fn handle(&mut self) -> bool;
}
