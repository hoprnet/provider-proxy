.POSIX:

# utility variables
space := $(subst ,, )
mydir := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))

# Set local cargo directory (for binaries)
# note: $(mydir) ends with '/'
CARGO_DIR := $(mydir).cargo

# add local Cargo install path (only once)
PATH := $(subst :${CARGO_DIR}/bin,,$(PATH)):${CARGO_DIR}/bin
# add users home Cargo install path (only once)
PATH := $(subst :${HOME}/.cargo/bin,,$(PATH)):${HOME}/.cargo/bin
# use custom PATH in all shell processes
# escape spaces
SHELL := env PATH=$(subst $(space),\$(space),$(PATH)) $(shell which bash)

all: help

.PHONY: build
build: ## build worker
	cargo install -q worker-build && worker-build --release

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
