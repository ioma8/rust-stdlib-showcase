//! Threading Documentation
//!
//! This module demonstrates basic threading concepts in Rust.
//!
//! ## Example
//!
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
//! See the [main showcase](https://github.com/ioma8/rust-stdlib-showcase) for the complete threading example.