#!/bin/bash
set -e

echo "STEP dicetest: cargo fmt"
cargo fmt

echo "STEP dicetest: cargo build"
cargo build

echo "STEP dicetest: cargo build --no-default-features"
cargo build --no-default-features

echo "STEP dicetest: cargo build --no-default-features --features hints"
cargo build --no-default-features --features hints

echo "STEP dicetest: cargo build --no-default-features --features stats"
cargo build --no-default-features --features stats

echo "STEP dicetest: cargo build --no-default-features --features rand_full"
cargo build --no-default-features --features rand_full

echo "STEP dicetest: cargo build --no-default-features --features quickcheck_full"
cargo build --no-default-features --features quickcheck_full

echo "STEP dicetest: cargo build --all-features"
cargo build --all-features

echo "STEP dicetest: cargo test -- --format=terse"
cargo test -- --format=terse

echo "STEP dicetest: cargo clippy"
cargo clippy

echo "STEP dicetest: cargo doc --no-deps"
cargo doc --no-deps

echo "STEP dicetest: cargo readme > README.md"
cargo readme > README.md


cd examples_readme

echo "STEP examples_readme: cargo fmt"
cargo fmt

echo "STEP examples_readme: cargo build"
cargo build

echo "STEP examples_readme: cargo clippy"
cargo clippy
