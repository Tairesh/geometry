export RUST_BACKTRACE=1
export CARGO_TERM_COLOR=always

default: build
before-commit: fmt check
check: fmt-check test clippy

build:
	cargo build --release

fmt:
	cargo fmt --

fmt-check:
	cargo fmt -- --check

test:
	cargo test

clippy:
	cargo clippy -- -D warnings -D clippy::pedantic --verbose --no-deps

clean:
	cargo clean
