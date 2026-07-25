pub enum ESMActionResult {
    Success,
    Failed,
}

pub trait EpollEvent {
    fn handle(&mut self) -> ESMActionResult;
}
