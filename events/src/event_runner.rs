use crate::event::EventHandler;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, Once};
use std::thread;

pub struct EventQueue<T: EventHandler> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T: EventHandler> EventQueue<T> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        })
    }

    pub fn push(&self, event: T) {
        self.queue
            .lock()
            .expect("Failed to lock the event queue")
            .push_back(event);
        self.condvar.notify_one();
    }

    pub fn pop_blocking(&self) -> Option<T> {
        let mut queue = match self.queue.lock() {
            Err(_) => return None,
            Ok(event) => event,
        };

        loop {
            if let Some(event) = queue.pop_front() {
                return Some(event);
            }
            queue = self.condvar.wait(queue).unwrap();
        }
    }

    pub fn try_pop(&self) -> Option<T> {
        match self.queue.lock() {
            Err(_) => None,
            Ok(mut dequeue) => dequeue.pop_front(),
        }
    }
}

pub struct EventManager<T: EventHandler + 'static> {
    queue: Arc<EventQueue<T>>,
    has_started: Once,
}

impl<T: EventHandler + 'static> EventManager<T> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: EventQueue::new(),
            has_started: Once::new(),
        })
    }

    /// Returns a handle producers can use to push events
    pub fn queue(&self) -> Arc<EventQueue<T>> {
        Arc::clone(&self.queue)
    }

    /// Spawns the consumer thread. Runs until the process exits. Can be called exactly once
    pub fn start(&self) {
        self.has_started.call_once(move || {
            let queue = Arc::clone(&self.queue);
            match queue.pop_blocking() {
                None => true,
                Some(mut event) => event.handle(),
            };

            thread::spawn(move || {
                loop {
                    match queue.pop_blocking() {
                        None => false,
                        Some(_) => true,
                        // Some(mut event) => event.handle(),
                    };
                }
            });
        })
    }
}
