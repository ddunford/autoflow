# Setup Required - AutoFlow Agents Missing

## Current Status

⚠️ **Critical Issue**: The `agents/` and `skills/` directories are not included in this repository.

The codebase is **functional** but requires agent definitions to work end-to-end.

## What Works

✅ **Core Infrastructure**:
- All 7 Rust crates compile successfully
- CLI with 14 commands fully implemented
- Git worktree isolation
- Quality gates and validation
- MCP server management

✅ **Manual Workflows**:
- `autoflow init` - Creates project structure
- `autoflow status` - Shows sprint progress
- `autoflow worktree` - Manages isolated workspaces
- `autoflow env` - Manages Docker containers
- `autoflow validate` - Runs quality checks

## What Doesn't Work

❌ **Autonomous Workflows** (require agents):
- `autoflow create` - Needs `make-docs` and `make-sprints` agents
- `autoflow start` - Needs sprint execution agents
- `autoflow add` - Needs `make-sprints` agent
- `autoflow fix` - Needs `debug-blocker` agent

## Required Setup

### Option 1: Use Existing Claude Code Agents (Recommended)

If you have Claude Code with existing agents:

```bash
# AutoFlow will use agents from ~/.claude/agents/*.agent.md
# Make sure you have these agents installed:
ls ~/.claude/agents/

# Required agents:
# - make-docs.agent.md
# - make-sprints.agent.md
# - debug-blocker.agent.md
# - code-implementer.agent.md
# - test-writer.agent.md
# - reviewer.agent.md
```

### Option 2: Create Agent Definitions

Create minimal agent definitions in `agents/` directory:

```bash
mkdir -p agents

# Create make-docs agent
cat > agents/make-docs.md << 'EOF'
---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Grep, Glob
---

# Documentation Generator Agent

Generate comprehensive project documentation from IDEA.md or requirements.

Create:
1. BUILD_SPEC.md - Technical specification
2. ARCHITECTURE.md - System architecture
3. API_SPEC.md - API endpoints (if backend)
4. UI_SPEC.md - UI/UX specifications (if frontend)

Use provided context to infer tech stack, patterns, and best practices.
EOF

# Create make-sprints agent
cat > agents/make-sprints.md << 'EOF'
---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Grep, Glob
---

# Sprint Planning Agent

Generate complete sprint plan in SPRINTS.yml format.

Output valid YAML with:
- Infrastructure sprints
- Core feature sprints
- Testing sprints
- Integration sprints

Follow TDD principles: tests before implementation.
EOF

# Create debug-blocker agent
cat > agents/debug-blocker.md << 'EOF'
---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Edit, Grep, Bash
---

# Bug Investigation Agent

Investigate bugs and provide detailed analysis:
1. Identify root cause
2. Affected files
3. Proposed solution
4. Test strategy

Create fixes when possible.
EOF

# Then reinstall
./scripts/install.sh
```

### Option 3: Use AutoFlow Without Agents

Work with the manual workflow:

```bash
# 1. Initialize project
mkdir my-app && cd my-app
autoflow init

# 2. Manually create BUILD_SPEC.md
cat > BUILD_SPEC.md << 'EOF'
# My Application

## Tech Stack
- Frontend: React
- Backend: Node.js
- Database: PostgreSQL
EOF

# 3. Manually create SPRINTS.yml
vim .autoflow/SPRINTS.yml

# 4. Use worktrees for isolated development
autoflow worktree create feature/my-feature
cd ../feature-my-feature
# Make changes
cd -
autoflow worktree merge feature/my-feature
```

## Roadmap

### Phase 7.1 - Agent Distribution (High Priority)

**Goal**: Make agents easily installable

**Options**:
1. Include agent definitions in repository
2. Create agent marketplace/registry
3. Generate agents from templates
4. Use Claude Code's agent system directly

**Recommendation**: Include minimal agent definitions in repo for core workflows.

### Phase 7.2 - Simplify Workflow (High Priority)

**Goal**: Make IDEA.md → app workflow seamless

**Current**: Multiple steps, requires agents, can fail
**Target**: Single command that "just works"

```bash
# Desired workflow
autoflow create my-app --idea IDEA.md
cd my-app
autoflow start --parallel
# Done - app is ready
```

**Improvements Needed**:
1. Better error messages when agents missing
2. Fallback to templates when agents fail
3. Guided prompts for missing information
4. Progress indicators
5. Automatic recovery from failures

## Immediate Next Steps

**For Users**:
1. Check if you have agents in `~/.claude/agents/`
2. If yes, AutoFlow should work with your existing agents
3. If no, create minimal agent definitions (Option 2 above)
4. Report issues/feedback for better onboarding

**For Contributors**:
1. Add agent definitions to repository
2. Improve error messages in `create.rs`
3. Add fallback logic when agents fail
4. Create agent templates
5. Document agent creation process

## Testing Current State

```bash
# Test what works
autoflow --version
autoflow --help
autoflow init
autoflow status

# Test with agents (requires setup)
autoflow create test-app --idea IDEA.md
```

## Feedback

If you encounter issues or have suggestions for improving the setup process, please:
1. Open an issue on GitHub
2. Include your setup (OS, Claude CLI version, agent status)
3. Describe expected vs actual behavior

---

**Status**: v0.1.1 - Core infrastructure complete, agent distribution pending
