# Makefile for common tasks
.PHONY: build test fmt lint run-example install

build:
	cargo build --release

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features

run-example:
	cargo run -- hello examples/poem.txt

install:
	cargo install --path .
