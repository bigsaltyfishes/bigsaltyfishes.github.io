#!/usr/bin/env bash
# This script compiles the assets for the project.
set -e

ASSETS_DIR=$1
OUTPUT_DIR=$2

if [ -z "$ASSETS_DIR" ] || [ -z "$OUTPUT_DIR" ]; then
    echo "Usage: $0 <assets_directory> <output_directory>"
    exit 1
fi

# Check if the required tools are installed
if ! command -v jq &> /dev/null; then
    echo "jq is not installed. Please install it to compile assets."
    exit 1
fi

if ! command -v zstd &> /dev/null; then
    echo "Zstd is not installed. Please install it to compile assets."
    exit 1
fi

test -d "$OUTPUT_DIR" || mkdir "$OUTPUT_DIR"
test -d "$OUTPUT_DIR/assets" && rm -rf "$OUTPUT_DIR/assets"
test -f "$OUTPUT_DIR/site.json" && rm -f "$OUTPUT_DIR/site.json"

# Create the assets directory if it doesn't exist
find "$ASSETS_DIR" -type d  -exec mkdir -p "$OUTPUT_DIR/assets/{}" \;

# Scan and minfy all json files
find "$ASSETS_DIR" -type f -name "*.json" | while read -r file; do
    jq -c . "$file" > "$OUTPUT_DIR/assets/$file"
done

# Compress all markdown files with zstd
find "$ASSETS_DIR" -type f -name "*.md" | while read -r file; do
    zstd -q "$file" -o "$OUTPUT_DIR/assets/${file%.md}.zst"
done

echo "Assets compiled successfully."