#!/usr/bin/env bash
# ThreadSanitizer validation for monad-core parallel execution.
#
# Runs the entire workspace test suite under TSAN to detect data races
# in the Block-STM parallel executor, CachedStateProvider (DashMap),
# and EcrecoverCache (DashMap) implementations.
#
# Requires: Rust nightly toolchain (auto-installed if missing).
# Usage:    bash scripts/tsan.sh
# Output:   tsan-output.log in the project root. Exit code 1 if races found.
#
# Note: TSAN requires the x86_64-unknown-linux-gnu target and only works
# on Linux. macOS support is experimental and may produce false positives.
set -euo pipefail

if ! rustup run nightly rustc --version &>/dev/null; then
    echo "Installing Rust nightly toolchain..."
    rustup toolchain install nightly
fi

echo "Running tests with ThreadSanitizer..."
RUSTFLAGS="-Zsanitizer=thread" \
    cargo +nightly test --workspace --target x86_64-unknown-linux-gnu \
    2>&1 | tee tsan-output.log

if grep -q "WARNING: ThreadSanitizer" tsan-output.log; then
    echo "FAIL: ThreadSanitizer found data race(s)"
    exit 1
else
    echo "PASS: No data races detected"
fi
