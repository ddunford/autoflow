#!/bin/bash
set -e

# Update AutoFlow Binary
# Run this after rebuilding to update your installed binary

echo "🔄 Updating AutoFlow binary..."

# Check if we're in the autoflow repo
if [ ! -f "Cargo.toml" ] || ! grep -q "autoflow" Cargo.toml; then
    echo "❌ Error: Must run from autoflow repository root"
    exit 1
fi

# Check if binary exists
if [ ! -f "target/release/autoflow" ]; then
    echo "❌ Error: Binary not found. Run 'cargo build --release' first"
    exit 1
fi

INSTALL_DIR="$HOME/.autoflow/bin"

# Create directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Backup and remove old binary
if [ -f "$INSTALL_DIR/autoflow" ]; then
    cp "$INSTALL_DIR/autoflow" "$INSTALL_DIR/autoflow.backup" 2>/dev/null || true
    rm -f "$INSTALL_DIR/autoflow"
    echo "  ✓ Backed up old binary to $INSTALL_DIR/autoflow.backup"
fi

# Copy new binary
cp target/release/autoflow "$INSTALL_DIR/autoflow"
chmod +x "$INSTALL_DIR/autoflow"

echo "  ✓ Binary updated: $INSTALL_DIR/autoflow"
echo ""

# Verify
VERSION=$("$INSTALL_DIR/autoflow" --version 2>&1 || echo "unknown")
echo "📦 Installed version: $VERSION"

# Check for pivot command
if "$INSTALL_DIR/autoflow" help 2>&1 | grep -q "pivot"; then
    echo "  ✓ Pivot command available"
else
    echo "  ⚠ Warning: Pivot command not found"
fi

echo ""
echo "✅ Update complete! You may need to run 'hash -r' in your shell"
