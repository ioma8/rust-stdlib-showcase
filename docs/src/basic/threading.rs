//! Threading in Rust
//!
//! Rust provides powerful and safe threading capabilities through the `std::thread` module.
//! This documentation covers the basics of thread creation, management, and synchronization.
//!
//! ## 📖 Overview
//!
//! Threading in Rust is designed to be both powerful and safe. The standard library provides:
//! - Thread creation with `thread::spawn`
//! - Thread naming with `thread::Builder`
//! - Thread identification with `thread::current()`
//! - Thread synchronization with `join()`
//!
//! ## 🎯 Key Concepts
//!
//! ### Thread Creation
//! 
//! The simplest way to create a thread is using `thread::spawn`:
//! ```rust
//! use std::thread;
//!
//! let handler = thread::spawn(|| {
//!     println!("Hello from a thread!");
//! });
//!
//! handler.join().unwrap();
//! ```
//!
//! ### Named Threads
//! 
//! You can name threads for better debugging:
//! ```rust
//! use std::thread;
//!
//! let handler = thread::Builder::new()
//!     .name("worker-thread".into())
//!     .spawn(|| {
//!         let current = thread::current();
//!         println!("Thread name: {:?}", current.name());
//!     })
//!     .unwrap();
//!
//! handler.join().unwrap();
//! ```
//!
//! ### Thread Identification
//! 
//! Each thread has a unique ID:
//! ```rust
//! use std::thread;
//!
//! let handler = thread::spawn(|| {
//!     let current = thread::current();
//!     println!("Thread ID: {:?}", current.id());
//! });
//!
//! let main_thread = thread::current();
//! println!("Main thread ID: {:?}", main_thread.id());
//!
//! handler.join().unwrap();
//! ```
//!
//! ### Thread Synchronization
//! 
//! Use `join()` to wait for threads to complete:
//! ```rust
//! use std::thread;
//!
//! let handler = thread::spawn(|| {
//!     // Simulate work
//!     thread::sleep(std::time::Duration::from_secs(1));
//!     println!("Worker thread completed!");
//! });
//!
//! println!("Main thread waiting...");
//! handler.join().unwrap();
//! println!("Both threads completed!");
//! ```
//!
//! ## 🔧 Best Practices
//!
//! ### 1. Always Join Threads
//! Unjoined threads may be terminated when the main thread exits.
//!
//! ```rust
//! // ❌ Bad - thread may not complete
//! thread::spawn(|| {
//!     // Important work that might not finish
//! });
//!
//! // ✅ Good - ensure thread completes
//! let handler = thread::spawn(|| {
//!     // Important work
//! });
//! handler.join().unwrap();
//! ```
//!
//! ### 2. Handle Thread Errors
//! Thread creation can fail due to resource limitations.
//!
//! ```rust
//! use std::thread;
//!
//! let handler = thread::Builder::new()
//!     .name("worker".into())
//!     .spawn(|| {
//!         // Thread work
//!     })
//!     .expect("Failed to spawn thread");
//!
//! handler.join().expect("Failed to join thread");
//! ```
//!
//! ### 3. Use Meaningful Thread Names
//! Named threads are easier to debug and profile.
//!
//! ```rust
//! thread::Builder::new()
//!     .name("data-processor".into())  // Descriptive name
//!     .spawn(|| {
//!         // Processing work
//!     });
//! ```
//!
//! ## 🚨 Common Pitfalls
//!
//! ### 1. Data Races
//! Rust's borrow checker prevents data races at compile time, but be careful with
//! shared mutable state.
//!
//! ### 2. Deadlocks
//! Be mindful of thread synchronization patterns that could lead to deadlocks.
//!
//! ### 3. Resource Leaks
//! Ensure all threads are properly joined to avoid resource leaks.
//!
//! ## 📚 Official Documentation
//!
//! - [std::thread module](https://doc.rust-lang.org/std/thread/index.html)
//! - [Thread struct](https://doc.rust-lang.org/std/thread/struct.Thread.html)
//! - [Builder struct](https://doc.rust-lang.org/std/thread/struct.Builder.html)
//!
//! ## 🎓 See Also
//!
//! - [Synchronization](../synchronization/index.html) - For thread-safe data sharing
//! - [Error Handling](../error_handling/index.html) - For robust thread error management