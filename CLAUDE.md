# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`token-counter` is a Rust workspace project for counting tokens. The project consists of:

- **token-counter-lib**: Core library with token counting functionality (in `lib/`)
- **token-counter-bin**: CLI binary application (in `bin/`)

Tooling:
- **mise**: Rust toolchain version management (configured in `.mise.toml`)
- **cargo**: Build system and package management
- **cargo-nextest**: Installed and available for faster test execution
- **thiserror**: Used in lib for structured error handling
- **anyhow**: Used in bin for application-level error handling

## Key Commands

### Build & Run

```bash
# Build the entire workspace
cargo build

# Build in release mode (optimized)
cargo build --release

# Build specific package
cargo build -p token-counter-lib
cargo build -p token-counter-bin

# Run the binary (token-counter)
cargo run

# Run release build
cargo run --release
```

### Testing

```bash
# Run all tests (using nextest - faster)
cargo nextest run

# Run all tests (standard cargo test)
cargo test

# Run specific test by name
cargo nextest run test_greet

# Run tests in specific package
cargo nextest run -p token-counter-lib

# Run doctests (nextest doesn't support doctests, use cargo test)
cargo test --doc

# Run benchmarks
cargo bench
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint with clippy
cargo clippy

# Fix clippy warnings automatically
cargo clippy --fix

# Check code without building
cargo check

# Check all targets
cargo check --all-targets
```

### Dependencies

```bash
# Add a dependency
cargo add <crate_name>

# Add to specific package
cargo add -p token-counter-lib <crate_name>

# Update dependencies
cargo update

# Show dependency tree
cargo tree
```

### Documentation

```bash
# Build and open docs
cargo doc --open

# Build docs for workspace (without dependencies)
cargo doc --workspace --no-deps --open
```

## Workspace Architecture

This is a **Rust workspace** with two crates:

### 1. Library (`lib/`)
- **Package name**: `token-counter-lib`
- **Purpose**: Core token counting logic using Hugging Face tokenizers
- **Error handling**: Uses `thiserror` for structured error types
- **Key exports**:
  - `load_tokenizer(path)` - Load a tokenizer from JSON file
  - `load_tokenizer_from_bytes(bytes)` - Load a tokenizer from embedded bytes
  - `count_tokens(text, tokenizer)` - Count tokens in a string
  - `count_stats(text, tokenizer)` - Count tokens, lines, and bytes
  - `count_tokens_in_file(path, tokenizer)` - Count tokens in a file
  - `count_tokens_from_reader(reader, tokenizer)` - Count tokens from stdin/reader
  - `TokenStats` struct - Holds token, line, and byte counts
  - `Error` enum - Custom error type
  - `Result<T>` type alias

### 2. Binary (`bin/`)
- **Package name**: `token-counter-bin`
- **Binary name**: `tc`
- **Purpose**: CLI for counting LLM tokens, similar to Unix `wc`
- **Default tokenizer**: Embedded GPT-2 tokenizer (~1.4MB)
- **Error handling**: Uses `anyhow` for application errors
- **CLI features**: Multiple files, stdin support, flexible output options, custom tokenizer support
- **Dependency**: Depends on `token-counter-lib`

### Workspace Configuration

The root `Cargo.toml` defines:
- Shared workspace dependencies (anyhow, thiserror, tokenizers, clap, serde, tokio)
- Release profile optimizations (LTO, single codegen unit, stripped binaries)
- Workspace metadata (version, edition, license, description, repository)

## Architecture Patterns

### Error Handling Pattern
- **Library (`lib/`)**: Custom error types with `thiserror` for structured errors that can be handled by consumers
- **Binary (`bin/`)**: `anyhow` for simpler application-level error handling with context

### Dependency Isolation
- Library has minimal dependencies (only `thiserror` and `tokenizers`)
- Binary can have more dependencies as needed (`clap`, `anyhow`)
- Shared dependencies defined in workspace `Cargo.toml`

### Embedded Tokenizer
- GPT-2 tokenizer embedded in binary using `include_bytes!()` macro
- Located at `bin/assets/gpt2-tokenizer.json`
- Allows `tc` to work without external configuration (like Unix `wc`)
- Custom tokenizers can still be used with `--tokenizer` flag

## Development Notes

- Binary depends on library: `bin/` imports from `lib/`
- Keep core logic in `lib/`, CLI-specific code in `bin/`
- Library uses `thiserror` for errors, binary uses `anyhow`
- Unit tests live alongside code in `lib/src/lib.rs` with `#[cfg(test)]`
- Integration tests go in `lib/tests/`
- Examples go in `lib/examples/`
- Benchmarks go in `lib/benches/`
- Run `cargo fmt` before committing
- Use `cargo nextest run` for faster test execution
- Documentation comments use `///` and support markdown
- Binary size is ~5.1MB (includes 1.4MB embedded tokenizer)
