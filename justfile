check:
	cargo fmt --all --check --quiet
	cargo clippy --all-targets --quiet
	cargo check --all-targets --quiet
	cargo build --all-targets --quiet
	cargo build --all-targets --no-default-features --quiet
	cargo build --all-targets --no-default-features --features derive --quiet
	cargo build --all-targets --no-default-features --features hints --quiet
	cargo build --all-targets --no-default-features --features stats --quiet
	cargo build --all-targets --all-features --quiet
	cargo test --quiet -- --format=terse

doc:
	RUSTFLAGS="--cfg docsrs" RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open
