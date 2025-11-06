#!/bin/bash

# AutoFlow Uninstall Script

INSTALL_DIR="$HOME/.autoflow"
BIN_DIR="$INSTALL_DIR/bin"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  AutoFlow Uninstaller"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Confirm uninstall
read -p "Are you sure you want to uninstall AutoFlow? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstall cancelled."
    exit 0
fi

echo ""
echo "🗑️  Removing AutoFlow..."

# Remove binary
if [ -f "$BIN_DIR/autoflow" ]; then
    rm -f "$BIN_DIR/autoflow"
    echo "  ✓ Binary removed"
fi

# Remove symlink
if [ -L "/usr/local/bin/autoflow" ]; then
    sudo rm -f /usr/local/bin/autoflow 2>/dev/null && \
        echo "  ✓ Symlink removed" || \
        echo "  ℹ Could not remove symlink (may need sudo)"
fi

# Remove installation directory
if [ -d "$INSTALL_DIR" ]; then
    read -p "Remove all AutoFlow data ($INSTALL_DIR)? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$INSTALL_DIR"
        echo "  ✓ Installation directory removed"
    else
        echo "  ℹ Keeping $INSTALL_DIR"
    fi
fi

# Note about agents and skills
echo ""
echo "📝 Note: Agents and skills in ~/.claude/ were preserved."
echo "   To remove them manually:"
echo "   rm -rf ~/.claude/agents/*.agent.md"
echo "   rm -rf ~/.claude/skills/*.md"
echo ""

# Remove from shell rc
SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
fi

if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
    if grep -q "autoflow/bin" "$SHELL_RC"; then
        read -p "Remove AutoFlow from PATH in $SHELL_RC? (y/N): " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            sed -i.bak '/# AutoFlow/,+1d' "$SHELL_RC"
            echo "  ✓ Removed from $SHELL_RC"
            echo "  ℹ Backup saved as ${SHELL_RC}.bak"
        fi
    fi
fi

echo ""
echo "✅ AutoFlow uninstalled"
echo ""
