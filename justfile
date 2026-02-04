check:
	cargo fmt --check --quiet
	cargo clippy --quiet
	cargo check --quiet
	cargo build --quiet --all-targets
	cargo build --quiet --all-targets --no-default-features
	cargo build --quiet --all-targets --no-default-features --features derive
	cargo build --quiet --all-targets --no-default-features --features hints
	cargo build --quiet --all-targets --no-default-features --features stats
	cargo build --quiet --all-targets --all-features
	cargo test --quiet -- --format=terse

doc:
	RUSTFLAGS="--cfg docsrs" RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open
