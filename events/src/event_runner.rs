use crate::event::EventHandler;
use std::thread;

use std::sync::mpsc::{self, Receiver, Sender};

pub struct EventManager<T: EventHandler + Send + 'static> {
    sender: Sender<T>,
    receiver: Option<Receiver<T>>,
}

impl<T: EventHandler + Send + 'static> EventManager<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: sender,
            receiver: Some(receiver),
        }
    }

    /// Returns a handle producers can use to push events
    pub fn queue(&self) -> Sender<T> {
        self.sender.clone()
    }

    /// Spawns the consumer thread. Runs until the process exits. Will start the process exectly once
    pub fn start(&mut self) {
        if !self.receiver.is_none() {
            let receiver = self.receiver.take().unwrap();

            thread::spawn(move || {
                while let Ok(mut event) = receiver.recv() {
                    event.handle();
                }
            });
        }
    }
}
