# AutoFlow User Guide

Complete guide to using AutoFlow for autonomous software development.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Workflows](#workflows)
  - [Creating a New Project](#workflow-1-creating-a-new-project-from-idea)
  - [Adding Features to Existing Project](#workflow-2-adding-features-to-existing-project)
  - [Fixing Bugs](#workflow-3-fixing-bugs)
  - [Working with Worktrees](#workflow-4-working-with-git-worktrees)
- [Command Reference](#command-reference)
- [Advanced Usage](#advanced-usage)
- [Best Practices](#best-practices)

---

## Quick Start

```bash
# 1. Install AutoFlow
./scripts/install.sh

# 2. Create your first project
echo "# Task Manager
A real-time task management app with user auth" > IDEA.md

autoflow create my-app --idea IDEA.md

# 3. Let AutoFlow build it
cd my-app
autoflow start --parallel

# 4. Done! Your app is ready
docker-compose up
```

---

## Core Concepts

### Sprints

AutoFlow organizes work into **sprints** - small, focused units of work following TDD principles:

1. **WRITE_UNIT_TESTS** - Define behavior through tests
2. **WRITE_CODE** - Minimal implementation
3. **CODE_REVIEW** - Quality gate
4. **RUN_UNIT_TESTS** - Verify functionality
5. **WRITE_E2E_TESTS** - End-to-end validation
6. **RUN_E2E_TESTS** - Integration check
7. **COMPLETE** → **DONE**

### Git Worktrees

Each sprint runs in an **isolated git worktree** with:
- Unique branch (`sprint-{id}`)
- Dedicated port (`3000 + sprint_id × 10`)
- Separate environment

This prevents conflicts and enables parallel development.

### Agents

**Agents** are specialized AI assistants for specific tasks:
- `make-docs` - Generate specifications
- `make-sprints` - Create sprint plans
- `test-writer` - Write unit tests
- `code-implementer` - Implement features
- `reviewer` - Code review
- `debug-blocker` - Investigate bugs

---

## Workflows

### Workflow 1: Creating a New Project from IDEA

**Goal**: Go from idea to running application autonomously.

#### Step 1: Write Your Idea

Create `IDEA.md` describing your project:

```markdown
# E-Commerce Platform

## Overview
A full-stack e-commerce platform with product catalog, shopping cart,
checkout, and admin dashboard.

## Features
- User authentication (email + OAuth)
- Product browsing with filters and search
- Shopping cart with session persistence
- Stripe payment integration
- Admin dashboard for product management
- Real-time inventory updates

## Tech Stack
- Frontend: React + TypeScript + Tailwind CSS
- Backend: Node.js + Express + PostgreSQL
- Real-time: WebSockets
- Deployment: Docker + Docker Compose

## Requirements
- Mobile-responsive design
- < 3s page load time
- Support 10k concurrent users
- PCI-DSS compliant payment handling
```

#### Step 2: Create Project

```bash
autoflow create ecommerce-platform --idea IDEA.md
```

This will:
1. Create project directory
2. Initialize git repository
3. Generate comprehensive documentation:
   - `BUILD_SPEC.md` - Technical specification
   - `ARCHITECTURE.md` - System design
   - `API_SPEC.md` - API endpoints
   - `UI_SPEC.md` - UI/UX specifications
4. Analyze tech stack and dependencies
5. Generate complete sprint plan (`SPRINTS.yml`)

#### Step 3: Review Generated Plan

```bash
cd ecommerce-platform

# View all sprints
autoflow sprints list

# Check specific sprint
autoflow sprints show 1

# Review documentation
cat BUILD_SPEC.md
cat ARCHITECTURE.md
cat .autoflow/SPRINTS.yml
```

#### Step 4: Start Autonomous Development

**Option A: Sequential** (safer, easier to debug)
```bash
autoflow start
```

**Option B: Parallel** (faster, recommended for independent sprints)
```bash
autoflow start --parallel
```

**Option C: Specific Sprint**
```bash
autoflow start --sprint 5
```

#### Step 5: Monitor Progress

```bash
# Check status
autoflow status

# Watch in real-time (terminal 2)
watch -n 5 autoflow status

# View sprint details
autoflow sprints show 3

# List worktrees
autoflow worktree list
```

#### Step 6: Test and Deploy

```bash
# Start development environment
autoflow env start

# Run tests
npm test

# View application
open http://localhost:3000

# When satisfied, create production build
docker-compose -f docker-compose.prod.yml up -d
```

**Complete Example Output**:
```
🚀 Creating new AutoFlow project...

📁 Creating project directory...
  ✓ Created: ecommerce-platform

📖 Reading IDEA.md...
  ✓ Read from: IDEA.md

🔧 Initializing git repository...
  ✓ Git initialized

⚙️  Initializing AutoFlow...
  ✓ AutoFlow initialized

📚 Generating project documentation...
  Spawning make-docs agent...
  ✓ Documentation generated

🔍 Analyzing project structure...
  ✓ Analysis complete

📋 Generating sprint plan...
  Spawning make-sprints agent...
  ✓ Sprint plan generated
  ✓ Saved to .autoflow/SPRINTS.yml

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✅ Project Created Successfully!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Project Stats:
  Total Sprints: 24
  Status: Ready to start

🚀 Next Steps:
  1. Review the generated files:
     cd ecommerce-platform
     cat BUILD_SPEC.md

  2. Start autonomous development:
     autoflow start --parallel (recommended)
     autoflow start (sequential)

  3. Monitor progress:
     autoflow status
```

---

### Workflow 2: Adding Features to Existing Project

**Goal**: Add new features to an existing codebase without breaking anything.

#### Step 1: Analyze Existing Codebase

```bash
cd existing-project

# Initialize AutoFlow if not already done
autoflow init

# Analyze codebase structure
autoflow analyze
```

This creates `INTEGRATION_GUIDE.md` with:
- Tech stack detection
- File structure analysis
- Integration patterns
- Testing strategy
- Naming conventions

#### Step 2: Describe New Feature

```bash
autoflow add "Add real-time notifications using WebSockets"
```

Or with detailed requirements:

```bash
autoflow add "Payment processing" \
  --requirements "Use Stripe API, support credit cards and PayPal,
  handle webhooks for async events, store transactions in database"
```

This will:
1. Analyze existing code
2. Generate new sprints for the feature
3. Add integration points to `SPRINTS.yml`
4. Update `INTEGRATION_GUIDE.md`

#### Step 3: Review New Sprints

```bash
# View newly added sprints
autoflow sprints list | grep PENDING

# Check integration points
autoflow sprints show 25 --integration
```

#### Step 4: Implement Feature

```bash
# Run new sprints
autoflow start --sprint 25

# Or run all pending sprints
autoflow sprints list | grep PENDING | while read id _; do
  autoflow start --sprint $id
done
```

#### Step 5: Test Integration

```bash
# Run all tests including new ones
npm test

# Run E2E tests
npm run test:e2e

# Manual testing
autoflow env start
open http://localhost:3000
```

**Example Output**:
```
🔍 Analyzing codebase...
  Detected: React + Node.js + PostgreSQL
  Found: 156 files, 12,453 lines of code
  ✓ INTEGRATION_GUIDE.md created

🤖 Generating feature sprints...
  Feature: Real-time notifications
  Spawning make-sprints agent...

  Generated 4 new sprints:
    Sprint 25: WebSocket server setup
    Sprint 26: Notification data models
    Sprint 27: Frontend WebSocket client
    Sprint 28: Notification UI components

  ✓ Added to SPRINTS.yml

🚀 Ready to implement!
  Run: autoflow start --sprint 25
```

---

### Workflow 3: Fixing Bugs

**Goal**: Investigate, reproduce, fix, and verify bug fixes autonomously.

#### Step 1: Report Bug

```bash
autoflow fix "Login button doesn't work on mobile"
```

Or with automatic fix:

```bash
autoflow fix "Search returns wrong results" --auto-fix
```

This will:
1. Create bugfix worktree (`sprint-900`, port 12000)
2. Create branch `bugfix-login-button-doesnt-work`
3. Spawn `debug-blocker` agent to investigate
4. Generate bug analysis report

#### Step 2: Review Investigation

```bash
# Read bug analysis
cat .autoflow/bugs/bug-20250106-143022.md
```

Example analysis:
```markdown
# Bug Analysis

**Date**: 2025-01-06 14:30:22
**Description**: Login button doesn't work on mobile

## Root Cause

Button has CSS `pointer-events: none` on mobile viewport,
preventing touch events.

## Affected Files

- `src/components/LoginForm.css` (line 45)
- `src/components/LoginForm.test.tsx` (missing mobile tests)

## Proposed Solution

1. Remove `pointer-events: none` from button
2. Add proper touch event handling
3. Increase touch target size to 44px minimum
4. Add mobile-specific E2E tests

## Test Results

✓ Desktop tests pass
✗ Mobile E2E tests fail (button not clickable)

## Implementation

Fixed in bugfix worktree at: ../sprint-900
```

#### Step 3: Test Fix

```bash
# Switch to bugfix worktree
cd ../sprint-900

# Verify fix
npm test
npm run test:e2e:mobile

# Manual testing
npm run dev  # Runs on port 12000
# Test on mobile device or emulator
```

#### Step 4: Merge Fix

```bash
# Return to main project
cd -

# Merge bugfix branch
autoflow worktree merge sprint-900

# Clean up
autoflow worktree delete sprint-900
```

**Example Output**:
```
🐛 Investigating bug...
Bug: Login button doesn't work on mobile

Creating bugfix worktree...
  ✓ Worktree created: ../sprint-900
  Branch: bugfix-login-button-doesnt-work
  Port: 12000

Running bug investigation...
  Spawning debug-blocker agent...
  ✓ Investigation complete

Investigation Results:
Root Cause: CSS pointer-events preventing touch interaction
Files Modified:
  - src/components/LoginForm.css
  - src/components/LoginForm.test.tsx

✓ Analysis saved to .autoflow/bugs/bug-20250106-143022.md

Next steps:
  1. Review fix in bugfix worktree
  2. Run tests: cd ../sprint-900 && npm test
  3. Merge if tests pass: autoflow worktree merge sprint-900
```

---

### Workflow 4: Working with Git Worktrees

**Goal**: Understand and manage isolated development environments.

#### List All Worktrees

```bash
autoflow worktree list

# Filter by type
autoflow worktree list --type=sprint
autoflow worktree list --type=bugfix
```

Example output:
```
📂 Active Worktrees

Sprint Worktrees:
  sprint-5
    Branch: sprint-5-user-authentication
    Path: ../sprint-5
    Port: 3050

  sprint-12
    Branch: sprint-12-payment-integration
    Path: ../sprint-12
    Port: 3120

Bugfix Worktrees:
  sprint-900
    Branch: bugfix-login-issue
    Path: ../sprint-900
    Port: 12000
```

#### Create Manual Worktree

```bash
autoflow worktree create feature/advanced-search
```

#### Work in Worktree

```bash
# Switch to worktree
cd ../sprint-5

# Make changes
vim src/auth/login.ts
git add .
git commit -m "Implement OAuth login"

# Run tests in isolation
npm test

# Start dev server (unique port)
npm run dev  # Uses port 3050
```

#### Merge Worktree

```bash
# Return to main project
cd -

# Merge completed work
autoflow worktree merge sprint-5

# Verify merge
git log

# Clean up worktree
autoflow worktree delete sprint-5
```

#### Handle Merge Conflicts

```bash
# If merge fails with conflicts
cd ../sprint-5

# Resolve conflicts
git status
# Edit conflicting files
vim src/conflicting-file.ts

# Complete merge
git add .
git commit

# Return and retry merge
cd -
autoflow worktree merge sprint-5
```

#### Clean Up Stale Worktrees

```bash
# Prune deleted but not removed worktrees
autoflow worktree prune

# Force delete problematic worktree
autoflow worktree delete sprint-8 --force
```

---

## Command Reference

### Project Management

```bash
autoflow create <name> [--idea IDEA.md]  # Create new project
autoflow init [--template react-node]    # Initialize in existing dir
autoflow status [--json]                 # Show project status
autoflow analyze                         # Analyze codebase structure
```

### Development

```bash
autoflow start [--parallel] [--sprint ID]  # Start autonomous development
autoflow add "feature description"         # Add new feature
autoflow fix "bug description" [--auto-fix] # Fix bug
autoflow rollback [--sprint ID]            # Reset sprint to PENDING
```

### Sprints

```bash
autoflow sprints list                      # List all sprints
autoflow sprints show <id> [--integration] # Show sprint details
```

### Worktrees

```bash
autoflow worktree list [--type sprint|bugfix]  # List worktrees
autoflow worktree create <branch>              # Create worktree
autoflow worktree merge <branch>               # Merge to main
autoflow worktree delete <branch> [--force]    # Delete worktree
autoflow worktree prune                        # Clean up stale references
```

### Environment

```bash
autoflow env start               # Start Docker containers
autoflow env stop                # Stop containers
autoflow env restart             # Restart containers
autoflow env logs [--follow]     # View container logs
autoflow env health              # Check service health
```

### Configuration

```bash
autoflow agents [--detailed]      # List available agents
autoflow skills                   # List available skills
autoflow mcp install [servers...] # Install MCP servers
autoflow mcp list                 # List installed servers
autoflow mcp info [server]        # Show server information
```

### Validation

```bash
autoflow validate [--fix]  # Run quality gates
```

---

## Advanced Usage

### Custom Sprint Configuration

Edit `.autoflow/SPRINTS.yml`:

```yaml
sprints:
  - id: 1
    goal: "User authentication system"
    status: PENDING
    duration: "4 hours"
    total_effort: "8 hours"
    max_effort: "12 hours"
    must_complete_first: true  # Block other sprints until this completes
    dependencies:
      - "Sprint 0: Infrastructure setup"
    integration_points:
      modifies:
        - "src/api/auth.ts"
        - "src/middleware/auth.ts"
      creates:
        - "src/models/User.ts"
        - "src/services/AuthService.ts"
      patterns:
        - "Use JWT for tokens"
        - "Store refresh tokens in Redis"
```

### Parallel Execution Strategies

```bash
# Execute independent sprints in parallel
autoflow start --parallel

# Manual parallel execution (advanced)
for sprint_id in 5 6 7 8; do
  (autoflow start --sprint $sprint_id &)
done
wait
```

### Custom Agent Configuration

Edit agent definitions in `~/.claude/agents/*.agent.md`:

```markdown
---
model: claude-sonnet-4-5
tools: Read, Write, Edit, Bash, Grep
max_turns: 10
---

# Custom Agent Instructions

You are a specialized agent for...
```

### Environment Variables

```bash
# Set custom paths
export AUTOFLOW_HOME=~/.autoflow
export AUTOFLOW_AGENTS_DIR=~/.claude/agents

# Agent configuration
export AUTOFLOW_MODEL=claude-sonnet-4-5
export AUTOFLOW_MAX_TURNS=10
export AUTOFLOW_TIMEOUT=600

# Feature flags
export AUTOFLOW_PARALLEL=true
export AUTOFLOW_AUTO_FIX=true
```

### Scripting with AutoFlow

```bash
#!/bin/bash
# Automated feature development script

FEATURE="$1"
if [ -z "$FEATURE" ]; then
  echo "Usage: $0 'feature description'"
  exit 1
fi

# Add feature
autoflow add "$FEATURE"

# Get new sprint IDs
NEW_SPRINTS=$(autoflow sprints list --json | jq '.[] | select(.status=="PENDING") | .id')

# Execute each sprint
for sprint_id in $NEW_SPRINTS; do
  echo "Executing sprint $sprint_id..."
  autoflow start --sprint $sprint_id || {
    echo "Sprint $sprint_id failed"
    autoflow rollback --sprint $sprint_id
    exit 1
  }
done

# Run full test suite
npm test

echo "Feature '$FEATURE' completed!"
```

---

## Best Practices

### 1. Start Small

```bash
# Don't create massive projects at once
# Break down into phases

# Phase 1: Core functionality
autoflow create my-app-core --idea CORE_IDEA.md
autoflow start --parallel

# Phase 2: Add features incrementally
cd my-app-core
autoflow add "User profiles"
autoflow add "Notifications"
autoflow start
```

### 2. Review Before Merging

```bash
# Always review generated code
cd ../sprint-5
git diff main
npm test
npm run lint

# Only merge when satisfied
cd -
autoflow worktree merge sprint-5
```

### 3. Use Descriptive Names

```bash
# Good
autoflow fix "Login form validation fails for emails with + symbol"
autoflow add "Add real-time collaboration using WebSockets and CRDT"

# Bad
autoflow fix "Fix bug"
autoflow add "Add feature"
```

### 4. Leverage Integration Points

```yaml
# In SPRINTS.yml, always specify integration points
integration_points:
  modifies:
    - "src/existing/file.ts"  # Files that will be modified
  creates:
    - "src/new/feature.ts"     # New files to create
  tests_existing:
    - "tests/integration/auth.test.ts"  # Tests to update
  patterns:
    - "Follow repository pattern"  # Patterns to maintain
    - "Use dependency injection"
```

### 5. Monitor Resource Usage

```bash
# Check worktree disk usage
du -sh ../*sprint*

# Monitor running services
autoflow env health

# Check Docker resources
docker stats
```

### 6. Regular Cleanup

```bash
# Delete merged worktrees
autoflow worktree list | grep merged | while read name _; do
  autoflow worktree delete "$name"
done

# Prune stale references
autoflow worktree prune
git worktree prune
```

### 7. Version Control Best Practices

```bash
# Commit AutoFlow config
git add .autoflow/SPRINTS.yml .autoflow/CLAUDE.md
git commit -m "Update sprint plan"

# Don't commit generated artifacts
echo ".autoflow/bugs/" >> .gitignore
echo ".autoflow/tasks/" >> .gitignore
```

### 8. Use Quality Gates

```bash
# Before merging any worktree
cd ../sprint-5
autoflow validate
npm run lint
npm test
npm run test:e2e

# Only merge if all pass
cd -
autoflow worktree merge sprint-5
```

---

## Next Steps

- **[Troubleshooting Guide](TROUBLESHOOTING.md)** - Fix common issues
- **[Configuration Guide](CONFIGURATION.md)** - Advanced configuration
- **[MCP Servers Guide](MCP_SERVERS.md)** - Extend capabilities
- **[Architecture Overview](ARCHITECTURE.md)** - Understand internals

---

**Questions?** Check the [FAQ](TROUBLESHOOTING.md) or [open an issue](https://github.com/autoflow/autoflow/issues).
