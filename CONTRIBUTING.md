# Contributing to hexler

Thank you for considering contributing to hexler! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and constructive in all interactions. We want to maintain a welcoming environment for all contributors.

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates. When you create a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Your environment (OS, Rust version, terminal type)
- Any relevant terminal output or screenshots

### Suggesting Enhancements

Enhancement suggestions are welcome! Please:

- Use a clear and descriptive title
- Provide a detailed description of the proposed enhancement
- Explain why this enhancement would be useful
- Include examples of how it would work

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/hexler.git
   cd hexler
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

3. **Make Your Changes**
   - Write clear, idiomatic Rust code
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

4. **Run Tests and Checks**
   ```bash
   # Run all tests
   cargo test
   
   # Check formatting
   cargo fmt -- --check
   
   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Build in release mode
   cargo build --release
   ```

5. **Commit Your Changes**
   - Use clear, descriptive commit messages
   - Follow conventional commit format if possible:
     - `feat:` for new features
     - `fix:` for bug fixes
     - `docs:` for documentation changes
     - `test:` for test changes
     - `refactor:` for code refactoring
     - `chore:` for maintenance tasks

6. **Push and Create Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a pull request on GitHub.

## Development Setup

### Prerequisites

- Rust 1.70 or later (stable)
- Cargo (comes with Rust)

### Building

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Run the development version
cargo run -- [arguments]
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_test
```

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Keep lines under 100 characters when practical
- Use meaningful variable and function names
- Add comments for complex logic

### Documentation

- Document public APIs with doc comments (`///`)
- Include examples in doc comments when helpful
- Keep README.md up to date with new features

## Project Structure

```
hexler/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Binary entry point
│   ├── ascii_renderer.rs   # ASCII character rendering
│   ├── border_writer.rs    # Border and formatting
│   ├── byte_to_color.rs    # Colorization logic
│   ├── error.rs            # Error types
│   ├── hex_formatter.rs    # Hex formatting
│   └── line_writer.rs      # Line output
├── tests/
│   ├── integration_test.rs # Integration tests
│   └── data/              # Test data files
├── .github/
│   └── workflows/         # CI/CD workflows
└── Cargo.toml             # Project metadata
```

## Performance Considerations

hexler is designed to be fast when processing large files:

- Avoid allocations in hot loops
- Use buffered I/O
- Consider SIMD for byte processing if applicable
- Profile before optimizing

### Running Benchmarks

Test performance with random data:

```bash
# Benchmark with 100MB
make bench-100mb

# Manual benchmark with custom size
dd if=/dev/urandom bs=1M count=50 | time target/release/hexler --stdout --num-bytes-per-line 16 > /dev/null
```

## Questions?

If you have questions, feel free to:
- Open an issue for discussion
- Check existing issues and pull requests
- Review the code and tests for examples

Thank you for contributing! 🎉
