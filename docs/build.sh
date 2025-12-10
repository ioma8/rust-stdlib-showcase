#!/bin/bash
# Build the documentation

set -e

echo "Building documentation..."
mdbook build

echo ""
echo "✅ Documentation built successfully!"
echo "📁 Output directory: book/"
echo ""
echo "To preview locally, run: mdbook serve"
echo "Then open: http://localhost:3000"
