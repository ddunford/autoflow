#!/bin/bash
set -e

# AutoFlow Installation Script
# This script installs AutoFlow and all required components

VERSION="0.1.3"
INSTALL_DIR="$HOME/.autoflow"
BIN_DIR="$INSTALL_DIR/bin"
AGENTS_DIR="$HOME/.claude/agents"
SKILLS_DIR="$HOME/.claude/skills"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  AutoFlow Installer v$VERSION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi
echo "  ✓ Rust/Cargo installed"

if ! command -v claude &> /dev/null; then
    echo "❌ Claude CLI not found. Please install Claude CLI first."
    echo "   Visit: https://claude.com/claude-code"
    exit 1
fi
echo "  ✓ Claude CLI installed"

if command -v docker &> /dev/null; then
    echo "  ✓ Docker installed"
else
    echo "  ⚠ Docker not found (optional, needed for environment management)"
fi

echo ""

# Build AutoFlow
echo "🔨 Building AutoFlow..."
cargo build --release
echo "  ✓ Build complete"
echo ""

# Create directories
echo "📁 Creating directories..."
mkdir -p "$BIN_DIR"
mkdir -p "$AGENTS_DIR"
mkdir -p "$SKILLS_DIR"
mkdir -p "$INSTALL_DIR/schemas"
mkdir -p "$INSTALL_DIR/templates"
mkdir -p "$INSTALL_DIR/reference"
echo "  ✓ Directories created"
echo ""

# Copy binary
echo "📦 Installing binary..."
cp target/release/autoflow "$BIN_DIR/autoflow"
chmod +x "$BIN_DIR/autoflow"
echo "  ✓ Binary installed to $BIN_DIR/autoflow"
echo ""

# Check for existing .claude directory
EXISTING_CLAUDE=false
EXISTING_AGENTS=0
EXISTING_SKILLS=0
EXISTING_MCP_SERVERS=0

if [ -d "$AGENTS_DIR" ]; then
    EXISTING_CLAUDE=true
    EXISTING_AGENTS=$(find "$AGENTS_DIR" -name "*.md" -not -name "*.agent.md" 2>/dev/null | wc -l)
fi

if [ -d "$SKILLS_DIR" ]; then
    EXISTING_SKILLS=$(find "$SKILLS_DIR" -name "*.md" 2>/dev/null | wc -l)
fi

if [ -f "$HOME/.claude/claude_desktop_config.json" ]; then
    # Count existing MCP servers
    EXISTING_MCP_SERVERS=$(grep -o '"[^"]*":' "$HOME/.claude/claude_desktop_config.json" | grep -v "mcpServers\|command\|args\|env" | wc -l)
fi

