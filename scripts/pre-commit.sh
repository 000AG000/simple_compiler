#!/bin/sh
set -e

echo "Running cargo fmt..."
cargo fmt --all -- --check

echo "Running cargo clippy..."
cargo clippy --all-targets -- -D warnings

echo "Running cargo check..."
cargo check

echo "All checks passed."