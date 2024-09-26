default: fmt build test clippy doc

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
	RUSTFLAGS="--cfg docsrs" RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features

doc-open:
	RUSTFLAGS="--cfg docsrs" RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open
