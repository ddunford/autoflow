#!/bin/bash
set -e

# Sync agents and skills from git repo to installed locations
# Run this after making changes to agents/skills to update your local installation

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

AUTOFLOW_AGENTS="$HOME/.autoflow/agents"
CLAUDE_AGENTS="$HOME/.claude/agents"
CLAUDE_SKILLS="$HOME/.claude/skills"
AUTOFLOW_TEMPLATES="$HOME/.autoflow/templates"

echo "🔄 Syncing agents and skills from $REPO_ROOT"
echo ""

# Sync agents
if [ -d "$REPO_ROOT/agents" ]; then
    echo "📦 Syncing agents..."
    mkdir -p "$AUTOFLOW_AGENTS"
    mkdir -p "$CLAUDE_AGENTS"

    AGENT_COUNT=0
    for agent in "$REPO_ROOT/agents"/*.md; do
        if [ -f "$agent" ]; then
            basename=$(basename "$agent" .md)
            # Copy to both locations
            cp "$agent" "$AUTOFLOW_AGENTS/${basename}.md"
            cp "$agent" "$CLAUDE_AGENTS/${basename}.agent.md"
            AGENT_COUNT=$((AGENT_COUNT + 1))
        fi
    done

    echo "  ✓ Synced $AGENT_COUNT agents"
else
    echo "  ⚠ No agents directory found"
fi
echo ""

# Sync skills
if [ -d "$REPO_ROOT/skills" ]; then
    echo "🛠️  Syncing skills..."
    mkdir -p "$CLAUDE_SKILLS"

    SKILL_COUNT=0
    # Directory-based skills
    for skill_dir in "$REPO_ROOT/skills"/*/; do
        if [ -d "$skill_dir" ] && [ -f "$skill_dir/SKILL.md" ]; then
            skill_name=$(basename "$skill_dir")
            rm -rf "$CLAUDE_SKILLS/$skill_name"
            cp -r "$skill_dir" "$CLAUDE_SKILLS/"
            SKILL_COUNT=$((SKILL_COUNT + 1))
        fi
    done

    echo "  ✓ Synced $SKILL_COUNT skills"
else
    echo "  ⚠ No skills directory found"
fi
echo ""

# Sync templates
if [ -d "$REPO_ROOT/templates" ]; then
    echo "📄 Syncing templates..."
    mkdir -p "$AUTOFLOW_TEMPLATES"

    cp "$REPO_ROOT/templates"/*.yml "$AUTOFLOW_TEMPLATES/" 2>/dev/null || true
    cp "$REPO_ROOT/templates"/*.md "$AUTOFLOW_TEMPLATES/" 2>/dev/null || true

    echo "  ✓ Synced templates"
else
    echo "  ⚠ No templates directory found"
fi
echo ""

echo "✅ Sync complete!"
echo ""
echo "📍 Synced to:"
echo "   Agents:    $AUTOFLOW_AGENTS"
echo "   Agents:    $CLAUDE_AGENTS (with .agent.md suffix)"
echo "   Skills:    $CLAUDE_SKILLS"
echo "   Templates: $AUTOFLOW_TEMPLATES"
echo ""
echo "💡 Restart AutoFlow to use updated agents/skills"
