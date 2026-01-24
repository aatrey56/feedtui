#!/usr/bin/env bash
set -e

echo "=== feedtui Installation Script ==="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Building feedtui..."
cargo build --release

echo ""
echo "Installing feedtui to ~/.cargo/bin..."
cargo install --path .

echo ""
echo "✓ Installation complete!"
echo ""
echo "Make sure ~/.cargo/bin is in your PATH."
echo ""
echo "To add it to your PATH, add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
echo ""
echo "Next steps:"
echo "  1. Run 'feedtui init' to configure your dashboard"
echo "  2. Run 'feedtui' to start the dashboard"
echo ""
