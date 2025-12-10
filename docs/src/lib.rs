//! # Rust Standard Library Showcase Documentation
//!
//! A comprehensive guide to 20 essential Rust standard library features.
//! This documentation provides detailed explanations and examples for each feature
//! demonstrated in the main showcase.
//!
//! ## 🎯 Purpose
//!
//! This documentation serves as a companion to the [main showcase repository](https://github.com/ioma8/rust-stdlib-showcase)
//! and provides in-depth explanations of each Rust feature.
//!
//! ## 📚 Features Covered
//!
//! The documentation is organized into three main sections:
//!
//! ### Basic Features (1-10)
//! - Foundational Rust concepts every developer should know
//!
//! ### Intermediate Features (11-15)
//! - Common patterns and traits used in Rust development
//!
//! ### Advanced Features (16-20)
//! - Powerful Rust features for experienced developers
//!
//! ## 🚀 Getting Started
//!
//! To explore the documentation:
//! 
//! 1. **Browse online**: Visit the [GitHub Pages site](https://ioma8.github.io/rust-stdlib-showcase/)
//! 2. **Generate locally**: Run `cargo doc --open` in the docs directory
//! 3. **Read sequentially**: Start with basic features and progress to advanced
//!
//! ## 📖 Navigation
//!
//! Use the sidebar to navigate between different features. Each feature has:
//! - Detailed explanation of the concept
//! - Practical code examples
//! - Use cases and best practices
//! - Links to official Rust documentation
//!
//! ## 🤝 Contributing
//!
//! Found an issue or want to improve the documentation?
//! - Open an issue on [GitHub](https://github.com/ioma8/rust-stdlib-showcase/issues)
//! - Submit a pull request with your improvements
//! - See [CONTRIBUTING.md](https://github.com/ioma8/rust-stdlib-showcase/blob/main/CONTRIBUTING.md)

///
/// These modules cover foundational Rust concepts.
pub mod basic {
    /// Threading in Rust
    pub mod threading;
    /// Time operations
    pub mod time_operations;
    /// Collections
    pub mod collections;
    /// File I/O
    pub mod file_io;
    /// Path operations
    pub mod path_operations;
    /// Synchronization
    pub mod synchronization;
    /// Environment
    pub mod environment;
    /// Process execution
    pub mod process_execution;
    /// Error handling
    pub mod error_handling;
    /// Option type
    pub mod option_type;
}

/// Intermediate Features (11-15)
/// ==============================
///
/// These modules cover common Rust patterns and traits.
pub mod intermediate {
    /// Network operations
    pub mod network_operations;
    /// Iterators
    pub mod iterators;
    /// Custom formatting
    pub mod custom_formatting;
    /// Memory operations
    pub mod memory_operations;
    /// Operator overloading
    pub mod operator_overloading;
}

/// Advanced Features (16-20)
/// ==========================
///
/// These modules cover advanced Rust concepts.
pub mod advanced {
    /// Pin and PhantomPinned
    pub mod pin_and_phantom_pinned;
    /// Future and async
    pub mod future_and_async;
    /// Rc and RefCell
    pub mod rc_and_refcell;
    /// Any type
    pub mod any_type;
    /// Panic handling
    pub mod panic_handling;
}
=======
///
/// These sections cover foundational Rust concepts.
///
/// - **Threading**: Named threads and thread management
/// - **Time Operations**: Timing and duration handling  
/// - **Collections**: HashMap and HashSet usage
/// - **File I/O**: File creation, reading, and writing
/// - **Path Operations**: Filesystem path manipulation
/// - **Synchronization**: Arc and Mutex for thread-safe sharing
/// - **Environment**: Accessing environment variables
/// - **Process Execution**: Spawning subprocesses
/// - **Error Handling**: Result pattern matching
/// - **Option Type**: Some/None handling
///
/// See the [main showcase](https://github.com/ioma8/rust-stdlib-showcase) for complete examples.
=======
/// Basic Features (1-10)
/// =====================
///
/// These modules cover foundational Rust concepts.
pub mod basic {
    /// Threading in Rust
    pub mod threading;
}

