# Makefile for Life Simulator

# Default target - builds the desktop version
.PHONY: all
all: desktop

# Build the desktop version (GUI)
.PHONY: desktop
desktop:
	cargo build --features desktop

# Build the release desktop version (GUI)
.PHONY: desktop-release
desktop-release:
	cargo build --release --features desktop

# Build the CLI version
.PHONY: cli
cli:
	cargo build --features cli

# Build the release CLI version
.PHONY: cli-release
cli-release:
	cargo build --release --features cli

# Run the desktop version (GUI)
.PHONY: run-desktop
run-desktop:
	cargo run --features desktop

# Run the CLI version
.PHONY: run-cli
run-cli:
	cargo run --features cli

# Run tests
.PHONY: test
test:
	cargo test

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Format code
.PHONY: format
format:
	cargo fmt

# Check code
.PHONY: check
check:
	cargo check

# Run clippy for linting
.PHONY: lint
lint:
	cargo clippy