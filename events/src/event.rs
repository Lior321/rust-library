use std::sync::{Arc, Mutex};

pub trait IEvent: Send {
    fn handle(&mut self) -> bool;
}

pub struct EventHandle(pub(crate) Arc<Mutex<dyn IEvent>>);

impl EventHandle {
    pub fn new<E: IEvent + 'static>(event: E) -> Self {
        Self(Arc::new(Mutex::new(event)))
    }

    pub fn clone_ref(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
