use crate::event::EventHandle;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, Once};
use std::thread;

pub struct EventQueue {
    queue: Mutex<VecDeque<EventHandle>>,
    condvar: Condvar,
}

impl EventQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        })
    }

    pub fn push(&self, event: EventHandle) {
        self.queue
            .lock()
            .expect("Failed to lock the event queue")
            .push_back(event);
        self.condvar.notify_one();
    }

    pub fn pop_blocking(&self) -> EventHandle {
        let mut queue = self.queue.lock().expect("Failed to lock the event queue");
        loop {
            if let Some(event) = queue.pop_front() {
                return event;
            }
            queue = self.condvar.wait(queue).unwrap();
        }
    }

    pub fn try_pop(&self) -> Option<EventHandle> {
        self.queue
            .lock()
            .expect("Failed to lock the event queue")
            .pop_front()
    }
}

pub struct EventManager {
    queue: Arc<EventQueue>,
    has_started: Once,
}

impl EventManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: EventQueue::new(),
            has_started: Once::new(),
        })
    }

    /// Returns a handle producers can use to push events
    pub fn queue(&self) -> Arc<EventQueue> {
        Arc::clone(&self.queue)
    }

    /// Spawns the consumer thread. Runs until the process exits. Can be called exactly once
    pub fn start(&self) {
        self.has_started.call_once(move || {
            let queue = Arc::clone(&self.queue);

            thread::spawn(move || {
                loop {
                    let event = queue.pop_blocking();
                    event.0.lock().expect("Failed to lock event").handle();
                }
            });
        })

    }
}
