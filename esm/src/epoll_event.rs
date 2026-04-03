pub trait EpollEvent {
    fn handle(&mut self) -> Option<bool>;
}
