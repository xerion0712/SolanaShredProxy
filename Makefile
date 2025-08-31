.PHONY: build build-release clean test run install-rust help

# Default target
all: build-release

# Build in debug mode
build:
	cargo build

# Build in release mode
build-release:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Run the application
run:
	cargo run

# Run with debug logging
run-debug:
	cargo run -- --debug

# Install Rust (if not already installed)
install-rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	. ~/.cargo/env

# Show help
help:
	@echo "Available targets:"
	@echo "  build         - Build in debug mode"
	@echo "  build-release - Build in release mode (default)"
	@echo "  clean         - Clean build artifacts"
	@echo "  test          - Run tests"
	@echo "  run           - Run the application"
	@echo "  run-debug     - Run with debug logging"
	@echo "  install-rust  - Install Rust (if not already installed)"
	@echo "  help          - Show this help message"
