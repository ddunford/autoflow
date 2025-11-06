#!/bin/bash
set -e

# AutoFlow Update Script
# Safely updates agents and skills without overwriting user customizations

AUTOFLOW_DIR="$HOME/.autoflow"
AGENTS_DIR="$HOME/.claude/agents"
SKILLS_DIR="$HOME/.claude/skills"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Parse arguments
DRY_RUN=false
UPDATE_AGENTS=true
UPDATE_SKILLS=true
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --agents)
            UPDATE_SKILLS=false
            shift
            ;;
        --skills)
            UPDATE_AGENTS=false
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: update.sh [--dry-run] [--agents] [--skills] [--force]"
            exit 1
            ;;
    esac
done

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  AutoFlow Update${NC}"
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}  (Dry Run - No Changes Will Be Made)${NC}"
fi
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get script directory (where agents/ and skills/ are)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

cd "$REPO_DIR"

# Function to backup file
backup_file() {
    local file=$1
    local backup="${file}.backup-$(date +%Y%m%d-%H%M%S)"

    if [ "$DRY_RUN" = false ]; then
        cp "$file" "$backup"
        echo -e "  ${BLUE}→${NC} Backed up to: $(basename "$backup")"
    else
        echo -e "  ${BLUE}→${NC} Would backup to: $(basename "$backup")"
    fi
}

# Function to check if file was customized by user
is_customized() {
    local file=$1

    # If file has .backup files, it was likely customized
    if ls "${file}.backup-"* 2>/dev/null | grep -q .; then
        return 0  # Customized
    fi

    # Check if file has been modified in last 7 days (recent user edit)
    if [ -f "$file" ]; then
        local mod_time=$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)
        local now=$(date +%s)
        local diff=$((now - mod_time))

        if [ $diff -lt 604800 ]; then  # 7 days
            return 0  # Recently modified
        fi
    fi

    return 1  # Not customized
}

