#!/usr/bin/env bash

set -e

echo "Running Rust test coverage..."

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "Project root: $PROJECT_ROOT"

# Ensure cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "cargo-tarpaulin not found. Installing..."
    cargo install cargo-tarpaulin
fi

# Run coverage
cargo tarpaulin \
    --verbose \
    --all-features \
    --workspace \
    --timeout 120 \
    --out Html \
    --out Xml \
    --output-dir coverage

echo "Coverage complete!"

# Open report
if command -v xdg-open &> /dev/null; then
    xdg-open coverage/tarpaulin-report.html >/dev/null 2>&1 &
fi

echo "Report available at: coverage/tarpaulin-report.html"