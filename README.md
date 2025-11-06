# AutoFlow - Autonomous Coding Agent

🚀 **Status**: Core Infrastructure Complete (v0.1.1) - Agent Setup Required

AutoFlow is a fully autonomous TDD-driven coding agent that takes you from requirements to production-ready code with minimal manual intervention. Built in Rust for performance and reliability.

## ⚠️ Current Status

**What Works**:
- ✅ Core infrastructure (7 Rust crates, 14 CLI commands)
- ✅ Git worktree isolation
- ✅ Quality gates and validation
- ✅ Project initialization and status tracking
- ✅ MCP server management

**What Requires Setup**:
- ⚠️ Autonomous workflows need agent definitions ([see setup guide](SETUP_REQUIRED.md))
- ⚠️ `agents/` and `skills/` directories not included in repo (yet)

**Quick Assessment**:
- Want to use AutoFlow TODAY? → See [SETUP_REQUIRED.md](SETUP_REQUIRED.md) for agent setup
- Want manual workflow? → Commands work without agents
- Contributing? → Help add agent definitions to the repo!

## Features

- ✅ **Fully Autonomous**: From `IDEA.md` to running application
- 🧪 **TDD Pipeline**: Automated test-first development (RED → GREEN → REFACTOR → REVIEW)
- 🌲 **Git Worktree Isolation**: Each sprint/bugfix in isolated workspace
- 🐛 **Autonomous Bug Fixing**: Investigate, reproduce, fix, and test
- 🔍 **Code-Aware**: Analyzes existing codebases and integrates seamlessly
- 🐳 **Environment Setup**: Automatic Docker, databases, services configuration
- 🎯 **Quality Gates**: Multi-layer validation to catch mistakes
- 📊 **Observable**: Structured logging, metrics, progress tracking

## Quick Start

### Prerequisites

- **Rust 1.70+** (for building from source)
- **Claude CLI** (not Claude Desktop) - https://claude.com/cli
- **Docker & Docker Compose** (optional, for dev environments)
- **Git** 2.20+ (for worktree support)

### Installation

```bash
# Clone repository
git clone https://github.com/autoflow/autoflow
cd autoflow

# Run installer (builds and installs everything)
./scripts/install.sh

# Reload shell
source ~/.bashrc  # or ~/.zshrc

# Verify installation
autoflow --version
```

The installer will:
- Build the release binary
- Install to `~/.autoflow/bin/`
- Copy 25+ agents to `~/.claude/agents/`
- Copy 13+ skills to `~/.claude/skills/`
- Add to your PATH
- Create configuration files

**Works with existing Claude Code setup** - Won't overwrite your custom agents or skills.

### Create Your First Project

⚠️ **Note**: Autonomous workflows require agent setup. See [SETUP_REQUIRED.md](SETUP_REQUIRED.md) for details.

```bash
# 1. Write your idea
cat > IDEA.md << 'EOF'
# Task Manager App
A real-time task management app with user auth,
task CRUD, WebSocket updates, and mobile support.
Tech: React + Node.js + PostgreSQL
EOF

# 2. Create project (requires make-docs and make-sprints agents)
autoflow create my-app --idea IDEA.md

# 3. Build autonomously (requires sprint execution agents)
cd my-app
autoflow start --parallel

# 4. Done! Your app is ready
docker-compose up
open http://localhost:3000
```

**Alternative (No Agents Required)**:
```bash
# Manual workflow without agents
mkdir my-app && cd my-app
autoflow init
# Manually create BUILD_SPEC.md and edit .autoflow/SPRINTS.yml
```

### Work with Existing Projects

```bash
# Initialize in existing project
cd existing-project
autoflow init

# Analyze codebase
autoflow analyze

# Add new feature
autoflow add "Add payment processing with Stripe"

# Fix a bug
autoflow fix "Login button doesn't work on mobile"

# Check progress
autoflow status

# Manage isolated workspaces
autoflow worktree list
```

## Architecture

```
autoflow/
├── crates/
│   ├── autoflow-cli/       # CLI application (14 commands)
│   ├── autoflow-core/      # Orchestration & state machine
│   ├── autoflow-agents/    # Agent execution & management
│   ├── autoflow-quality/   # Quality gates & validation
│   ├── autoflow-data/      # Data structures (Sprint, Task, etc.)
│   ├── autoflow-git/       # Git worktree operations
│   └── autoflow-utils/     # Shared utilities
├── agents/                 # 25+ specialized agents
├── skills/                 # 13+ diagnostic skills
├── reference/              # Standards & guides
├── schemas/                # JSON schemas
└── templates/              # Project templates
```

## Documentation

### User Documentation
- **[USER_GUIDE.md](USER_GUIDE.md)** - Complete workflows and examples
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - Common issues and solutions
- **[MCP_SERVERS.md](MCP_SERVERS.md)** - Extend capabilities with MCP servers
- **[CONFIGURATION.md](CONFIGURATION.md)** - Global vs project-level config

