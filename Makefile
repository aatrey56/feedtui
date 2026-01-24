.PHONY: help install build run clean test fmt clippy dev init

help:
	@echo "feedtui Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  install    - Install feedtui to ~/.cargo/bin"
	@echo "  build      - Build the project in release mode"
	@echo "  run        - Run the project with cargo run"
	@echo "  dev        - Run the project in debug mode"
	@echo "  init       - Run the configuration wizard"
	@echo "  clean      - Clean build artifacts"
	@echo "  test       - Run tests"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  clippy     - Run clippy linter"

install:
	@echo "Installing feedtui..."
	@cargo install --path .
	@echo "✓ Installation complete!"
	@echo "Run 'feedtui init' to configure, then 'feedtui' to start."

build:
	@echo "Building feedtui..."
	@cargo build --release

run:
	@cargo run --release

dev:
	@cargo run

init:
	@cargo run --release -- init

clean:
	@cargo clean

test:
	@cargo test

fmt:
	@cargo fmt

clippy:
	@cargo clippy -- -D warnings
