# MCP Servers for AutoFlow

MCP (Model Context Protocol) servers extend AutoFlow's capabilities by providing specialized tools and integrations.

## Quick Start

```bash
# Install all recommended MCP servers
autoflow mcp install

# Or install specific servers
autoflow mcp install memory playwright github

# List installed servers
autoflow mcp list

# Get info about available servers
autoflow mcp info
autoflow mcp info memory
```

## Recommended Servers

### 1. **memory** - Knowledge Graph Memory

**Why you need it**: Builds up project knowledge across sessions. AutoFlow learns from your codebase and remembers patterns, bugs, and solutions.

**Use cases**:
- Store learnings about your project's architecture
- Remember bug fixes and their solutions
- Build knowledge graph of functions and their relationships
- Track technical debt and refactoring opportunities

**Capabilities**:
- `memory.create_entities()` - Store functions, classes, patterns, bugs
- `memory.add_observations()` - Add notes about code behavior
- `memory.create_relations()` - Link entities (e.g., "function X calls function Y")
- `memory.search_nodes()` - Query stored knowledge

**Installation**:
```bash
autoflow mcp install memory
```

**Example usage in agents**:
```markdown
Before implementing, query memory:
1. Search for similar patterns: memory.search_nodes("authentication pattern")
2. Check for known issues: memory.search_nodes("login bug")
3. After fixing, store the solution: memory.create_entities()
```

---

### 2. **playwright** - Browser Automation

**Why you need it**: Essential for E2E testing and autonomous bug fixing. Allows agents to interact with your running application to reproduce bugs and verify fixes.

**Use cases**:
- E2E test automation
- Bug reproduction (see exactly what's failing)
- Screenshot generation for documentation
- UI interaction testing
- Mobile responsiveness testing

**Capabilities**:
- `playwright.launch()` - Start browser (Chrome, Firefox, Safari)
- `playwright.navigate()` - Go to URL
- `playwright.click()` - Click elements
- `playwright.screenshot()` - Capture visuals
- `playwright.fill()` - Fill forms
- `playwright.computed_styles()` - Get CSS properties

**Installation**:
```bash
autoflow mcp install playwright
```

**Used by**:
- `autoflow fix` - Bug investigation with visual reproduction
- `e2e-writer` agent - Generate E2E tests
- `e2e-fixer` agent - Fix failing tests
- `debug-blocker` agent - Reproduce UI bugs

---

### 3. **github** - GitHub Integration

**Why you need it**: Automate GitHub workflows - create issues, PRs, manage repositories directly from AutoFlow.

**Use cases**:
- Auto-create GitHub issues for bugs
- Generate pull requests from completed sprints
- Search repositories for similar solutions
- Manage project boards
- Review and comment on PRs

**Capabilities**:
- `github.create_issue()` - Create issues with labels
- `github.create_pull_request()` - Open PRs
- `github.list_pull_requests()` - Query PRs
- `github.search_repositories()` - Find similar projects
- `github.get_file_contents()` - Read files from repos

**Installation**:
```bash
autoflow mcp install github

# Then set your GitHub token:
# Edit ~/.claude/claude_desktop_config.json
# Set GITHUB_TOKEN in the github server config
```

**Generate token**: https://github.com/settings/tokens

---

### 4. **postgres** - PostgreSQL Database

**Why you need it**: Direct database access for schema inspection, migrations, and data analysis during development.

**Use cases**:
- Inspect database schema
- Run migrations during sprints
- Query data for testing
- Analyze performance issues
- Generate seed data

**Capabilities**:
- `postgres.query()` - Execute SQL
- `postgres.list_tables()` - Show all tables
- `postgres.describe_table()` - Get table schema
- `postgres.analyze_query()` - Query performance

**Installation**:
```bash
autoflow mcp install postgres

# Then set connection string:
# Edit ~/.claude/claude_desktop_config.json
# Set POSTGRES_CONNECTION_STRING
# Example: postgresql://user:pass@localhost:5432/dbname
```

---

### 5. **filesystem** - Advanced File Operations

**Why you need it**: Enhanced file operations beyond basic read/write, with search capabilities.

**Use cases**:
- Search file contents across directories
- Batch file operations
- Directory management
- File pattern matching
- Safe file moves and renames

**Capabilities**:
- `filesystem.read_file()` - Read with encoding detection
- `filesystem.write_file()` - Write with safety checks
- `filesystem.search_files()` - Content search
- `filesystem.list_directory()` - Enhanced ls
- `filesystem.move()` - Safe file moves

**Installation**:
```bash
autoflow mcp install filesystem
```

---

### 6. **git** - Git Repository Access

**Why you need it**: Read git history, diffs, and commit info to understand code evolution.

**Use cases**:
- Analyze commit history
- View file changes over time
- Find when bugs were introduced
- Understand code evolution
- Generate changelogs

**Capabilities**:
- `git.log()` - Read commit history
- `git.diff()` - Show changes
- `git.show()` - View specific commits
- `git.blame()` - See who changed what
- `git.branches()` - List branches

**Installation**:
```bash
autoflow mcp install git
```

---

### 7. **fetch** - HTTP Requests

**Why you need it**: Fetch documentation, test APIs, scrape web content.

**Use cases**:
- Test API endpoints during development
- Fetch library documentation
- Scrape web content for data
- Verify external integrations
- Download resources

**Capabilities**:
- `fetch.get()` - HTTP GET requests
- `fetch.post()` - HTTP POST requests
- `fetch.parse_html()` - Extract content
- `fetch.download()` - Save files

**Installation**:
```bash
autoflow mcp install fetch
```

---

### 8. **sqlite** - SQLite Database

**Why you need it**: Local database access for SQLite-based projects.

**Use cases**:
- Mobile app databases
- Local data storage
- Testing with SQLite
- Data analysis
- Seed data generation

**Capabilities**:
- `sqlite.query()` - Execute SQL
- `sqlite.list_tables()` - Show schema
- `sqlite.pragma()` - Database info

**Installation**:
```bash
autoflow mcp install sqlite
```

---

## Installation Workflow

### Option 1: Install All (Recommended)

```bash
# One command installs all 8 servers
autoflow mcp install
```

This installs:
- memory (knowledge graph)
- playwright (browser automation)
- github (GitHub API)
- postgres (PostgreSQL)
- filesystem (file operations)
- git (repository access)
- fetch (HTTP requests)
- sqlite (local database)

### Option 2: Install Specific Servers

```bash
# Essential trio for most projects
autoflow mcp install memory playwright github

# Backend-focused
autoflow mcp install memory postgres git

# Frontend-focused
autoflow mcp install memory playwright fetch
```

## Configuration

After installation, servers are configured in:
```
~/.claude/claude_desktop_config.json
```

### Setting Environment Variables

Some servers need configuration:

**GitHub (requires token)**:
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_TOKEN": "ghp_your_token_here"
      }
    }
  }
}
```

**PostgreSQL (requires connection string)**:
```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://user:pass@localhost:5432/dbname"
      }
    }
  }
}
```

## How MCP Servers Enhance AutoFlow

### Bug Fixing with Playwright

```bash
autoflow fix "Login button not working on mobile"
```

**Without playwright**: Agent reads code, guesses the issue

**With playwright**:
1. Agent launches browser
2. Navigates to login page
3. Tries to click button
4. Takes screenshot
5. Gets computed styles
6. Sees exact issue: button too small (32px instead of 48px)
7. Implements precise fix
8. Verifies fix works

### Knowledge Accumulation with Memory

**First bug fix**:
```bash
autoflow fix "React state not updating"
# memory.create_entities(): "React setState is async, need useEffect"
```

**Second bug fix (similar issue)**:
```bash
autoflow fix "Component not re-rendering"
# memory.search_nodes("React state") → finds previous solution
# Applies learned pattern immediately
```

### GitHub Integration

```bash
# After completing sprint
autoflow worktree merge sprint-5

