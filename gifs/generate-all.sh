#!/bin/bash
# Generate all GIF demos for Piemme
# Requires: vhs (https://github.com/charmbracelet/vhs)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
TAPES_DIR="$SCRIPT_DIR/tapes"
OUTPUT_DIR="$SCRIPT_DIR/output"
DEMO_DIR="$SCRIPT_DIR/demo-workspace"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Piemme GIF Generator${NC}"
echo "===================="
echo ""

# Check for vhs
if ! command -v vhs &> /dev/null; then
    echo -e "${RED}Error: vhs is not installed${NC}"
    echo "Install it with: go install github.com/charmbracelet/vhs@latest"
    echo "Or: brew install vhs"
    exit 1
fi

# Build piemme first (release mode for better performance in demos)
echo -e "${YELLOW}Building piemme in release mode...${NC}"
cd "$PROJECT_DIR"
cargo build --release
echo -e "${GREEN}Build complete!${NC}"
echo ""

# Backup existing .piemme folder if it exists
BACKUP_DIR=""
if [ -d "$PROJECT_DIR/.piemme" ]; then
    BACKUP_DIR=$(mktemp -d)
    echo -e "${YELLOW}Backing up existing .piemme folder to $BACKUP_DIR${NC}"
    cp -r "$PROJECT_DIR/.piemme" "$BACKUP_DIR/.piemme"
fi

# Cleanup function to restore backup
cleanup() {
    # Remove demo workspace
    rm -rf "$DEMO_DIR"
    rm -rf "$PROJECT_DIR/.piemme"
    
    # Restore backup if it exists
    if [ -n "$BACKUP_DIR" ] && [ -d "$BACKUP_DIR/.piemme" ]; then
        echo -e "${YELLOW}Restoring original .piemme folder${NC}"
        cp -r "$BACKUP_DIR/.piemme" "$PROJECT_DIR/.piemme"
        rm -rf "$BACKUP_DIR"
    fi
}

# Set trap to cleanup on exit (success or failure)
trap cleanup EXIT

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Count tapes
TAPE_COUNT=$(ls -1 "$TAPES_DIR"/*.tape 2>/dev/null | wc -l)
if [ "$TAPE_COUNT" -eq 0 ]; then
    echo -e "${RED}No tape files found in $TAPES_DIR${NC}"
    exit 1
fi

echo "Found $TAPE_COUNT tape files to process"
echo ""

# Process each tape
CURRENT=0
FAILED=0

# Change to gifs directory so relative Output paths in tapes resolve correctly
cd "$SCRIPT_DIR"

for tape in "$TAPES_DIR"/*.tape; do
    CURRENT=$((CURRENT + 1))
    TAPE_NAME=$(basename "$tape" .tape)
    
    echo -e "${YELLOW}[$CURRENT/$TAPE_COUNT] Recording: $TAPE_NAME${NC}"
    
    # Clean up any leftover .piemme from previous runs
    rm -rf "$PROJECT_DIR/.piemme"
    
    if vhs "$tape" 2>&1; then
        echo -e "${GREEN}  ✓ Success: output/$TAPE_NAME.gif${NC}"
    else
        echo -e "${RED}  ✗ Failed: $TAPE_NAME${NC}"
        FAILED=$((FAILED + 1))
    fi
    
    echo ""
done

# Summary
echo "===================="
echo -e "${GREEN}Generated: $((TAPE_COUNT - FAILED)) GIFs${NC}"
if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED${NC}"
fi
echo ""
echo "Output directory: $OUTPUT_DIR"
ls -la "$OUTPUT_DIR"/*.gif 2>/dev/null || echo "(no GIFs generated yet)"
