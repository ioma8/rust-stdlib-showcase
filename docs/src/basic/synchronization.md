# Synchronization

## Overview

Rust provides thread-safe primitives through `Arc` (Atomic Reference Counting) and `Mutex` (Mutual Exclusion). These enable safe shared state across threads.

## Code Example

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for i in 0..3 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
        println!("Thread {} incremented counter to {}", i, *num);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Final counter value: {}", *counter.lock().unwrap());
```

## Explanation

- **`Arc<T>`**: Atomic reference-counted pointer for shared ownership
- **`Mutex<T>`**: Mutual exclusion lock for interior mutability
- **`Arc::clone()`**: Increments reference count (cheap operation)
- **`lock()`**: Acquires the mutex, blocks if held by another thread
- **`move`**: Transfers ownership into the thread closure

## Key Concepts

### Arc - Shared Ownership
`Arc<T>` allows multiple owners of the same data:
```rust
let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);  // Same data, multiple owners
```

Thread-safe alternative to `Rc<T>` (single-threaded reference counting).

### Mutex - Interior Mutability
`Mutex<T>` provides synchronized mutable access:
```rust
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}  // Lock automatically released
```

### Combined Pattern
`Arc<Mutex<T>>` is the standard pattern for shared mutable state:
- `Arc` for sharing across threads
- `Mutex` for safe mutation

## Use Cases

- **Shared Counters**: Statistics, progress tracking
- **Shared State**: Configuration, caches shared across threads
- **Thread Pools**: Work queues accessed by multiple workers
- **Resource Pools**: Database connections, file handles

## Best Practices

✅ **Do:**
- Keep critical sections (locked code) small
- Use `Arc::clone()` explicitly for clarity
- Consider `RwLock` for read-heavy workloads
- Handle lock poisoning (when thread panics while holding lock)

❌ **Don't:**
- Hold locks across await points (use async Mutex instead)
- Create deadlocks (acquire locks in consistent order)
- Clone Arc unnecessarily (passing `&Arc` is fine)
- Forget that `lock()` can panic

## Other Synchronization Primitives

### RwLock
Read-write lock - multiple readers or one writer:
```rust
use std::sync::RwLock;

let lock = Arc::new(RwLock::new(5));

// Multiple readers
let r1 = lock.read().unwrap();
let r2 = lock.read().unwrap();

// One writer (exclusive)
let mut w = lock.write().unwrap();
*w += 1;
```

### Channels
Message passing for thread communication:
```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
});

let msg = rx.recv().unwrap();
println!("{}", msg);
```

### Atomic Types
Lock-free atomic operations:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::SeqCst);
```

## Common Patterns

### Shared Resource Pool
```rust
let pool = Arc::new(Mutex::new(Vec::new()));

// Fill pool
for i in 0..10 {
    pool.lock().unwrap().push(i);
}

// Multiple threads consume from pool
let mut handles = vec![];
for _ in 0..3 {
    let pool = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        while let Some(item) = pool.lock().unwrap().pop() {
            // Process item
        }
    });
    handles.push(handle);
}
```

### Avoiding Deadlocks
```rust
// Bad: can deadlock
let lock1 = Arc::new(Mutex::new(0));
let lock2 = Arc::new(Mutex::new(0));

// Thread 1: lock1 -> lock2
// Thread 2: lock2 -> lock1  // DEADLOCK!

// Good: always acquire in same order
// Both threads: lock1 -> lock2
```

## Related Features

- [Threading](./threading.md) - Creating and managing threads
- [Rc and RefCell](../advanced/rc-refcell.md) - Single-threaded alternatives

## Learn More

- [std::sync documentation](https://doc.rust-lang.org/std/sync/)
- [The Rust Book - Shared State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
