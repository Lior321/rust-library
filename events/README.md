# The Events crate

This crate is for a multi-producer single-consumer event manager inside a single process.

Since we have a single consumer all events execution is thread-safe.

# Important notes:
1. The is not lock on the IEvent! If you change the event from another thread you need to make `handle()` function to lock to prevent racing
2. The `EventManager.start()` can be called multiple times, but it will execute only once 