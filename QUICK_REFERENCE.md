# Quick Reference: hexler Project Structure

## 📁 Project Layout

```
hexler/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml           # Continuous Integration
│   │   ├── release.yml      # Automated releases
│   │   └── benchmark.yml    # Performance tests
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   ├── pull_request_template.md
│   └── dependabot.yml       # Dependency updates
├── .vscode/
│   ├── tasks.json           # VS Code tasks
│   └── extensions.json      # Recommended extensions
├── src/                     # Source code
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── data/                # Test data
├── .clippy.toml            # Linting config
├── .gitignore
├── Cargo.toml
├── CONTRIBUTING.md         # Contribution guide
├── LICENSE
├── Makefile               # Development commands
├── README.md
├── rustfmt.toml           # Code formatting
└── SECURITY.md            # Security policy
```

## 🚀 Quick Commands

### Development
```bash
make build          # Build debug version
make release        # Build optimized version
make test          # Run all tests
make check         # Run format, lint, and tests
make run           # Run with example file
```

### Code Quality
```bash
make fmt           # Format code
make fmt-check     # Check formatting
make lint          # Run clippy
make coverage      # Generate coverage report
```

### Benchmarking
```bash
make bench-100mb   # Benchmark with 100MB random data
make bench         # Run cargo bench (if benchmarks exist)
```

### Testing
```bash
make test-unit           # Unit tests only
make test-integration    # Integration tests only
cargo test -- --nocapture  # See test output
```

## 🔧 CI/CD Workflows

### CI Workflow (runs on push/PR)
- ✅ Tests on Ubuntu, Windows, macOS
- ✅ Tests with stable and beta Rust
- ✅ Code formatting check
- ✅ Clippy linting
- ✅ Code coverage
- ✅ Multi-platform builds

### Release Workflow (runs on git tag)
- 🚀 Creates GitHub release
- 📦 Builds binaries for:
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl - static)
  - Windows x86_64
  - macOS x86_64
  - macOS ARM64 (Apple Silicon)
- 📤 Uploads release artifacts
- 📢 Publishes to crates.io (optional)

## 📝 Creating a Release

1. Update version in `Cargo.toml`
3. Commit changes
4. Create and push a git tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
5. GitHub Actions will automatically:
   - Build binaries for all platforms
   - Create a GitHub release
   - Upload the binaries

## 🧪 Testing

### All Tests (63 total)
- 54 unit tests (in src/)
- 6 integration tests (in tests/)
- 3 doc tests

### Running Tests Locally
```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# Integration tests only
cargo test --test integration_test
```

## 📊 Code Quality Standards

### Before Submitting PR
1. ✅ All tests pass: `cargo test`
2. ✅ Code is formatted: `cargo fmt`
3. ✅ No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
4. ✅ Documentation updated if needed

Or simply run:
```bash
make check
```

## 🔐 GitHub Secrets Needed

For full automation, configure these secrets in GitHub:
- `CARGO_TOKEN` - For publishing to crates.io (optional)

## 🎯 Key Features Added

1. **Automated Testing** - Every push/PR runs comprehensive tests
2. **Multi-Platform Support** - Automatic builds for all major platforms
3. **Code Quality** - Enforced formatting and linting
4. **Documentation** - Clear contribution guidelines
5. **Integration Tests** - Test actual binary behavior
6. **Dependency Management** - Dependabot keeps deps updated
7. **Professional Structure** - Issue templates, PR templates
8. **Easy Development** - Makefile for common tasks
9. **VS Code Integration** - Recommended extensions and tasks

## 📚 Documentation

- `README.md` - Project overview and usage
- `CONTRIBUTING.md` - How to contribute
- `SECURITY.md` - Security policy
- `PROJECT_IMPROVEMENTS.md` - Detailed improvements list

## 💡 Tips

- Use `make help` to see all available Makefile targets
- VS Code will suggest installing recommended extensions
- Run `cargo test` before committing

## 🐛 Troubleshooting

**Tests fail with "failed to determine terminal width"**
- Integration tests now use `--num-bytes-per-line 16` to work without a terminal

**Formatting warnings about unstable features**
- These are informational only; using stable rustfmt features is intentional

**Clippy warnings in CI**
- Fix locally with `cargo clippy --all-targets --all-features -- -D warnings`
- Then run `cargo fmt` to format fixes
