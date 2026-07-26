pub trait EventHandler: Send {
    fn handle(&mut self) -> bool;
}
