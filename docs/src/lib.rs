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

/// Advanced Features (16-20)
/// ==========================
///
/// These sections cover advanced Rust concepts.
///
/// - **Pin and PhantomPinned**: Self-referential structs
/// - **Future and Async**: Manual future implementation
/// - **Rc and RefCell**: Shared ownership and interior mutability
/// - **Any Type**: Dynamic typing and downcasting
/// - **Panic Handling**: Safe panic recovery