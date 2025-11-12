#!/bin/bash
set -e

# AutoFlow Release Packaging Script
# Creates distributable packages for different platforms

VERSION="0.1.0"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  AutoFlow Package Builder v$VERSION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd "$PROJECT_ROOT"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"
cargo clean
echo "  ✓ Cleaned"
echo ""

# Build release binary
echo "🔨 Building release binary..."
cargo build --release
echo "  ✓ Build complete"
echo ""

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        PLATFORM="linux"
        ;;
    Darwin*)
        PLATFORM="macos"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        ;;
    *)
        PLATFORM="unknown"
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH="x64"
        ;;
    aarch64|arm64)
        ARCH="arm64"
        ;;
    *)
        ARCH="unknown"
        ;;
esac

PACKAGE_NAME="autoflow-v${VERSION}-${PLATFORM}-${ARCH}"
PACKAGE_DIR="$DIST_DIR/$PACKAGE_NAME"

echo "📦 Creating package: $PACKAGE_NAME"
mkdir -p "$PACKAGE_DIR"
echo ""

# Copy binary
echo "📋 Packaging files..."
mkdir -p "$PACKAGE_DIR/bin"
if [ "$PLATFORM" = "windows" ]; then
    cp target/release/autoflow.exe "$PACKAGE_DIR/bin/"
else
    cp target/release/autoflow "$PACKAGE_DIR/bin/"
fi
echo "  ✓ Binary"

# Copy agents (if they exist)
if [ -d "agents" ] && [ "$(ls -A agents)" ]; then
    mkdir -p "$PACKAGE_DIR/agents"
    cp agents/*.agent.md "$PACKAGE_DIR/agents/" 2>/dev/null || true
    echo "  ✓ Agents"
fi

# Copy skills (if they exist)
if [ -d "skills" ] && [ "$(ls -A skills)" ]; then
    mkdir -p "$PACKAGE_DIR/skills"
    cp skills/*.md "$PACKAGE_DIR/skills/" 2>/dev/null || true
    echo "  ✓ Skills"
fi

# Copy schemas
if [ -d "schemas" ]; then
    cp -r schemas "$PACKAGE_DIR/"
    echo "  ✓ Schemas"
fi

# Copy templates
if [ -d "templates" ]; then
    cp -r templates "$PACKAGE_DIR/"
    echo "  ✓ Templates"
fi

# Copy reference materials
if [ -d "reference" ]; then
    cp -r reference "$PACKAGE_DIR/"
    echo "  ✓ Reference"
fi

# Copy documentation
cp README.md "$PACKAGE_DIR/" 2>/dev/null || true
cp LICENSE "$PACKAGE_DIR/" 2>/dev/null || true
cp ARCHITECTURE.md "$PACKAGE_DIR/" 2>/dev/null || true
cp GETTING_STARTED.md "$PACKAGE_DIR/" 2>/dev/null || true
echo "  ✓ Documentation"

# Copy install scripts
mkdir -p "$PACKAGE_DIR/scripts"
cp scripts/install.sh "$PACKAGE_DIR/scripts/" 2>/dev/null || true
cp scripts/uninstall.sh "$PACKAGE_DIR/scripts/" 2>/dev/null || true
chmod +x "$PACKAGE_DIR/scripts/"*.sh 2>/dev/null || true
echo "  ✓ Scripts"

# Create installation instructions
cat > "$PACKAGE_DIR/INSTALL.txt" << 'EOF'
AutoFlow Installation Instructions
===================================

Quick Install:
--------------
  ./scripts/install.sh

Manual Install:
---------------
1. Copy the binary:
   mkdir -p ~/.autoflow/bin
   cp bin/autoflow ~/.autoflow/bin/

2. Add to PATH (add to ~/.bashrc or ~/.zshrc):
   export PATH="$HOME/.autoflow/bin:$PATH"

3. Copy agents and skills:
   mkdir -p ~/.claude/agents ~/.claude/skills
   cp -r agents/* ~/.claude/agents/ 2>/dev/null || true
   cp -r skills/* ~/.claude/skills/ 2>/dev/null || true

4. Copy config files:
   cp -r schemas templates reference ~/.autoflow/

5. Reload shell:
   source ~/.bashrc  # or ~/.zshrc

6. Verify:
   autoflow --version

Getting Started:
----------------
1. Create a new project:
   mkdir my-project && cd my-project

2. Initialize:
   autoflow init

3. Start development:
   autoflow start

For full documentation, see README.md
EOF

echo "  ✓ Installation instructions"
echo ""

# Create archive
echo "📦 Creating archive..."
cd "$DIST_DIR"

if command -v tar &> /dev/null; then
    tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"
    echo "  ✓ Created: ${PACKAGE_NAME}.tar.gz"

    # Calculate size and checksum
    SIZE=$(du -h "${PACKAGE_NAME}.tar.gz" | cut -f1)
    if command -v sha256sum &> /dev/null; then
        CHECKSUM=$(sha256sum "${PACKAGE_NAME}.tar.gz" | cut -d' ' -f1)
        echo "$CHECKSUM  ${PACKAGE_NAME}.tar.gz" > "${PACKAGE_NAME}.tar.gz.sha256"
        echo "  ✓ SHA256: ${CHECKSUM:0:16}..."
    fi
    echo "  ✓ Size: $SIZE"
else
    echo "  ⚠ tar not found, keeping directory only"
fi

cd "$PROJECT_ROOT"
echo ""

# Create release notes
cat > "$DIST_DIR/RELEASE_NOTES.md" << EOF
# AutoFlow v${VERSION} Release Notes

## Package Information
- **Version**: $VERSION
- **Platform**: $PLATFORM
- **Architecture**: $ARCH
- **Build Date**: $(date +"%Y-%m-%d %H:%M:%S")

## What's Included
- AutoFlow CLI binary
- Agent definitions (25+)
- Skill definitions (13+)
- JSON schemas
- Project templates
- Reference materials
- Installation scripts
- Documentation

## Installation
Run: \`./scripts/install.sh\`

Or see INSTALL.txt for manual installation.

## Requirements
- Rust 1.70+ (for building from source)
- Claude CLI (required)
- Docker & Docker Compose (optional, for environment management)

## Quick Start
\`\`\`bash
# Install
./scripts/install.sh

# Create project
mkdir my-project && cd my-project
autoflow init

# Start development
autoflow start
\`\`\`

## Features
- ✅ Fully autonomous TDD pipeline
- ✅ Git worktree isolation
- ✅ Autonomous bug fixing
- ✅ Codebase analysis and integration
- ✅ Quality gates and validation
- ✅ Docker environment management
- ✅ 13 CLI commands
- ✅ 25+ specialized agents

## Documentation
- README.md - Overview and usage
- ARCHITECTURE.md - System design
- GETTING_STARTED.md - Step-by-step guide

## Support
- GitHub: https://github.com/autoflow/autoflow
- Issues: https://github.com/autoflow/autoflow/issues

---
Built with ❤️ by the AutoFlow team
EOF

echo "✅ Package created successfully!"
echo ""
echo "📍 Location: $DIST_DIR"
echo ""
echo "📦 Files created:"
ls -lh "$DIST_DIR" | grep -v "^d" | awk '{print "  ", $9, "(" $5 ")"}'
echo ""
echo "🚀 Ready for distribution!"
echo ""
