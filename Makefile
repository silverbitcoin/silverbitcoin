# SilverBitcoin Makefile
# Provides convenient commands for building, testing, and deploying

.PHONY: help build build-release build-cross install uninstall test bench clean docker docker-build docker-up docker-down fmt clippy docs

# Default target
help:
	@echo "SilverBitcoin Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build              - Build debug binaries"
	@echo "  build-release      - Build optimized release binaries for current platform"
	@echo "  build-cross        - Build release binaries for all platforms"
	@echo "  install            - Install binaries to system (requires sudo)"
	@echo "  uninstall          - Uninstall binaries from system (requires sudo)"
	@echo "  test               - Run all tests"
	@echo "  bench              - Run benchmarks"
	@echo "  clean              - Clean build artifacts"
	@echo "  docker             - Build Docker images"
	@echo "  docker-build       - Build Docker images"
	@echo "  docker-up          - Start Docker development network"
	@echo "  docker-down        - Stop Docker development network"
	@echo "  fmt                - Format code with rustfmt"
	@echo "  clippy             - Run clippy linter"
	@echo "  docs               - Generate documentation"
	@echo ""
	@echo "Environment variables:"
	@echo "  VERSION            - Version number (default: 0.1.0)"
	@echo "  PREFIX             - Installation prefix (default: /usr/local)"

# Build debug binaries
build:
	@echo "Building debug binaries..."
	cargo build --bins

# Build release binaries for current platform
build-release:
	@echo "Building release binaries..."
	./scripts/build-release.sh

# Build release binaries for all platforms
build-cross:
	@echo "Building cross-platform binaries..."
	./scripts/build-cross-platform.sh

# Install binaries to system
install: build-release
	@echo "Installing SilverBitcoin..."
	@cd dist/silverbitcoin-*-$$(uname -s | tr '[:upper:]' '[:lower:]')-* && sudo ./install.sh

# Uninstall binaries from system
uninstall:
	@echo "Uninstalling SilverBitcoin..."
	@cd dist/silverbitcoin-*-$$(uname -s | tr '[:upper:]' '[:lower:]')-* && sudo ./uninstall.sh

# Run all tests
test:
	@echo "Running tests..."
	cargo test --all --all-features

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	cargo bench --all

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf dist/
	rm -rf data/db/*
	rm -rf logs/*.log

# Build Docker images
docker: docker-build

docker-build:
	@echo "Building Docker images..."
	cd docker && docker-compose build

# Start Docker development network
docker-up:
	@echo "Starting Docker development network..."
	cd docker && docker-compose up -d
	@echo ""
	@echo "Services started:"
	@echo "  RPC API:      http://localhost:9041"
	@echo "  WebSocket:    ws://localhost:9042"
	@echo "  GraphQL:      http://localhost:8080/graphql"
	@echo "  REST API:     http://localhost:8081"
	@echo "  Prometheus:   http://localhost:9090"
	@echo "  Grafana:      http://localhost:3000 (admin/admin)"

# Stop Docker development network
docker-down:
	@echo "Stopping Docker development network..."
	cd docker && docker-compose down

# Stop Docker and remove volumes
docker-clean:
	@echo "Stopping Docker and removing volumes..."
	cd docker && docker-compose down -v

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy --all --all-features -- -D warnings

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --all --no-deps --open

# Check code (fmt + clippy + test)
check: fmt clippy test
	@echo "All checks passed!"

# Development workflow
dev: fmt clippy build
	@echo "Development build complete!"

# CI workflow
ci: fmt clippy test
	@echo "CI checks passed!"

# Release workflow
release: clean fmt clippy test build-release
	@echo "Release build complete!"
	@echo "Distribution packages in dist/"
