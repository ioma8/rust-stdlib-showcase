# Rust Standard Library Showcase 🦀

A comprehensive demonstration of **20 essential Rust standard library features** for developers getting familiar with Rust.

## 🎯 Purpose

This repository serves as an **educational resource** for Rust developers who want to:

- Learn the most important Rust standard library features
- See practical examples of Rust's powerful abstractions
- Understand how to use Rust's core functionality
- Get hands-on experience with both basic and advanced concepts

## 🚀 Features Demonstrated

The showcase demonstrates **20 key Rust standard library features**:

### **Basic Features**
1. **Threading** - Named threads and thread management
2. **Time Operations** - Timing and duration handling
3. **Collections** - HashMap and HashSet usage
4. **File I/O** - File creation, reading, and writing
5. **Path Operations** - Filesystem path manipulation
6. **Synchronization** - Arc and Mutex for thread-safe sharing
7. **Environment** - Accessing environment variables
8. **Process Execution** - Spawning subprocesses
9. **Error Handling** - Result pattern matching
10. **Option Type** - Some/None handling

### **Intermediate Features**
11. **Network Operations** - TCP listener creation
12. **Iterators** - Functional programming with iterators
13. **Custom Formatting** - Implementing Display trait
14. **Memory Operations** - Size, alignment, and capacity management
15. **Operator Overloading** - Implementing Add trait

### **Advanced Features**
16. **Pin and PhantomPinned** - Self-referential structs
17. **Future and Async** - Manual future implementation
18. **Rc and RefCell** - Shared ownership and interior mutability
19. **Any Type** - Dynamic typing and downcasting
20. **Panic Handling** - Safe panic recovery

## 📚 How to Use

### Prerequisites
- Rust toolchain installed (`rustc`, `cargo`)
- Basic familiarity with Rust syntax

### Running the Showcase

```bash
# Clone the repository
git clone https://github.com/ioma8/rust-stdlib-showcase.git
cd rust-stdlib-showcase

# Build and run
cargo build
cargo run
```

### Exploring the Documentation

This repository includes **comprehensive documentation** built with mdBook:

```bash
# Build documentation locally
cd docs
mdbook serve
# Then open http://localhost:3000
```

Or visit the **online documentation**: [https://ioma8.github.io/rust-stdlib-showcase/](https://ioma8.github.io/rust-stdlib-showcase/)

The documentation includes:
- **20 dedicated pages** - One for each feature with detailed explanations
- **Code examples** - Real code from the showcase with annotations
- **Best practices** - Recommended patterns and anti-patterns
- **Search functionality** - Quick navigation to any topic

### Expected Output
The program will execute all 20 features sequentially, showing:
- Clear section headers for each feature
- Practical examples with real output
- Error handling demonstrations
- Memory and performance insights

## 🎓 Learning Path

This showcase is organized to help you learn Rust progressively:

1. **Start with basics** (features 1-10) - Core Rust functionality
2. **Move to intermediate** (features 11-15) - Common patterns and traits
3. **Master advanced** (features 16-20) - Advanced Rust concepts

## 🔧 Code Structure

```
src/
└── main.rs          # Contains all 20 feature demonstrations

README.md           # This file - documentation and usage guide
Cargo.toml          # Project configuration (no external dependencies)
```

## 💡 Key Learning Points

- **Memory Safety**: See how Rust ensures safety even with complex patterns
- **Zero-Cost Abstractions**: Learn how Rust provides high-level features without runtime overhead
- **Fearless Concurrency**: Understand Rust's approach to thread safety
- **Trait System**: Explore how traits enable polymorphism and code reuse
- **Error Handling**: Master Rust's approach to robust error management

## 🤝 Contributing

This is an educational resource, and contributions are welcome!

- Found a better way to demonstrate a feature? Open a PR!
- Want to add more features? Let's discuss!
- Found an issue? Please report it!

## 📚 Documentation Site

This project includes a **comprehensive documentation site** built with mdBook:

### 🌐 Online Documentation

📖 **[Visit the Documentation Site](https://ioma8.github.io/rust-stdlib-showcase/)**

The online documentation provides:
- **20 individual feature pages** - Each feature explained in depth
- **Three difficulty levels** - Basic, Intermediate, and Advanced
- **Search functionality** - Find what you need quickly
- **Code examples** - Real examples with detailed explanations
- **Best practices** - Recommended patterns and anti-patterns

### 📁 Documentation Structure

```
docs/
├── src/
│   ├── SUMMARY.md          # Table of contents
│   ├── introduction.md     # Getting started guide
│   ├── basic/              # Features 1-10 (threading, collections, etc.)
│   ├── intermediate/       # Features 11-15 (networking, iterators, etc.)
│   ├── advanced/           # Features 16-20 (Pin, async, etc.)
│   └── contributing.md     # Contribution guidelines
├── book.toml              # mdBook configuration
└── build.sh               # Build script
```

### 🚀 Building Locally

```bash
# Navigate to docs directory
cd docs

# Serve with live reload
mdbook serve

# Or build static site
mdbook build
```

### 🔧 Automatic Deployment

The documentation is automatically deployed to GitHub Pages using GitHub Actions:
- **Workflow**: `.github/workflows/deploy-docs.yml`
- **Trigger**: Pushes to main branch
- **URL**: [https://ioma8.github.io/rust-stdlib-showcase/](https://ioma8.github.io/rust-stdlib-showcase/)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust team for creating such an amazing language
- The Rust community for their excellent documentation
- All contributors who help improve this educational resource

## 🌟 Why This Showcase?

Unlike fragmented tutorials, this repository provides:
- **All in one place**: 20 features in a single, working example
- **Practical examples**: Real code you can run and experiment with
- **Progressive learning**: From basic to advanced concepts
- **No external dependencies**: Pure Rust standard library
- **Educational focus**: Designed for learning, not production

Perfect for:
- Rust beginners who want to see real examples
- Intermediate developers looking to expand their knowledge
- Educators teaching Rust concepts
- Anyone preparing for Rust interviews

**Happy Rusting!** 🦀🔥