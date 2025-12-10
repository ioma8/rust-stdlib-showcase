# Future and Async

## Overview

The `Future` trait is the foundation of Rust's async/await system, representing values that may not be ready yet.

## Code Example

```rust
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::pin::Pin;

struct SimpleFuture {
    value: i32,
}

impl Future for SimpleFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

let future = SimpleFuture { value: 42 };
let mut boxed_future = Box::pin(future);

// Create a dummy waker for polling
struct NoOpWaker;
impl std::task::Wake for NoOpWaker {
    fn wake(self: std::sync::Arc<Self>) {}
}
let waker: Waker = std::sync::Arc::new(NoOpWaker).into();
let mut cx = Context::from_waker(&waker);

match boxed_future.as_mut().poll(&mut cx) {
    Poll::Ready(value) => println!("Future resolved with value: {}", value),
    Poll::Pending => println!("Future is pending"),
}
```

## Key Concepts

- **Future**: Async computation that may not be complete
- **Poll**: Check if future is ready
- **Async/await**: Built on top of Future trait

## Learn More

- [std::future documentation](https://doc.rust-lang.org/std/future/)
- [Async Book](https://rust-lang.github.io/async-book/)
