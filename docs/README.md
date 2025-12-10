# Rust Standard Library Showcase Documentation 📚

This directory contains the **Rust documentation** for the showcase, generated using `rustdoc`.

## 🎯 Purpose

Provide **authentic Rust documentation** with the same look and feel as the official Rust docs, but focused on the 20 features demonstrated in the main showcase.

## 🚀 Building the Documentation

### Local Development

```bash
# Navigate to docs directory
cd docs

# Build documentation
./build_docs.sh

# Or manually
cargo doc --no-deps --open
```

### Documentation Structure

```
docs/
├── src/                    # Source Rust files with documentation
│   ├── lib.rs              # Main documentation hub
│   ├── basic/              # Basic features (1-10)
│   ├── intermediate/       # Intermediate features (11-15)
│   └── advanced/           # Advanced features (16-20)
├── Cargo.toml             # Documentation project configuration
├── build_docs.sh          # Build script
└── README.md              # This file
```

## 📖 Documentation Features

### Authentic Rust Docs Layout
- Same visual style as `doc.rust-lang.org`
- Search functionality
- Sidebar navigation
- Source code viewing
- Trait implementation lists

### Comprehensive Coverage
Each feature documentation includes:
- **Concept explanation** - What the feature does
- **Code examples** - Practical usage patterns
- **Best practices** - Recommended approaches
- **Common pitfalls** - What to avoid
- **Official links** - References to Rust documentation

## 🔧 Adding New Documentation

To add documentation for a new feature:

1. **Create a new module** in the appropriate category
2. **Add documentation** using Rustdoc comments (`///`)
3. **Include examples** with code blocks
4. **Link to official docs** for reference
5. **Add to main lib.rs** to include in navigation

## 🤝 Contributing

Documentation contributions are welcome! See the main [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## 📚 Online Documentation

The documentation is automatically deployed to GitHub Pages:
- **URL**: [https://ioma8.github.io/rust-stdlib-showcase/](https://ioma8.github.io/rust-stdlib-showcase/)
- **Deployment**: Triggered on pushes to main branch
- **Workflow**: `.github/workflows/docs.yml`

## 🎓 Learning Path

We recommend exploring the documentation in this order:

1. **Basic Features** - Foundational concepts
2. **Intermediate Features** - Common patterns
3. **Advanced Features** - Powerful techniques

Each section builds on the previous ones!

**Happy Learning!** 🦀📚