#!/usr/bin/env bash
set -e

APP_NAME="ruCIDR"
DIST_DIR="dist"

mkdir -p "$DIST_DIR"

OS="$(uname -s)"
ARCH="$(uname -m)"

echo "Building on $OS ($ARCH)"

if [[ "$OS" == "Linux" ]]; then
    TARGET="x86_64-unknown-linux-gnu"

elif [[ "$OS" == "Darwin" ]]; then
    if [[ "$ARCH" != "arm64" ]]; then
        echo "❌ macOS supported only on ARM (Apple Silicon)"
        exit 1
    fi
    TARGET="aarch64-apple-darwin"

else
    echo "❌ Unsupported OS"
    exit 1
fi

echo "Target: $TARGET"

cargo build --release --target "$TARGET"

cp "target/$TARGET/release/$APP_NAME" "$DIST_DIR/$APP_NAME-$TARGET"

echo "✅ Build finished:"
ls -lh "$DIST_DIR"
