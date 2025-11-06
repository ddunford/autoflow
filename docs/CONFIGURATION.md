# AutoFlow Configuration Guide

AutoFlow supports both **global** and **project-level** configurations, working seamlessly alongside your existing Claude Code setup.

## Configuration Levels

### Global Configuration
- Location: `~/.autoflow/config.toml`
- Applies to all AutoFlow projects
- Default settings, MCP servers, agents, skills

### Project Configuration
- Location: `.autoflow/CLAUDE.md` and `.autoflow/settings.json`
- Applies only to current project
- Project-specific agents, MCP servers, overrides

## Installation Strategy

### Non-Invasive Installation

AutoFlow **never** overwrites existing Claude Code configurations. Instead:

1. **Agents**: Installed to `~/.claude/agents/` with `.agent.md` suffix
   - Your existing agents: `~/.claude/agents/*.md`
   - AutoFlow agents: `~/.claude/agents/*.agent.md`
   - ✅ No conflicts

2. **Skills**: Installed to `~/.claude/skills/` with unique names
   - Prefixed with category: `autoflow-*`, `react-*`, `playwright-*`
   - ✅ No conflicts

3. **MCP Servers**: Merged into `~/.claude/claude_desktop_config.json`
   - Preserves existing servers
   - Only adds new ones
   - ✅ No conflicts

## Global vs Project MCP Servers

### Global MCP Servers
**Location**: `~/.claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "memory": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-memory"]
    },
    "playwright": {
      "command": "npx",
      "args": ["-y", "@executeautomation/playwright-mcp-server"]
    }
  }
}
```

**Use for**: Servers you want available in all projects
- memory (knowledge graph)
- filesystem
- git
- fetch

### Project-Level MCP Servers
**Location**: `.autoflow/settings.json` (project-specific)

```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://localhost/myapp"
      }
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    }
  }
}
```

**Use for**: Project-specific servers
- Database connections (postgres, mysql)
- GitHub with project-specific tokens
- Custom project APIs

## Installation Modes

### 1. Clean Install (No Existing .claude/)

```bash
./scripts/install.sh
```

**What happens**:
- Creates `~/.claude/` directory
- Installs agents to `~/.claude/agents/`
- Installs skills to `~/.claude/skills/`
- Creates `~/.claude/claude_desktop_config.json`

### 2. Existing .claude/ Directory

```bash
./scripts/install.sh
```

**What happens**:
- ✅ Preserves existing `~/.claude/` contents
- ✅ Adds AutoFlow agents alongside yours
- ✅ Merges MCP servers (no overwrites)
- ✅ Shows diff of what was added

**Example**:
```
Found existing .claude/ directory

  Existing agents: 5
  AutoFlow agents: 25
  → Installing 25 new agents with .agent.md suffix

  Existing MCP servers: 2 (fetch, git)
  AutoFlow MCP servers: 8
  → Adding 6 new servers (memory, playwright, github, postgres, filesystem, sqlite)
  → Keeping existing servers intact

  ✓ Installation complete
  ✓ No conflicts detected
```

## MCP Server Management

### Installing MCP Servers

**Global (all projects)**:
```bash
autoflow mcp install memory playwright github
# Installs to ~/.claude/claude_desktop_config.json
```

**Project-specific**:
```bash
cd my-project
autoflow mcp install postgres --project
# Installs to .autoflow/settings.json
```

### Listing MCP Servers

```bash
# Global servers
autoflow mcp list

# Project servers
autoflow mcp list --project

# All (global + project)
autoflow mcp list --all
```

### Configuration Merging

When AutoFlow runs, it merges configurations in this order:

1. Global: `~/.claude/claude_desktop_config.json`
2. Project: `.autoflow/settings.json`
3. Environment variables override both

**Example**:
```bash
# Global has: memory, playwright
# Project has: postgres with connection string

# AutoFlow sees all 3 servers when running in that project
```

## Agent Discovery

AutoFlow discovers agents from multiple locations:

1. **Global AutoFlow agents**: `~/.claude/agents/*.agent.md`
2. **Your custom agents**: `~/.claude/agents/*.md`
3. **Project agents**: `.autoflow/agents/*.agent.md`

**Priority**: Project > Custom > AutoFlow

```bash
# List all available agents
autoflow agents

# Output:
# Global Agents (25):
#   code-implementer (AutoFlow)
#   test-writer (AutoFlow)
#   my-custom-agent (Custom)
#
# Project Agents (2):
#   payment-processor (Project-specific)
```

## Migration from Existing Setup

### If You Already Use Claude Code

**Good news**: AutoFlow is fully compatible!

**Migration steps**:
1. Install AutoFlow: `./scripts/install.sh`
2. Existing setup preserved automatically
3. Start using: `autoflow init` in any project

**What changes**:
- New agents added with `.agent.md` suffix
- MCP servers merged (existing ones untouched)
- New `~/.autoflow/` directory for AutoFlow config

**What stays the same**:
- Your existing agents still work
- Your MCP servers still work
- Claude Code commands unchanged

### Conflict Resolution

**If agent names conflict**:
```bash
# Your agent: ~/.claude/agents/code-reviewer.md
# AutoFlow agent: ~/.claude/agents/code-reviewer.agent.md

# No conflict! Different extensions
# Use yours: specify in .autoflow/CLAUDE.md
# Use AutoFlow's: default
```

