.POSIX:

all: help

.PHONY: build
build: ## build worker
	cargo build

.PHONY: clean
clean: # Cleanup build directories (lib,build, ...etc.)
	cargo clean

.PHONY: lint
lint: ## run linter for Rust
	cargo fmt --check

.PHONY: fmt
fmt: ## run code formatter for Rust
	cargo fmt

.PHONY: test
test: ## run tests
	./test.sh

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
