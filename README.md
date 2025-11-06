# AutoFlow - Autonomous Coding Agent

🚀 **Status**: v0.1.3 - Production Ready with Auto-Update

AutoFlow is a fully autonomous TDD-driven coding agent that takes you from requirements to production-ready code with minimal manual intervention. Just create an `IDEA.md` file and run `autoflow start` - everything else is automated. Built in Rust for performance and reliability.

## ✅ Current Status

**Fully Working - Production Ready**:
- ✅ Core infrastructure (7 Rust crates, 15 CLI commands)
- ✅ 13 production agents included in repo
- ✅ Complete TDD pipeline (IDEA → Docs → Sprints → Tests → Code → Review)
- ✅ Git worktree isolation for parallel development
- ✅ Quality gates and validation
- ✅ Project initialization and status tracking
- ✅ MCP server management
- ✅ Auto-update system for agents/skills
- ✅ Documentation pivot/refinement command
- ✅ Autonomous bug fixing with investigation
- ✅ Feature addition to existing codebases

**What's Included**:
- 📦 13 specialized agents (make-docs, make-sprints, code-implementer, test writers, fixers, reviewers)
- 📋 10 comprehensive documentation types (BUILD_SPEC, ARCHITECTURE, API_SPEC, UI_SPEC, DATA_MODEL, etc.)
- 🔄 Automatic agent/skill updates on startup
- 🎯 Smart sprint state preservation

**Installation**:
```bash
git clone https://github.com/ddunford/autoflow
cd autoflow
./scripts/install.sh
```

The installer automatically sets up all agents, skills, and configuration. Just run `autoflow create my-project --idea IDEA.md` to get started!

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
git clone https://github.com/ddunford/autoflow
cd autoflow

# Run installer (handles everything automatically)
./scripts/install.sh

# Verify installation
autoflow --version
```

The installer will:
- Build the release binary with `cargo build --release`
- Install to `~/.autoflow/bin/`
- Copy 13 production agents to `~/.claude/agents/` (with `.agent.md` suffix)
- Copy skills (if any) to `~/.claude/skills/`
- Add to your PATH ($HOME/.bashrc or $HOME/.zshrc)
- Create configuration files

**Works with existing Claude Code setup** - Uses `.agent.md` suffix to avoid conflicts with your custom agents.

### Create Your First Project

**New Simplified Workflow**: Just create an `IDEA.md` and run `autoflow start`!

```bash
# 1. Create a directory with your idea
mkdir my-app && cd my-app

cat > IDEA.md << 'EOF'
# Task Manager App
A real-time task management app with user auth,
task CRUD, WebSocket updates, and mobile support.
Tech: React + Node.js + PostgreSQL
EOF

# 2. Start autonomous development
autoflow start --parallel

# That's it! AutoFlow will:
# - Generate comprehensive documentation (.autoflow/docs/)
# - Create sprint plan (.autoflow/SPRINTS.yml)
# - Execute all sprints in parallel
# - Build your app in ./src and ./tests
```

**What happens automatically:**
- ✅ Docs generation (BUILD_SPEC, ARCHITECTURE, API_SPEC, UI_SPEC)
- ✅ Sprint plan generation with proper task breakdown
- ✅ TDD workflow (tests → code → review → deploy)
- ✅ All code in `./src`, all tests in `./tests` (simple structure)

**Alternative - Create with template**:
```bash
# Create project with initial structure
autoflow create my-app --idea IDEA.md
cd my-app
autoflow start --parallel
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

### AutoFlow Repository Structure
```
autoflow/
├── crates/
│   ├── autoflow-cli/       # CLI application (15 commands)
│   ├── autoflow-core/      # Orchestration & state machine
│   ├── autoflow-agents/    # Agent execution & management
│   ├── autoflow-quality/   # Quality gates & validation
│   ├── autoflow-data/      # Data structures (Sprint, Task, etc.)
│   ├── autoflow-git/       # Git worktree operations
│   └── autoflow-utils/     # Shared utilities
├── agents/                 # 13 production agents (installed to ~/.claude/agents/)
├── skills/                 # Diagnostic skills (installed to ~/.claude/skills/)
├── reference/              # Standards & guides
├── schemas/                # JSON schemas
└── templates/              # Project templates
```

### Generated Project Structure
When you create a project with AutoFlow, it generates a simple, flat structure:

```
my-project/
├── src/                    # All source code (backend, frontend, everything)
├── tests/                  # All tests
├── .autoflow/
│   ├── docs/
│   │   ├── BUILD_SPEC.md         # Technical specification (always)
│   │   ├── ARCHITECTURE.md       # System architecture (always)
│   │   ├── TESTING_STRATEGY.md   # Testing approach & requirements (always)
│   │   ├── ERROR_HANDLING.md     # Error management patterns (always)
│   │   ├── DEPLOYMENT.md         # Deployment & operations (always)
│   │   ├── API_SPEC.md           # API documentation (if backend)
│   │   ├── UI_SPEC.md            # UI specifications (if frontend)
│   │   ├── DATA_MODEL.md         # Database schema (if database)
│   │   ├── STATE_MANAGEMENT.md   # Frontend state patterns (if frontend)
│   │   └── SECURITY.md           # Security implementation (if backend)
│   ├── sprints/
│   │   └── sprint-XXX/
│   │       └── logs/             # Agent execution logs (JSON & text)
│   ├── SPRINTS.yml         # Sprint plan with task breakdown
│   ├── CLAUDE.md           # Project context for agents
│   └── INTEGRATION_GUIDE.md # Existing codebase integration guide
├── IDEA.md                 # Your original project idea
└── .git/                   # Git repository
```