# Update agents
if [ "$UPDATE_AGENTS" = true ]; then
    echo -e "${GREEN}🤖 Updating agents...${NC}"

    if [ ! -d "agents" ]; then
        echo -e "  ${YELLOW}⚠${NC} No agents directory found in repo"
    else
        UPDATED=0
        SKIPPED=0

        for agent in agents/*.md; do
            if [ -f "$agent" ]; then
                basename=$(basename "$agent" .md)
                target="$AGENTS_DIR/${basename}.agent.md"

                if [ -f "$target" ]; then
                    # Check if customized
                    if is_customized "$target" && [ "$FORCE" = false ]; then
                        echo -e "  ${YELLOW}⚠${NC} Skipping $basename (appears customized, use --force to override)"
                        SKIPPED=$((SKIPPED + 1))
                        continue
                    fi

                    # Check if different
                    if ! diff -q "$agent" "$target" > /dev/null 2>&1; then
                        echo -e "  ${CYAN}↻${NC} Updating $basename"
                        backup_file "$target"

                        if [ "$DRY_RUN" = false ]; then
                            cp "$agent" "$target"
                        fi
                        UPDATED=$((UPDATED + 1))
                    else
                        echo -e "  ${GREEN}✓${NC} $basename (already up to date)"
                    fi
                else
                    # New agent
                    echo -e "  ${GREEN}+${NC} Installing new agent: $basename"
                    if [ "$DRY_RUN" = false ]; then
                        cp "$agent" "$target"
                    fi
                    UPDATED=$((UPDATED + 1))
                fi
            fi
        done

        echo ""
        if [ $UPDATED -gt 0 ]; then
            echo -e "  ${GREEN}✓${NC} $UPDATED agents updated"
        fi
        if [ $SKIPPED -gt 0 ]; then
            echo -e "  ${YELLOW}→${NC} $SKIPPED agents skipped (customized)"
        fi
    fi
    echo ""
fi

# Update skills
if [ "$UPDATE_SKILLS" = true ]; then
    echo -e "${GREEN}🛠️  Updating skills...${NC}"

    if [ ! -d "skills" ]; then
        echo -e "  ${YELLOW}⚠${NC} No skills directory found in repo"
    else
        UPDATED=0
        SKIPPED=0

        # Update directory-based skills
        for skill_dir in skills/*/; do
            if [ -d "$skill_dir" ] && [ -f "$skill_dir/SKILL.md" ]; then
                skill_name=$(basename "$skill_dir")
                target="$SKILLS_DIR/$skill_name"

                if [ -d "$target" ]; then
                    # Check if customized
                    if is_customized "$target/SKILL.md" && [ "$FORCE" = false ]; then
                        echo -e "  ${YELLOW}⚠${NC} Skipping $skill_name (appears customized, use --force to override)"
                        SKIPPED=$((SKIPPED + 1))
                        continue
                    fi

                    # Check if different
                    if ! diff -q "$skill_dir/SKILL.md" "$target/SKILL.md" > /dev/null 2>&1; then
                        echo -e "  ${CYAN}↻${NC} Updating $skill_name"
                        backup_file "$target/SKILL.md"

                        if [ "$DRY_RUN" = false ]; then
                            cp -r "$skill_dir"/* "$target/"
                        fi
                        UPDATED=$((UPDATED + 1))
                    else
                        echo -e "  ${GREEN}✓${NC} $skill_name (already up to date)"
                    fi
                else
                    # New skill
                    echo -e "  ${GREEN}+${NC} Installing new skill: $skill_name"
                    if [ "$DRY_RUN" = false ]; then
                        cp -r "$skill_dir" "$target"
                    fi
                    UPDATED=$((UPDATED + 1))
                fi
            fi
        done

        # Update flat .md skills (backwards compatibility)
        for skill in skills/*.md; do
            if [ -f "$skill" ]; then
                skill_name=$(basename "$skill")
                target="$SKILLS_DIR/$skill_name"

                if [ -f "$target" ]; then
                    if is_customized "$target" && [ "$FORCE" = false ]; then
                        echo -e "  ${YELLOW}⚠${NC} Skipping $skill_name (appears customized)"
                        SKIPPED=$((SKIPPED + 1))
                        continue
                    fi

                    if ! diff -q "$skill" "$target" > /dev/null 2>&1; then
                        echo -e "  ${CYAN}↻${NC} Updating $skill_name"
                        backup_file "$target"

                        if [ "$DRY_RUN" = false ]; then
                            cp "$skill" "$target"
                        fi
                        UPDATED=$((UPDATED + 1))
                    fi
                else
                    echo -e "  ${GREEN}+${NC} Installing new skill: $skill_name"
                    if [ "$DRY_RUN" = false ]; then
                        cp "$skill" "$target"
                    fi
                    UPDATED=$((UPDATED + 1))
                fi
            fi
        done

        echo ""
        if [ $UPDATED -gt 0 ]; then
            echo -e "  ${GREEN}✓${NC} $UPDATED skills updated"
        fi
        if [ $SKIPPED -gt 0 ]; then
            echo -e "  ${YELLOW}→${NC} $SKIPPED skills skipped (customized)"
        fi
    fi
    echo ""
fi

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}  Dry run complete - no changes made${NC}"
else
    echo -e "${GREEN}  Update complete!${NC}"
fi
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ "$DRY_RUN" = false ]; then
    echo -e "${BLUE}💡 Tips:${NC}"
    echo -e "  • Backups stored in ~/.claude/agents/*.backup-* and ~/.claude/skills/*/*.backup-*"
    echo -e "  • Use ${CYAN}--dry-run${NC} to preview changes"
    echo -e "  • Use ${CYAN}--force${NC} to update customized files"
    echo -e "  • Use ${CYAN}--agents${NC} or ${CYAN}--skills${NC} to update selectively"
fi
