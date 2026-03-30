#!/bin/bash
# Script to link CSS from main crate to examples
# Usage: ./scripts/link-css.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "🔗 Linking CSS from main crate to examples..."

# Get the main CSS file
MAIN_CSS="$ROOT_DIR/styles.css"

if [ ! -f "$MAIN_CSS" ]; then
    echo "❌ Main CSS file not found: $MAIN_CSS"
    exit 1
fi

# Process each example
for example_dir in "$ROOT_DIR"/examples/*/; do
    example_name=$(basename "$example_dir")
    example_css="$example_dir/styles.css"
    
    # Skip if it's not a directory or if we're targeting a specific example
    [ -d "$example_dir" ] || continue
    
    echo "📦 Processing example: $example_name"
    
    # Remove existing CSS file if it exists
    if [ -f "$example_css" ] || [ -L "$example_css" ]; then
        rm -f "$example_css"
    fi
    
    # Create symlink to main CSS
    ln -sf "$MAIN_CSS" "$example_css"
    
    echo "✅ Linked $example_name -> $MAIN_CSS"
done

echo "✨ CSS linking complete!"
echo "📝 Note: For production, replace symlinks with CDN references in index.html"
