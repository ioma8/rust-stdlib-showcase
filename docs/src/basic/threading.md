# Threading

## Overview

Rust's threading support provides safe, fearless concurrency through the `std::thread` module. Named threads, thread builders, and thread handles make it easy to create and manage concurrent execution.

## Code Example

```rust
use std::thread;

let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        println!("Inner thread ID: {:?}, Name: {:?}", 
                 handle.id(), handle.name());
    })
    .unwrap();

let main_handle = thread::current();
println!("Main thread ID: {:?}, Name: {:?}", 
         main_handle.id(), main_handle.name());
handler.join().unwrap();
```

## Explanation

- **`thread::Builder`**: Creates threads with custom configurations (name, stack size)
- **`spawn()`**: Launches a new thread with a closure
- **`thread::current()`**: Gets a handle to the current thread
- **`join()`**: Waits for a thread to complete and returns its result

## Key Concepts

### Thread Creation
Rust provides multiple ways to create threads:
- Simple: `thread::spawn(|| { /* code */ })`
- With builder: `thread::Builder::new().name("name").spawn(|| { })`

### Thread Safety
Rust's ownership system ensures thread safety at compile time. Data must be:
- `Send` - can be transferred between threads
- `Sync` - can be shared between threads safely

### Thread Handles
Thread handles (`JoinHandle<T>`) allow you to:
- Wait for thread completion with `join()`
- Get the thread's return value
- Check if thread has finished

## Use Cases

- **Parallel Processing**: Distribute work across CPU cores
- **Background Tasks**: Run long operations without blocking
- **Concurrent Servers**: Handle multiple client connections
- **Data Pipeline**: Process data in parallel stages

## Best Practices

✅ **Do:**
- Use `thread::Builder` for named threads (easier debugging)
- Always `join()` threads or handle panics
- Use channels or `Arc<Mutex<T>>` for communication

❌ **Don't:**
- Share mutable state without synchronization
- Create excessive threads (use thread pools)
- Ignore `join()` results (may hide panics)

## Related Features

- [Synchronization](./synchronization.md) - Arc and Mutex for shared state
- [Time Operations](./time.md) - Thread sleep and timing

## Learn More

- [The Rust Book - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [std::thread documentation](https://doc.rust-lang.org/std/thread/)
