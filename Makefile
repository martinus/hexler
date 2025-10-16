.PHONY: help build release test clean fmt lint check install run

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build in debug mode
	cargo build

release: ## Build in release mode with optimizations
	cargo build --release

test: ## Run all tests
	cargo test --all

test-integration: ## Run integration tests only
	cargo test --test integration_test

test-unit: ## Run unit tests only
	cargo test --lib

clean: ## Clean build artifacts
	cargo clean

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt-check lint test ## Run all checks (format, lint, test)

install: release ## Install the binary to ~/.cargo/bin
	cargo install --path .

run: ## Run the debug version with example file
	dd if=/dev/urandom bs=2000 count=1 | cargo run

bench: ## Run benchmarks (if any)
	cargo bench

bench-100mb: release ## Benchmark with 100MB of random data
	@echo "Benchmarking hexler with 100MB of random data..."
	@dd if=/dev/urandom bs=1M count=100 2>/dev/null | time target/release/hexler --stdout --num-bytes-per-line 16 > /dev/null
	@echo "Done!"

doc: ## Generate documentation
	cargo doc --no-deps --open

update: ## Update dependencies
	cargo update

audit: ## Security audit of dependencies (requires: cargo install cargo-audit)
	@which cargo-audit > /dev/null || (echo "cargo-audit not found. Install with: cargo install cargo-audit" && exit 1)
	cargo audit

bloat: ## Analyze binary size (requires: cargo install cargo-bloat)
	@which cargo-bloat > /dev/null || (echo "cargo-bloat not found. Install with: cargo install cargo-bloat" && exit 1)
	cargo bloat --release

coverage: ## Generate code coverage report (requires: cargo install cargo-tarpaulin)
	@which cargo-tarpaulin > /dev/null || (echo "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin" && exit 1)
	cargo tarpaulin --out Html --output-dir coverage

watch: ## Watch for changes and rebuild (requires: cargo install cargo-watch)
	@which cargo-watch > /dev/null || (echo "cargo-watch not found. Install with: cargo install cargo-watch" && exit 1)
	cargo watch -x build

watch-test: ## Watch for changes and run tests (requires: cargo install cargo-watch)
	@which cargo-watch > /dev/null || (echo "cargo-watch not found. Install with: cargo install cargo-watch" && exit 1)
	cargo watch -x test
