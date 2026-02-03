.PHONY: all build test lint fmt clean check help example release

CARGO := cargo +nightly

all: fmt lint test

build:
	$(CARGO) build

test:
	$(CARGO) test

lint:
	$(CARGO) clippy -- -D warnings
	
fmt:
	$(CARGO) fmt

clean:
	$(CARGO) clean

check:
	$(CARGO) check

example:
	$(CARGO) run --example gitlab_demo

release:
	$(CARGO) publish --dry-run

help:
	@echo "Available targets:"
	@echo "  all       - Run fmt, lint, and test (default)"
	@echo "  build     - Build the project"
	@echo "  test      - Run tests"
	@echo "  lint      - Run clippy with strict settings"
	@echo "  fmt       - Format code using rustfmt"
	@echo "  check     - Quick syntax check"
	@echo "  example   - Run the gitlab_demo example"
	@echo "  release   - Dry-run publish to crates.io"
	@echo "  clean     - Remove build artifacts"