**If MCP server names conflict**:
```bash
# Install preserves existing server
autoflow mcp install memory

# Output:
# ⚠ Server 'memory' already configured
# → Skipping (use --force to overwrite)
# → Or use: autoflow mcp install memory --project
```

## Project-Specific Configuration

### .autoflow/CLAUDE.md

```markdown
# Project: My App

## Agents
Use these agents for this project:
- code-implementer: AutoFlow default
- payment-processor: .autoflow/agents/payment.agent.md (custom)

## MCP Servers
- postgres: Project database
- github: Project repository

## Override Global Settings
- max_iterations: 100 (default: 50)
- parallel: true (default: false)
```

### .autoflow/settings.json

```json
{
  "version": "0.1.0",
  "orchestrator": {
    "max_iterations": 100,
    "default_parallel": true
  },
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://localhost/myapp"
      }
    }
  },
  "agents": {
    "custom_path": ".autoflow/agents"
  }
}
```

## Environment Variables

AutoFlow respects environment variables for sensitive data:

```bash
# .env
GITHUB_TOKEN=ghp_xxx
POSTGRES_CONNECTION_STRING=postgresql://user:pass@host/db

# settings.json can reference them
{
  "mcpServers": {
    "github": {
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    }
  }
}
```

## Sharing Configurations

### Team Configuration

**Commit to repo**:
```bash
git add .autoflow/settings.json
git add .autoflow/CLAUDE.md
git add .autoflow/agents/  # Project-specific agents
git commit -m "Add AutoFlow configuration"
```

**Team members**:
```bash
git clone repo
cd repo
autoflow init  # Detects existing .autoflow/
autoflow start --parallel
# Uses team's shared configuration
```

### Secrets Management

**Don't commit**:
- `.autoflow/.env` (add to .gitignore)
- Database passwords
- API tokens

**Do commit**:
- `.autoflow/settings.json` (with ${VAR} placeholders)
- `.autoflow/CLAUDE.md`
- `.autoflow/agents/` (custom project agents)

## Examples

### Example 1: Personal Projects

```bash
# Global MCP servers for all projects
autoflow mcp install memory playwright fetch git

# Each project uses global servers
cd project1
autoflow init
autoflow start

cd ../project2
autoflow init
autoflow start
# Both use same global servers
```

### Example 2: Work Projects with Databases

```bash
# Global: General-purpose servers
autoflow mcp install memory playwright github

# Project A: PostgreSQL
cd project-a
autoflow init
autoflow mcp install postgres --project
# Edit .autoflow/settings.json with DB connection

# Project B: MySQL
cd ../project-b
autoflow init
autoflow mcp install mysql --project
# Edit .autoflow/settings.json with DB connection
```

### Example 3: Team Collaboration

```bash
# Lead developer sets up project
autoflow create team-app --idea IDEA.md
cd team-app

# Add team-specific agents
mkdir .autoflow/agents
cat > .autoflow/agents/payment.agent.md << 'EOF'
---
model: claude-sonnet-4-5-20250929
tools: [Read, Write, Edit, Bash]
description: Payment processing specialist for Stripe integration
---
# Payment Processor
Handles Stripe API integration with PCI compliance
EOF

# Configure project MCP servers
autoflow mcp install postgres github --project
# Edit .autoflow/settings.json with team DB

# Commit configuration
git add .autoflow/
git commit -m "Add AutoFlow team configuration"
git push

# Team members clone and use
git clone repo
cd repo
autoflow start --parallel
# Uses team configuration automatically
```

## Best Practices

1. **Global for common tools**: memory, playwright, fetch, git
2. **Project for specifics**: database connections, project APIs
3. **Use .env for secrets**: Never commit passwords
4. **Share project config**: Commit .autoflow/settings.json with placeholders
5. **Custom agents in project**: Keep project-specific agents in `.autoflow/agents/`
6. **Test before committing**: Run `autoflow validate` to check configuration

## Troubleshooting

### Agents not found

```bash
# Check agent discovery
autoflow agents

# Check paths
echo ~/.claude/agents/*.agent.md
echo .autoflow/agents/*.agent.md
```

### MCP server conflicts

```bash
# List all servers
autoflow mcp list --all

# Check configuration
cat ~/.claude/claude_desktop_config.json
cat .autoflow/settings.json
```

### Project configuration not working

```bash
# Verify files exist
ls -la .autoflow/

# Validate configuration
autoflow validate

# Check for JSON errors
cat .autoflow/settings.json | jq .
```

## Migration Checklist

Moving from existing Claude Code setup:

- [ ] Backup existing `~/.claude/` directory
- [ ] Run `./scripts/install.sh`
- [ ] Verify no conflicts: `diff ~/.claude.backup ~/.claude`
- [ ] Test existing agents: `claude` (should still work)
- [ ] Test AutoFlow: `autoflow init` in test project
- [ ] Install MCP servers: `autoflow mcp install`
- [ ] Verify setup: `autoflow mcp list`
- [ ] Start using: `autoflow create my-app --idea IDEA.md`

---

**AutoFlow plays nicely with your existing setup!** ✅

No overwrites, no conflicts, just additional capabilities.
