.PHONY: all build run test lint format clean help

all: run

build:
	@cargo build

release:
	@cargo build --release

run:
	@cargo run -- $(ARGS)

test:
	@cargo test

lint:
	@cargo clippy -- -D warnings
	@cargo fmt -- --check

format:
	@cargo fmt

clean:
	@cargo clean

help:
	@echo "Usage: make [target] [ARGS=\"...\"]"
	@echo ""
	@echo "Targets:"
	@echo "  build    Build the project (debug)"
	@echo "  release  Build the project (release)"
	@echo "  run      Run the project. Use ARGS=\"...\" to pass arguments."
	@echo "           Example: make run ARGS=\"run -f ~/.config/caged/caged.yaml\""
	@echo "  test     Run tests"
	@echo "  lint     Run clippy and fmt check"
	@echo "  format   Run cargo fmt"
	@echo "  clean    Remove build artifacts"
	@echo "  help     Show this help message"
