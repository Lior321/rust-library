use crate::event::EventHandler;
use crate::event_runner::EventManager;
use std::thread;
use std::time::Duration;

pub fn count_once<T: EventHandler>(event_manager: &EventManager<T>, event: T, time: Duration) {
    let queue = event_manager.queue();
    thread::spawn(move || {
        thread::sleep(time);
        queue.send(event).expect("Error: Failed to send event");
    });
}

pub fn count_on_interval<T: EventHandler + Clone>(
    event_manager: &EventManager<T>,
    event: T,
    initial_timeout: Duration,
    interval: Duration,
) {
    let queue = event_manager.queue();
    thread::spawn(move || {
        thread::sleep(initial_timeout);
        queue
            .send(event.clone())
            .expect("Error: Failed to send event");
        loop {
            thread::sleep(interval);
            queue
                .send(event.clone())
                .expect("Error: Failed to send event");
        }
    });
}
