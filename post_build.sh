#!/bin/bash
mkdir -p libs

# search for the current environment which store the cargo target folder
TARGET_DIR="${CARGO_TARGET_DIR:-target/release}"

# fetch the .so or .dylib from the target folder and store it to libs
find "$TARGET_DIR" -maxdepth 1 -type f \( -name "*.so" -o -name "*.dylib" \) -exec mv {} libs/ \;