# Agent automatically:
# 1. Creates PR on GitHub
# 2. Adds description from sprint goal
# 3. Links related issues
# 4. Requests review
```

## Verification

After installation:

```bash
# Check installed servers
autoflow mcp list

# Should show:
# Found 8 servers:
#   • memory
#   • playwright
#   • github
#   • postgres
#   • filesystem
#   • git
#   • fetch
#   • sqlite

# Restart Claude Desktop for changes to take effect
```

## Troubleshooting

### Server not found

```bash
# Check Node.js/npm installed
node --version
npm --version

# Reinstall specific server
autoflow mcp install memory
```

### GitHub token not working

```bash
# Verify token has correct permissions:
# - repo (full control)
# - workflow (update workflows)

# Test token:
curl -H "Authorization: token YOUR_TOKEN" https://api.github.com/user
```

### PostgreSQL connection fails

```bash
# Test connection string:
psql "postgresql://user:pass@localhost:5432/dbname"

# Update in config:
nano ~/.claude/claude_desktop_config.json
```

## Best Practices

1. **Install memory first** - Builds knowledge over time
2. **Use playwright for bug fixing** - Visual reproduction is powerful
3. **Configure github early** - Automates PR workflow
4. **Set database connections** - Enables schema introspection
5. **Restart Claude Desktop** - After any config changes

## Supported Agents

These agents leverage MCP servers:

| Agent | Uses MCP Server | Purpose |
|-------|----------------|---------|
| debug-blocker | playwright, memory | Bug investigation with browser |
| e2e-writer | playwright | Generate E2E tests |
| e2e-fixer | playwright | Fix failing E2E tests |
| code-implementer | memory, git | Learn from history |
| reviewer | memory | Remember past issues |
| health-check | postgres, sqlite | Database validation |
| make-sprints | memory, fetch | Learn patterns, fetch docs |

---

**Install now to supercharge AutoFlow!**

```bash
autoflow mcp install
```

After installation, restart Claude Desktop and all MCP servers will be available to AutoFlow agents.
