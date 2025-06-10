#!/usr/bin/env bash
# This script compiles the assets for the project.
set -e

# Check if the required tools are installed
if ! command -v jq &> /dev/null; then
    echo "jq is not installed. Please install it to compile assets."
    exit 1
fi

if ! command -v zstd &> /dev/null; then
    echo "Zstd is not installed. Please install it to compile assets."
    exit 1
fi

test -d target || mkdir target
test -d target/assets && rm -rf target/assets
test -f target/site.json && rm -f target/site.json

cd assets

# Create the assets directory if it doesn't exist
find . -type d  -exec mkdir -p ../target/assets/{} \;

# Scan and minfy all json files
find . -type f -name "*.json" | while read -r file; do
    jq -c . "$file" > "../target/assets/$file"
done

# Compress all markdown files with zstd
find . -type f -name "*.md" | while read -r file; do
    zstd -q "$file" -o "../target/assets/${file%.md}.zst"
done

echo "Assets compiled successfully."