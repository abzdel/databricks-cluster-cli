#!/usr/bin/env bash
## Format all code directories in the repostitory using cargo fmt.

# lint all rs files in cluster-rs
cd cluster-rs
cargo clippy --all-targets --all-features -- -D warnings
echo "Lint complete."