#!/bin/bash
mkdir -p libs

# search for the current environment which store the cargo target folder
echo "Search folder: $1"
echo "Folder content:"
ls -R "$1"

# fetch the .so or .dylib from the target folder and store it to libs
find "$1" -maxdepth 1 -type f \( -name "*.so" -o -name "*.dylib" \) -exec mv {} libs/ \;