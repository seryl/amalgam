#!/bin/bash
# Build script for Amalgam WASM bindings

set -e

echo "Building Amalgam WASM bindings..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
cd crates/amalgam-wasm

echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg-bundler

echo "Building for web target..."
wasm-pack build --target web --out-dir pkg-web

echo "Building for nodejs target..."
wasm-pack build --target nodejs --out-dir pkg-node

echo "WASM build complete!"
echo ""
echo "Packages created in:"
echo "  - crates/amalgam-wasm/pkg-bundler (for webpack/rollup)"
echo "  - crates/amalgam-wasm/pkg-web (for direct browser use)"
echo "  - crates/amalgam-wasm/pkg-node (for Node.js)"