if [ "$EXISTING_CLAUDE" = true ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Found Existing Claude Code Setup"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  Existing agents: $EXISTING_AGENTS"
    echo "  Existing skills: $EXISTING_SKILLS"
    echo "  Existing MCP servers: $EXISTING_MCP_SERVERS"
    echo ""
    echo "  ✅ All existing configuration will be preserved"
    echo "  ✅ AutoFlow agents installed with .agent.md suffix"
    echo "  ✅ No conflicts will occur"
    echo ""
fi

# Copy agents
echo "🤖 Installing agents..."
if [ -d "agents" ]; then
    # Install AutoFlow agents with .agent.md suffix
    INSTALLED_AGENTS=0
    for agent in agents/*.md; do
        if [ -f "$agent" ]; then
            basename=$(basename "$agent" .md)
            cp "$agent" "$AGENTS_DIR/${basename}.agent.md"
            INSTALLED_AGENTS=$((INSTALLED_AGENTS + 1))
        fi
    done

    if [ $INSTALLED_AGENTS -gt 0 ]; then
        echo "  ✓ $INSTALLED_AGENTS AutoFlow agents installed"
        if [ "$EXISTING_CLAUDE" = true ]; then
            TOTAL_AGENTS=$((EXISTING_AGENTS + INSTALLED_AGENTS))
            echo "  ℹ Total agents now: $TOTAL_AGENTS ($EXISTING_AGENTS existing + $INSTALLED_AGENTS AutoFlow)"
        fi
    else
        echo "  ℹ No agent files found"
    fi
else
    echo "  ℹ No agents directory found"
fi
echo ""

# Copy skills
echo "🛠️  Installing skills..."
if [ -d "skills" ]; then
    # Copy skills with name checking to avoid conflicts
    INSTALLED_SKILLS=0
    SKIPPED_SKILLS=0

    # Copy both directory-based skills (skills/*/SKILL.md) and flat skills (skills/*.md)
    for skill_dir in skills/*/; do
        if [ -d "$skill_dir" ] && [ -f "$skill_dir/SKILL.md" ]; then
            skill_name=$(basename "$skill_dir")

            # Check if skill already exists
            if [ -d "$SKILLS_DIR/$skill_name" ]; then
                echo "  ⚠ Skipping $skill_name (already exists)"
                SKIPPED_SKILLS=$((SKIPPED_SKILLS + 1))
            else
                cp -r "$skill_dir" "$SKILLS_DIR/"
                INSTALLED_SKILLS=$((INSTALLED_SKILLS + 1))
            fi
        fi
    done

    # Also handle flat .md files in skills/ for backwards compatibility
    for skill in skills/*.md; do
        if [ -f "$skill" ]; then
            skill_name=$(basename "$skill")

            # Check if skill already exists
            if [ -f "$SKILLS_DIR/$skill_name" ]; then
                echo "  ⚠ Skipping $skill_name (already exists)"
                SKIPPED_SKILLS=$((SKIPPED_SKILLS + 1))
            else
                cp "$skill" "$SKILLS_DIR/"
                INSTALLED_SKILLS=$((INSTALLED_SKILLS + 1))
            fi
        fi
    done

    if [ $INSTALLED_SKILLS -gt 0 ]; then
        echo "  ✓ $INSTALLED_SKILLS skills installed"
    fi
    if [ $SKIPPED_SKILLS -gt 0 ]; then
        echo "  → $SKIPPED_SKILLS skills skipped (already exist)"
    fi

    if [ "$EXISTING_CLAUDE" = true ]; then
        TOTAL_SKILLS=$((EXISTING_SKILLS + INSTALLED_SKILLS))
        echo "  ℹ Total skills now: $TOTAL_SKILLS"
    fi
else
    echo "  ℹ No skills directory found"
fi
echo ""

# Copy schemas
echo "📋 Installing schemas..."
if [ -d "schemas" ]; then
    cp -r schemas/* "$INSTALL_DIR/schemas/" 2>/dev/null || true
    echo "  ✓ Schemas installed"
else
    echo "  ℹ No schemas directory found"
fi
echo ""

# Copy templates
echo "📄 Installing templates..."
if [ -d "templates" ]; then
    cp -r templates/* "$INSTALL_DIR/templates/" 2>/dev/null || true
    echo "  ✓ Templates installed"
else
    echo "  ℹ No templates directory found"
fi
echo ""

# Copy reference materials
echo "📚 Installing reference materials..."
if [ -d "reference" ]; then
    cp -r reference/* "$INSTALL_DIR/reference/" 2>/dev/null || true
    echo "  ✓ Reference materials installed"
else
    echo "  ℹ No reference directory found"
fi
echo ""

# Create config
echo "⚙️  Creating configuration..."
cat > "$INSTALL_DIR/config.toml" << 'EOF'
# AutoFlow Configuration
version = "0.1.0"

[paths]
agents_dir = "~/.claude/agents"
skills_dir = "~/.claude/skills"
schemas_dir = "~/.autoflow/schemas"
templates_dir = "~/.autoflow/templates"

[orchestrator]
max_iterations = 50
default_parallel = false

[quality]
auto_fix = true
schema_validation = true

[worktree]
base_port = 3000
port_increment = 10
bugfix_sprint_id = 900

[agent]
default_model = "claude-sonnet-4-5-20250929"
max_turns = 10
timeout_seconds = 300
EOF
echo "  ✓ Config created at $INSTALL_DIR/config.toml"
echo ""

# Add to PATH
echo "🔗 Setting up PATH..."

SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "autoflow/bin" "$SHELL_RC" 2>/dev/null; then
        echo "" >> "$SHELL_RC"
        echo "# AutoFlow" >> "$SHELL_RC"
        echo "export PATH=\"\$HOME/.autoflow/bin:\$PATH\"" >> "$SHELL_RC"
        echo "  ✓ Added to PATH in $SHELL_RC"
        echo "  ⚠ Run: source $SHELL_RC  (or restart your shell)"
    else
        echo "  ✓ Already in PATH"
    fi
else
    echo "  ⚠ Could not detect shell. Manually add to PATH:"
    echo "    export PATH=\"\$HOME/.autoflow/bin:\$PATH\""
fi
echo ""

# Create symlink (optional, for system-wide access)
if [ -w "/usr/local/bin" ]; then
    echo "🔗 Creating system symlink..."
    ln -sf "$BIN_DIR/autoflow" /usr/local/bin/autoflow 2>/dev/null && \
        echo "  ✓ Symlink created at /usr/local/bin/autoflow" || \
        echo "  ℹ Could not create symlink (run with sudo if needed)"
    echo ""
fi

# Verify installation
echo "✅ Verifying installation..."
if "$BIN_DIR/autoflow" --version &> /dev/null; then
    echo "  ✓ AutoFlow is ready!"
else
    echo "  ⚠ Warning: Could not verify installation"
fi
echo ""

# Print summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Installation Complete! 🎉"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📍 Installation paths:"
echo "   Binary:    $BIN_DIR/autoflow"
echo "   Config:    $INSTALL_DIR/config.toml"
echo "   Agents:    $AGENTS_DIR"
echo "   Skills:    $SKILLS_DIR"
echo ""
echo "🚀 Quick Start:"
echo "   1. Reload shell:  source $SHELL_RC"
echo "   2. Create project: mkdir my-project && cd my-project"
echo "   3. Initialize:     autoflow init"
echo "   4. Start coding:   autoflow start"
echo ""
echo "📖 Documentation:"
echo "   README:     https://github.com/autoflow/autoflow"
echo "   Commands:   autoflow --help"
echo "   Agents:     autoflow agents"
echo "   Skills:     autoflow skills"
echo ""
echo "💡 Next Steps:"
echo "   • Run 'autoflow init' in a project directory"
echo "   • Check out the examples in the README"
echo "   • Join the community (coming soon)"
echo ""
