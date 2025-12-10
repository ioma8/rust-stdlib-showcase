//! Pin and PhantomPinned
//!
//! Advanced Rust features for working with self-referential structs and pinned data.
//! This documentation explains the `Pin` type and `PhantomPinned` marker for creating
//! structs that cannot be moved.
//!
//! ## 📖 Overview
//!
//! `Pin` and `PhantomPinned` are advanced Rust features that enable:
//! - Self-referential structs (structs that contain pointers to themselves)
//! - Async/await implementation
//! - Safe handling of data that must not move
//!
//! ## 🎯 Key Concepts
//!
//! ### What is Pin?
//!
//! `Pin` is a wrapper type that prevents the inner value from being moved.
//! This is crucial for self-referential structs where pointers would become
//! invalid if the struct moved.
//!
//! ```rust
//! use std::pin::Pin;
//! use std::marker::PhantomPinned;
//!
//! struct SelfReferential {
//!     data: String,
//!     data_ptr: *const String,
//!     _pin: PhantomPinned,  // Prevents automatic Unpin implementation
//! }
//!
//! impl SelfReferential {
//!     fn new(data: String) -> Pin<Box<Self>> {
//!         let mut boxed = Box::pin(SelfReferential {
//!             data,
//!             data_ptr: std::ptr::null(),
//!             _pin: PhantomPinned,
//!         });
//!
//!         let data_ptr = &boxed.data as *const String;
//!
//!         // SAFETY: We're setting the pointer while the data is pinned
//!         unsafe {
//!             let mut_ref = Pin::as_mut(&mut boxed);
//!             Pin::get_unchecked_mut(mut_ref).data_ptr = data_ptr;
//!         }
//!
//!         boxed
//!     }
//!
//!     fn get_data(&self) -> &str {
//!         &self.data
//!     }
//! }
//! ```
//!
//! ### PhantomPinned
//!
//! `PhantomPinned` is a marker type that prevents automatic `Unpin` implementation.
//! This ensures that the struct cannot be moved even when pinned.
//!
//! ```rust
//! use std::marker::PhantomPinned;
//!
//! struct CannotMove {
//!     _pin: PhantomPinned,
//!     // Other fields...
//! }
//! ```
//!
//! ### Pinning Types
//!
//! There are several ways to pin data:
//!
//! ```rust
//! use std::pin::Pin;
//!
//! // Pin a Box
//! let pinned_box = Box::pin(value);
//!
//! // Pin a reference
//! let pinned_ref = Pin::new(&value);
//!
//! // Pin on the stack (nightly only)
//! // let pinned_stack = Pin::new_unchecked(&mut value);
//! ```
//!
//! ## 🔧 When to Use Pin
//!
//! ### 1. Self-Referential Structs
//!
//! When a struct contains pointers to its own fields:
//!
//! ```rust
//! struct SelfRef {
//!     data: String,
//!     ptr_to_data: *const String,
//!     _pin: PhantomPinned,
//! }
//! ```
//!
//! ### 2. Async/Await Implementation
//!
//! The Rust async/await system relies heavily on `Pin` to ensure futures
//! remain in place during polling.
//!
//! ### 3. FFI with C APIs
//!
//! When interfacing with C code that expects stable memory addresses.
//!
//! ## 🚨 Safety Considerations
//!
//! ### Unsafe Code Required
//!
//! Working with `Pin` often requires `unsafe` blocks because:
//! - You're dealing with raw pointers
//! - You're bypassing Rust's normal safety guarantees
//! - You're responsible for ensuring memory safety
//!
//! ### Common Pitfalls
//!
//! 1. **Moving Pinned Data**: Never move data that's supposed to be pinned
//! 2. **Invalid Pointers**: Ensure pointers remain valid
//! 3. **Double Pinning**: Avoid pinning already-pinned data
//!
//! ## 📚 Official Documentation
//!
//! - [std::pin module](https://doc.rust-lang.org/std/pin/index.html)
//! - [Pin struct](https://doc.rust-lang.org/std/pin/struct.Pin.html)
//! - [PhantomPinned](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html)
//!
//! ## 🎓 See Also
//!
//! - [Future and Async](../future_and_async/index.html) - How Pin is used in async Rust
//! - [Memory Operations](../../intermediate/memory_operations/index.html) - Understanding memory layout