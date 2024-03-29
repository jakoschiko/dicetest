default: fmt build test clippy doc readme

fmt:
	cargo fmt
	cargo fmt --package examples_readme

build:
	cargo build --all-targets
	cargo build --all-targets --no-default-features
	cargo build --all-targets --no-default-features --features hints
	cargo build --all-targets --no-default-features --features stats
	cargo build --all-targets --no-default-features --features rand
	cargo build --all-targets --all-features
	cargo build --all-targets --package examples_readme

test:
	cargo test -- --format=terse

clippy:
	cargo clippy
	cargo clippy --package examples_readme

doc:
	cargo doc --no-deps

readme:
	cargo readme > README.md
