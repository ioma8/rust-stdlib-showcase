#!/bin/bash

# Build documentation for Rust Standard Library Showcase

echo "📚 Building Rust documentation..."

# Navigate to docs directory
cd "$(dirname "$0")"

# Generate documentation using cargo doc
echo "🔧 Generating documentation with cargo doc..."
cargo doc --no-deps --open

# Check if generation was successful
if [ $? -eq 0 ]; then
    echo "✅ Documentation generated successfully!"
    echo "📁 Documentation located in: target/doc/rust_stdlib_showcase_docs/"
else
    echo "❌ Documentation generation failed!"
    exit 1
fi

echo "🎉 Documentation build complete!"