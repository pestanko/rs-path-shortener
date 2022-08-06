#! /bin/bash

EXEC_NAME="shorten-path"
INSTALL_DIR="$HOME/.local/bin/${EXEC_NAME}"
BUILD_EXEC="target/release/${EXEC_NAME}"

cargo build --release

cp "$BUILD_EXEC" "$INSTALL_DIR"


