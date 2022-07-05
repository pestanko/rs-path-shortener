#! /bin/bash

INSTALL_DIR="$HOME/.local/bin"
BUILD_EXEC="target/release/shorten-path"

cargo build --release

cp "$BUILD_EXEC" "$INSTALL_DIR"


