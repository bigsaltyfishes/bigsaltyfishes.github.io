#!/usr/bin/env bash
# This script generates an index file for the assets directory.
set -e

ASSETS_DIR=$1
OUTPUT_DIR=$2

# Check if the assets directory is provided
if [ -z "$ASSETS_DIR" ] || [ -z "$OUTPUT_DIR" ]; then
    echo "Usage: $0 <assets_directory> <output_directory>"
    exit 1
fi

# Check if the assets directory exists
if [ ! -d "$ASSETS_DIR" ]; then
    echo "Assets directory '$ASSETS_DIR' does not exist."
    exit 1
fi

# Check if the required tools are installed
if ! command -v jq &> /dev/null; then
    echo "jq is not installed. Please install it to generate the index."
    exit 1
fi

# Read site.json to get site assets directory and articles directory
SITE_JSON="$ASSETS_DIR/site.json"
if [ ! -f "$SITE_JSON" ]; then
    echo "site.json not found in '$ASSETS_DIR'."
    exit 1
fi

SITE_ASSETS_DIR=$(jq -r '.assets.directory' "$SITE_JSON")
ARTICLES_DIR=$(jq -r '.assets.articles' "$SITE_JSON")

if [ -z "$SITE_ASSETS_DIR" ] || [ -z "$ARTICLES_DIR" ]; then
    echo "Invalid site.json format. 'assets.directory' or 'assets.articles' is missing."
    exit 1
fi

SITE_ASSETS_PATH="$ASSETS_DIR/$SITE_ASSETS_DIR"
ARTICLES_PATH="$SITE_ASSETS_PATH/$ARTICLES_DIR"
INDEX_FILE="$OUTPUT_DIR/index.json"
SPECIAL_INDEX_FILE="$OUTPUT_DIR/special.json"

# Check if the site assets and articles directories exist
if [ ! -d "$SITE_ASSETS_PATH" ]; then
    echo "Assets directory '$SITE_ASSETS_PATH' does not exist."
    exit 1
fi

if [ ! -d "$ARTICLES_PATH" ]; then
    echo "Articles directory '$ARTICLES_PATH' does not exist."
    exit 1
fi

# Scan the articles directory and generate the index
## First find special articles
START=true
echo -n "{" > "$SPECIAL_INDEX_FILE"
find "$ARTICLES_PATH" -type f -name ".special" | while read -r special_file; do
    # Directory structure:
    # $ARTICLES_PATH/<article_name>/.special
    # $ARTICLES_PATH/<article_name>/meta.json
    article_dir=$(dirname "$special_file")
    article_id=$(basename "$article_dir")
    meta_file="$article_dir/meta.json"
    if [ -f "$meta_file" ]; then
        test -f "$article_dir/index.md" || {
            echo "Warning: index.md not found for special article '$article_id'. Skipping."
            continue
        }
        if [ "$START" = true ]; then
            START=false
        else
            echo -n "," >> "$SPECIAL_INDEX_FILE"
        fi
        # Extract title and description from meta.json
        title=$(jq -r '.title' "$meta_file")
        description=$(jq -r '.description' "$meta_file")
        echo -n "\"$article_id\":{\"title\":\"$title\",\"description\":\"$description\"}" >> "$SPECIAL_INDEX_FILE"
        echo "Special article '$article_id' indexed."
    else
        echo "Warning: meta.json not found for special article '$article_id'. Skipping."
    fi
done
echo "}" >> "$SPECIAL_INDEX_FILE"

# Scan normal articles
START=true
echo -n "{" > "$INDEX_FILE"
find "$ARTICLES_PATH" -type f -name "index.md" | while read -r index_file; do
    # Directory structure:
    # $ARTICLES_PATH/<article_name>/index.md
    article_dir=$(dirname "$index_file")
    article_id=$(basename "$article_dir")
    meta_file="$article_dir/meta.json"
    # Skip if the article is marked as special
    test -f "$article_dir/.special" && {
        continue
    }
    if [ -f "$meta_file" ]; then
        # Extract title and description from meta.json
        if [ "$START" = true ]; then
            START=false
        else
            echo -n "," >> "$INDEX_FILE"
        fi
        echo -n "\"$article_id\":$(jq -c . "$meta_file")" >> "$INDEX_FILE"
        echo "Article '$article_id' indexed."
    else
        echo "Warning: meta.json not found for article '$article_id'. Skipping."
    fi
done
echo "}" >> "$INDEX_FILE"
echo "Index files generated successfully."