/// Intermediate Features (11-15)
/// ==============================
///
/// These sections cover common Rust patterns and traits.
///
/// - **Network Operations**: TCP listener creation
/// - **Iterators**: Functional programming with iterators
/// - **Custom Formatting**: Implementing Display trait
/// - **Memory Operations**: Size, alignment, and capacity management
/// - **Operator Overloading**: Implementing Add trait
///
/// Advanced Features (16-20)
/// ==========================
///
/// These sections cover advanced Rust concepts.
///
/// - **Pin and PhantomPinned**: Self-referential structs
/// - **Future and Async**: Manual future implementation
/// - **Rc and RefCell**: Shared ownership and interior mutability
/// - **Any Type**: Dynamic typing and downcasting
/// - **Panic Handling**: Safe panic recovery=====================
///
/// These sections cover foundational Rust concepts.
///
/// - **Threading**: Named threads and thread management
/// - **Time Operations**: Timing and duration handling  
/// - **Collections**: HashMap and HashSet usage
/// - **File I/O**: File creation, reading, and writing
/// - **Path Operations**: Filesystem path manipulation
/// - **Synchronization**: Arc and Mutex for thread-safe sharing
/// - **Environment**: Accessing environment variables
/// - **Process Execution**: Spawning subprocesses
/// - **Error Handling**: Result pattern matching
/// - **Option Type**: Some/None handling
///
/// See the [main showcase](https://github.com/ioma8/rust-stdlib-showcase) for complete examples.
///
/// Intermediate Features (11-15)
/// ==============================
///
/// These sections cover common Rust patterns and traits.
///
/// - **Network Operations**: TCP listener creation
/// - **Iterators**: Functional programming with iterators
/// - **Custom Formatting**: Implementing Display trait
/// - **Memory Operations**: Size, alignment, and capacity management
/// - **Operator Overloading**: Implementing Add trait
///
/// Advanced Features (16-20)
/// ==========================
///
/// These sections cover advanced Rust concepts.
///
/// - **Pin and PhantomPinned**: Self-referential structs
/// - **Future and Async**: Manual future implementation
/// - **Rc and RefCell**: Shared ownership and interior mutability
/// - **Any Type**: Dynamic typing and downcasting
/// - **Panic Handling**: Safe panic recovery=====================
///
/// These modules cover foundational Rust concepts.
pub mod basic {
    /// Threading in Rust
    pub mod threading;
    /// Time operations
    pub mod time_operations;
    /// Collections
    pub mod collections;
    /// File I/O
    pub mod file_io;
    /// Path operations
    pub mod path_operations;
    /// Synchronization
    pub mod synchronization;
    /// Environment
    pub mod environment;
    /// Process execution
    pub mod process_execution;
    /// Error handling
    pub mod error_handling;
    /// Option type
    pub mod option_type;
}

/// Intermediate Features (11-15)
/// ==============================
///
/// These modules cover common Rust patterns and traits.
pub mod intermediate {
    /// Network operations
    pub mod network_operations;
    /// Iterators
    pub mod iterators;
    /// Custom formatting
    pub mod custom_formatting;
    /// Memory operations
    pub mod memory_operations;
    /// Operator overloading
    pub mod operator_overloading;
}

/// Advanced Features (16-20)
/// ==========================
///
/// These modules cover advanced Rust concepts.
pub mod advanced {
    /// Pin and PhantomPinned
    pub mod pin_and_phantom_pinned;
    /// Future and async
    pub mod future_and_async;
    /// Rc and RefCell
    pub mod rc_and_refcell;
    /// Any type
    pub mod any_type;
    /// Panic handling
    pub mod panic_handling;
}