### Developer Documentation
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design and internals
- **[CHANGELOG.md](CHANGELOG.md)** - Version history

## Commands

### Project Management
```bash
autoflow create <name> --idea IDEA.md  # Create new project from idea
autoflow init [--template react-node]  # Initialize in existing directory
autoflow status [--json]                # Show sprint progress
autoflow analyze                        # Analyze codebase structure
```

### Development
```bash
autoflow start [--parallel] [--sprint ID]  # Start autonomous development
autoflow add "feature description"         # Add new feature
autoflow fix "bug description"             # Investigate and fix bug
autoflow rollback [--sprint ID]            # Reset sprint to PENDING
```

### Worktrees (Isolated Workspaces)
```bash
autoflow worktree list                  # List all worktrees
autoflow worktree create <branch>       # Create new worktree
autoflow worktree merge <branch>        # Merge to main
autoflow worktree delete <branch>       # Remove worktree
```

### Sprints & Agents
```bash
autoflow sprints list                   # List all sprints
autoflow sprints show <id>              # Show sprint details
autoflow agents [--detailed]            # List available agents
autoflow skills                         # List available skills
```

### Environment & Quality
```bash
autoflow env start|stop|restart         # Manage Docker containers
autoflow env logs [--follow]            # View container logs
autoflow validate [--fix]               # Run quality gates
autoflow mcp install [servers...]       # Install MCP servers
```

## Example Workflows

### Creating a New App

```bash
autoflow create ecommerce --idea IDEA.md
cd ecommerce
autoflow start --parallel

# Monitor progress
autoflow status

# Output:
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#   Sprint Progress
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# ✓ Sprint 1: Infrastructure Setup (DONE)
# ✓ Sprint 2: Database Models (DONE)
# ⚙ Sprint 3: API Endpoints (IN_PROGRESS)
# ⏳ Sprint 4: Frontend Components (PENDING)
#
# Total: 24 sprints
# Completed: 2 | In Progress: 1 | Pending: 21
```

### Fixing a Bug

```bash
autoflow fix "Login button not working on mobile"

# Output:
# 🐛 Bug Investigation
#
# Creating bugfix worktree...
#   ✓ Worktree created: ../sprint-900 (port 12000)
#
# Running investigation...
#   ✓ Root cause identified: Missing CSS media queries
#   ✓ Fix implemented
#   ✓ Tests created
#
# Next steps:
#   1. Review: cd ../sprint-900
#   2. Test: npm test
#   3. Merge: autoflow worktree merge sprint-900
```

### Adding a Feature

```bash
autoflow add "Add real-time notifications using WebSockets"

# Output:
# 🔍 Analyzing codebase...
#   Detected: React + Node.js + PostgreSQL
#
# 🤖 Generating feature sprints...
#   Generated 4 new sprints:
#     Sprint 25: WebSocket server setup
#     Sprint 26: Notification data models
#     Sprint 27: Frontend WebSocket client
#     Sprint 28: Notification UI components
#
# ✓ Added to SPRINTS.yml
#
# Run: autoflow start --sprint 25
```

## Development

### Building from Source

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run from source
cargo run -- --help

# Run tests
cargo test --all

# Watch mode (requires cargo-watch)
cargo watch -x 'run -- status'
```

### Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific crate
cargo test -p autoflow-data

# With output
cargo test -- --nocapture
```

## Implementation Status

✅ **Phase 1-6: COMPLETE**
- Full TDD pipeline automation
- Git worktree isolation
- Autonomous bug fixing
- Feature addition to existing code
- Quality gates & validation
- MCP server integration

⏳ **Phase 7: Enhancement (Upcoming)**
- Progress bars with `indicatif`
- Interactive modes (`-i` flag)
- `autoflow doctor` diagnostics
- Performance optimizations
- Comprehensive test suite

## Troubleshooting

**Common Issues**:

```bash
# Command not found
source ~/.bashrc

# Project not initialized
autoflow init

# Agent missing
./scripts/install.sh

# Sprint blocked
autoflow rollback --sprint <id>

# Worktree conflicts
autoflow worktree delete <name> --force
```

See **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** for complete guide.

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

See the code quality guidelines:
- Follow SOLID principles
- Keep functions < 50 lines
- Write descriptive error messages
- Add doc comments for public APIs
- Test coverage > 70%

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Claude CLI](https://claude.com/cli)
- Inspired by TDD best practices

---

**Built with ❤️  by the AutoFlow community**

[Documentation](USER_GUIDE.md) | [Troubleshooting](TROUBLESHOOTING.md) | [Configuration](CONFIGURATION.md) | [MCP Servers](MCP_SERVERS.md) | [Architecture](ARCHITECTURE.md)