**Why `./src` instead of monorepo?**
- ✅ Simpler structure for most projects
- ✅ Easier navigation and development
- ✅ Agents can reason about the codebase more effectively
- ✅ Works great for microservices, full-stack apps, and libraries
- ⚠️ For complex monorepos, you can organize within `./src` as needed

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
autoflow pivot "instruction"               # Update docs and regenerate sprints
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

### Refining Documentation (Pivot)

```bash
# After reviewing generated docs, you spot something wrong
autoflow pivot "Add WebSocket support to the architecture - you forgot to include it"

# Output:
# 🔄 Pivoting project based on your feedback...
#
# 📖 Reading current documentation...
#   ✓ Read 10 documentation files
#
# 📋 Saving current sprint states...
#   ✓ Saved 24 sprint states
#
# 🤖 Updating documentation based on your feedback...
#   Spawning make-docs agent...
#   ✓ Documentation updated
#
# 📋 Regenerating sprint plan with updated documentation...
#   Spawning make-sprints agent...
#   ✓ Sprint plan regenerated
#   ✓ Restored sprint states (kept 2 active/completed sprints)
#   ✓ Saved to .autoflow/SPRINTS.yml
#
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#   ✅ Pivot Complete!
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# 📝 What changed:
#   • Documentation updated based on your feedback
#   • Sprint plan regenerated from updated docs
#   • Existing sprint states preserved where possible
```

**When to use `pivot`:**
- Documentation missed a key requirement
- Architecture needs adjustment before coding starts
- API design changed after review
- Database schema needs modification
- Any time you catch an issue in the generated docs

**Smart state preservation:**
- Keeps completed sprints (`DONE`)
- Keeps in-progress sprints (any phase in TDD pipeline)
- Resets pending/blocked sprints to match new plan

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

## Future Improvements

### Planned Features

**High Priority**:
- 🔄 **Resume Interrupted Sprints**: Graceful handling of Ctrl+C with state persistence
- 🎯 **Smart Context Loading**: Selective file reading based on task requirements (reduce token usage)
- 📊 **Real-time Progress Dashboard**: Live view of parallel sprint execution with ETA
- 🧪 **Test Coverage Tracking**: Enforce minimum coverage thresholds per sprint
- 🔍 **Intelligent Code Search**: Better context gathering for existing codebases
- 💾 **Sprint Checkpoints**: Save/restore sprint state at each phase transition

**Medium Priority**:
- 🤖 **Agent Hot-Reload**: Update agents without restarting workflow
- 🎨 **Project Templates**: Pre-configured setups (e-commerce, SaaS, mobile, etc.)
- 🔐 **Secrets Management**: Secure handling of API keys and credentials
- 📝 **Documentation Generation**: Auto-generate API docs, README, guides
- 🌍 **Multi-Language Support**: Beyond current JS/TS/Python/Rust
- 🔄 **Dependency Management**: Auto-update and security scanning

**Low Priority**:
- 🎮 **Interactive Mode**: Step-through sprint execution with confirmations
- 📈 **Metrics & Analytics**: Track velocity, success rates, common failures
- 🔌 **Plugin System**: Community-contributed agents and skills
- 🌐 **Remote Execution**: Run sprints on cloud workers
- 🤝 **Team Collaboration**: Multi-developer coordination
- 🎯 **Learning Mode**: Improve agent prompts based on outcomes

### Known Issues & Improvements

**Quality of Life**:
- Add `autoflow doctor` diagnostic command
- Better error messages with actionable suggestions
- Automatic cleanup of failed sprints/worktrees
- Git commit message templates based on sprint context
- Configurable max parallel sprints based on system resources

**Performance**:
- Parallel doc generation (currently sequential)
- Caching of LLM responses for identical contexts
- Incremental sprint updates (don't regenerate unchanged tasks)
- Lazy loading of large files during analysis

**Reliability**:
- Retry logic with exponential backoff for agent failures
- Automatic rollback on critical errors
- Better handling of network issues
- Validation of generated YAML before saving

**Developer Experience**:
- VS Code extension for sprint visualization
- GitHub Actions integration for CI/CD
- Docker image for easy distribution
- Homebrew formula for macOS
- Snap package for Linux

See [CHANGELOG.md](CHANGELOG.md) for completed improvements and [GitHub Issues](https://github.com/ddunford/autoflow/issues) to track or suggest new features.

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
