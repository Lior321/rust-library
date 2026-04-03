use crate::event::EventHandle;
use crate::event_runner::EventManager;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn count_once(event_manager: Arc<EventManager>, event: EventHandle, time: Duration) {
    thread::spawn(move || {
        thread::sleep(time);
        event_manager.queue().push(event);
    });
}

pub fn count_on_interval(
    event_manager: Arc<EventManager>,
    event: EventHandle,
    initial_timeout: Duration,
    interval: Duration,
) {
    thread::spawn(move || {
        thread::sleep(initial_timeout);
        event_manager.queue().push(event.clone_ref());
        loop {
            thread::sleep(interval);
            event_manager.queue().push(event.clone_ref());
        }
    });
}
