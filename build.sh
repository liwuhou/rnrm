#!/bin/bash
set -e

# Build script for all platforms
# Usage: ./build.sh [target]

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

build_target() {
    local target=$1
    echo "Building for $target..."

    cargo build --release --target "$target"

    echo "Build completed for $target"
}

if [ $# -eq 0 ]; then
    # Build all targets
    for target in "${TARGETS[@]}"; do
        build_target "$target"
    done
else
    # Build specific target
    build_target "$1"
fi

echo "All builds completed!"
