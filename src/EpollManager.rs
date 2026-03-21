enum EventType {
    EpollIn,
    EpollOut,
    EpollEdgeTrigger,
}

struct EpollManager {}

impl EpollManager {
    fn new() -> Self {
        EpollManager {}
    }

    fn add_event<F>(event_type: EventType, mut function: F) -> Result<u32, ()>
    where
        F: FnMut() -> Result<(), ()>,
    {
        todo!()
    }

    fn remove_event(event_id: u32) -> Result<(), ()> { todo!() }

    fn poll_once() -> Option<u32> { todo!() }

    fn poll_forever() {
        loop {
            Self::poll_once();
        }
    }
}
