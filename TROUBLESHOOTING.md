# AutoFlow Troubleshooting Guide

This guide helps you diagnose and fix common issues with AutoFlow.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Problems](#configuration-problems)
- [Command Failures](#command-failures)
- [Agent Execution Issues](#agent-execution-issues)
- [Git & Worktree Problems](#git--worktree-problems)
- [MCP Server Issues](#mcp-server-issues)
- [Performance Issues](#performance-issues)
- [Getting Help](#getting-help)

---

## Installation Issues

### "claude: command not found"

**Problem**: AutoFlow requires Claude CLI but it's not installed.

**Solution**:
```bash
# Install Claude CLI (not Claude Desktop or claude-code)
# Visit: https://claude.com/cli

# Verify installation
claude --version
```

### "Rust/Cargo not found"

**Problem**: Building from source requires Rust toolchain.

**Solution**:
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
cargo --version
```

### Permission denied during installation

**Problem**: Install script doesn't have execute permissions or writing to protected directories.

**Solution**:
```bash
# Make install script executable
chmod +x scripts/install.sh

# If installing to /usr/local/bin requires sudo
sudo ln -s ~/.autoflow/bin/autoflow /usr/local/bin/autoflow
```

### PATH not updated after installation

**Problem**: `autoflow` command not found after installation.

**Solution**:
```bash
# Reload shell configuration
source ~/.bashrc  # or ~/.zshrc

# Or restart your terminal

# Verify PATH
echo $PATH | grep autoflow

# Manual PATH addition if needed
export PATH="$HOME/.autoflow/bin:$PATH"
```

---

## Configuration Problems

### Project not initialized

**Error**: `Project not initialized. Run 'autoflow init' first.`

**Solution**:
```bash
# Initialize AutoFlow in current directory
autoflow init

# Verify initialization
ls -la .autoflow/
```

### SPRINTS.yml not found

**Error**: `Failed to read .autoflow/SPRINTS.yml`

**Solution**:
```bash
# If using existing project, create initial sprints
autoflow analyze
autoflow add "Initial feature description"

# Or manually create SPRINTS.yml
cp templates/SPRINTS.template.yml .autoflow/SPRINTS.yml
```

### MCP servers not recognized

**Problem**: Claude Code doesn't see installed MCP servers.

**Solution**:
```bash
# Check MCP configuration
autoflow mcp list

# Verify file exists
cat ~/.claude/settings.local.json  # User scope
cat .mcp.json                       # Project scope

# Reinstall if needed
autoflow mcp install

# Restart Claude Code CLI
pkill claude && claude --version
```

---

## Command Failures

### `autoflow create` fails

**Error**: Various errors during project creation.

**Diagnosis**:
```bash
# Run with verbose logging
autoflow --verbose create my-project --idea IDEA.md

# Check common issues:
# 1. Directory already exists
ls my-project  # Should NOT exist before creation

# 2. IDEA.md not found or malformed
cat IDEA.md  # Check syntax and content

# 3. Agent execution failed
autoflow agents --detailed  # Verify make-docs agent exists
```

**Solution**:
```bash
# Remove failed project and retry
rm -rf my-project
autoflow create my-project --idea IDEA.md

# If agent fails, check agent definition
ls ~/.claude/agents/make-docs.agent.md
```

### `autoflow start` hangs or fails

**Error**: Sprint execution stops or times out.

**Diagnosis**:
```bash
# Check sprint status
autoflow status --json

# View logs
tail -f ~/.autoflow/logs/orchestrator.log

# Check specific sprint
autoflow sprints show <id>
```

**Solutions**:

1. **Sprint blocked**:
   ```bash
   # Reset blocked sprint
   autoflow rollback --sprint <id>

   # Or fix manually and retry
   autoflow start --sprint <id>
   ```

2. **Agent timeout**:
   ```bash
   # Increase timeout in config
   vim ~/.autoflow/config.toml
   # Set: timeout_seconds = 600
   ```

3. **Missing dependencies**:
   ```bash
   # Check sprint dependencies
   autoflow sprints show <id> --integration

   # Complete prerequisite sprints first
   autoflow start --sprint <prerequisite_id>
   ```

### `autoflow fix` doesn't create worktree

**Error**: Bug investigation fails to create bugfix branch.

**Diagnosis**:
```bash
# Check if git repo
ls -la .git

# Check existing worktrees
git worktree list

# Check branch conflicts
git branch | grep bugfix
```

**Solution**:
```bash
# If not a git repo
git init
git add .
git commit -m "Initial commit"

# If worktree exists
autoflow worktree delete bugfix-<name> --force

# If branch exists
git branch -D bugfix-<name>
```

---

## Agent Execution Issues

### "Agent file not found"

**Error**: `Agent file not found: ~/.claude/agents/xxx.agent.md`

**Solution**:
```bash
# List available agents
autoflow agents

# Reinstall agents
cd ~/autoflow  # Or wherever you cloned autoflow
./scripts/install.sh

# Verify installation
ls ~/.claude/agents/*.agent.md
```

### Agent produces invalid output

**Problem**: Agent returns malformed YAML or markdown.

**Diagnosis**:
```bash
# Enable verbose mode to see agent output
autoflow --verbose start --sprint <id>

# Check agent definition
cat ~/.claude/agents/<agent-name>.agent.md
```

**Solution**:
```bash
# Update agent definitions
cd ~/autoflow
git pull
./scripts/install.sh

# Or manually fix output in task directory
vim .autoflow/tasks/task-<sprint>/<output-file>
```

### "claude CLI failed"

**Error**: `Failed to spawn claude CLI` or permission errors.

**Solution**:
```bash
# Test Claude CLI directly
echo "Hello" | claude --print

# Check Claude CLI installation
which claude
claude --version

# Reinstall if needed
# Visit: https://claude.com/cli

# Check permissions
ls -la $(which claude)
```

---

## Git & Worktree Problems

### "Worktree already exists"

**Error**: Cannot create worktree because it already exists.

**Solution**:
```bash
# List all worktrees
autoflow worktree list

# Remove stale worktree
autoflow worktree delete sprint-<id>

# Or use git directly
git worktree remove ../sprint-<id> --force
```

### Merge conflicts

**Error**: `Merge conflict in branch: xxx`

**Solution**:
```bash
# Switch to worktree directory
cd ../sprint-<id>

# Resolve conflicts manually
git status
# Edit conflicting files
git add .
git commit

# Or abort merge
git merge --abort

# Then retry from main repo
cd -
autoflow worktree merge sprint-<id>
```

### Port conflicts

**Problem**: Service already running on assigned port.

**Solution**:
```bash
# Check which service is using the port
lsof -i :3010  # Or whichever port

# Kill the service
kill -9 <PID>

# Or modify docker-compose.yml in worktree
cd ../sprint-<id>
vim docker-compose.yml  # Change port mapping
```

---

## MCP Server Issues

### Memory server not working

**Problem**: Knowledge graph not persisting or recalling information.

**Solution**:
```bash
# Verify memory server installed
autoflow mcp list | grep memory

# Check logs
tail -f ~/.claude/logs/mcp-memory.log

# Reinstall
autoflow mcp install memory

# Test directly with Claude CLI
echo "Remember: test data" | claude --print
echo "What did I ask you to remember?" | claude --print
```

### Playwright server fails

**Problem**: Browser automation doesn't work.

**Solution**:
```bash
# Install Playwright dependencies
npx playwright install

# Install system dependencies
npx playwright install-deps

# Verify
npx playwright --version

# Reinstall MCP server
autoflow mcp install playwright
```

### GitHub token invalid

**Problem**: GitHub MCP server fails with auth errors.

**Solution**:
```bash
# Generate new token
# Visit: https://github.com/settings/tokens
# Scopes: repo, read:org

# Update token
vim ~/.claude/settings.local.json
# Update "env": { "GITHUB_TOKEN": "ghp_xxx" }

# Or set environment variable
export GITHUB_TOKEN=ghp_xxx
```

---

## Performance Issues

### Slow agent execution

**Problem**: Agents take too long to complete.

**Solutions**:

1. **Use faster model for simple tasks**:
   ```bash
   # Edit agent definition
   vim ~/.claude/agents/xxx.agent.md
   # Change: model: claude-haiku-4
   ```

2. **Reduce max_turns**:
   ```bash
   vim ~/.autoflow/config.toml
   # [agent]
   # max_turns = 5  # Reduce from 10
   ```

3. **Use parallel execution**:
   ```bash
   autoflow start --parallel
   ```

### High memory usage

**Problem**: AutoFlow or agents consuming too much memory.

**Solutions**:

1. **Clear agent cache** (when implemented):
   ```bash
   rm -rf ~/.autoflow/cache/agents/
   ```

2. **Limit parallel sprints**:
   ```bash
   # Run sequentially instead
   autoflow start  # Without --parallel
   ```

3. **Check for resource leaks**:
   ```bash
   # Monitor memory
   top -p $(pgrep autoflow)

   # Check for zombie processes
   ps aux | grep claude
   ```

---

## Common Error Messages

### "Schema validation failed"

**Problem**: SPRINTS.yml doesn't match expected schema.

**Solution**:
```bash
# Validate against schema
autoflow validate

# Fix common issues:
# - Missing required fields (id, goal, status, tasks)
# - Invalid status values (must be SCREAMING_SNAKE_CASE)
# - Malformed YAML syntax

# Use schema as reference
cat ~/.autoflow/schemas/sprint.schema.json
```

### "Blocked sprint detected"

**Problem**: Sprint failed multiple times and is marked BLOCKED.

**Solution**:
```bash
# View sprint details
autoflow sprints show <id>

# Reset sprint to try again
autoflow rollback --sprint <id>

# Or manually fix the issue first:
cd ../sprint-<id>
# Fix the problem
git add .
git commit -m "Fix blocking issue"

# Then reset status
autoflow rollback --sprint <id>
autoflow start --sprint <id>
```

---

## Getting Help

### Enable Debug Logging

```bash
# Run any command with verbose flag
autoflow --verbose <command>

# Check logs
tail -f ~/.autoflow/logs/autoflow.log
```

### Diagnostic Command

```bash
# Check system status (when implemented)
autoflow doctor

# Should show:
# - Rust/Cargo version
# - Claude CLI status
# - Git status
# - MCP servers status
# - Agent availability
# - Config validation
```

### Report Issues

1. **Check existing issues**: https://github.com/autoflow/autoflow/issues

2. **Gather information**:
   ```bash
   autoflow --version
   claude --version
   git --version
   cargo --version

   # System info
   uname -a

   # Configuration
   cat ~/.autoflow/config.toml
   ```

3. **Create issue with**:
   - AutoFlow version
   - Command that failed
   - Full error message
   - Steps to reproduce
   - Operating system

---

## Quick Fixes Summary

| Problem | Quick Fix |
|---------|-----------|
| Command not found | `source ~/.bashrc` |
| Project not initialized | `autoflow init` |
| Agent missing | `./scripts/install.sh` (from autoflow repo) |
| Sprint blocked | `autoflow rollback --sprint <id>` |
| Worktree exists | `autoflow worktree delete <name> --force` |
| MCP server not found | `autoflow mcp install` |
| Invalid SPRINTS.yml | `autoflow validate --fix` |
| Merge conflict | Resolve manually in worktree, then `git add . && git commit` |

---

**Still stuck?** Join our community or open an issue on GitHub!
