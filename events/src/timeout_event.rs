use crate::event::EventHandler;
use crate::event_runner::EventManager;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn count_once<T: EventHandler>(event_manager: Arc<EventManager<T>>, event: T, time: Duration) {
    thread::spawn(move || {
        thread::sleep(time);
        event_manager.queue().push(event);
    });
}

pub fn count_on_interval<T: EventHandler + Clone>(
    event_manager: Arc<EventManager<T>>,
    event: T,
    initial_timeout: Duration,
    interval: Duration,
) {
    thread::spawn(move || {
        thread::sleep(initial_timeout);
        event_manager.queue().push(event.clone());
        loop {
            thread::sleep(interval);
            event_manager.queue().push(event.clone());
        }
    });
}
