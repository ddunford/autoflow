# AutoFlow - Best-in-Class Autonomous Coding Agent

🚀 **Status**: Early Development (v0.1.0)

AutoFlow is a fully autonomous TDD-driven coding agent that takes you from requirements to production-ready code with zero manual intervention.

## Features

- ✅ **Fully Autonomous**: From `BUILD_SPEC.md` to running application
- 🧪 **TDD Pipeline**: Automated test-first development (RED → GREEN → REFACTOR)
- 🌲 **Git Worktree Isolation**: Each sprint/bugfix in isolated workspace
- 🐛 **Autonomous Bug Fixing**: Investigate, reproduce (Playwright MCP), fix, test
- 🔍 **Code-Aware**: Analyzes existing codebases and integrates seamlessly
- 🐳 **Environment Setup**: Automatic Docker, databases, services configuration
- 🎯 **Quality Gates**: Multi-layer validation to catch LLM mistakes
- 📊 **Observable**: Structured logging, metrics, progress tracking

## Quick Start

### Prerequisites

- Rust 1.70+ (for building from source)
- Docker & Docker Compose (for development environments)
- Claude Code CLI installed

### Installation

```bash
# Clone repository
git clone https://github.com/autoflow/autoflow
cd autoflow

# Build from source
cargo build --release

# Install (coming soon)
cargo install --path crates/autoflow-cli
```

### Usage

```bash
# Initialize new project
autoflow init

# Start autonomous development
autoflow start

# Add feature to existing codebase
autoflow add "Add payment processing"

# Fix a bug
autoflow fix "Login button doesn't work on mobile"

# Check status
autoflow status

# Manage worktrees
autoflow worktree list
```

## Architecture

```
autoflow/
├── crates/
│   ├── autoflow-cli/       # CLI application
│   ├── autoflow-core/      # Orchestration & state machine
│   ├── autoflow-agents/    # Agent management
│   ├── autoflow-quality/   # Quality gates
│   ├── autoflow-data/      # Data structures
│   ├── autoflow-git/       # Git worktree operations
│   └── autoflow-utils/     # Utilities
├── agents/                 # Agent definitions (25+)
├── skills/                 # Skill definitions (13+)
├── reference/              # Standards & guides
├── schemas/                # JSON schemas
└── templates/              # Project templates
```

## Documentation

- [Architecture](ARCHITECTURE.md) - System design and components
- [Rebuild Plan](REBUILD_PLAN.md) - Technology decisions and roadmap
- [Feature Workflow](FEATURE_WORKFLOW.md) - Adding features to existing code
- [Bug Fix Workflow](BUG_FIX_WORKFLOW.md) - Autonomous bug fixing
- [Environment Setup](ENVIRONMENT_SETUP.md) - Infrastructure automation

## Development

```bash
# Run tests
cargo test --all

# Build debug version
cargo build

# Run from source
cargo run -- --help

# Watch mode (requires cargo-watch)
cargo watch -x 'run -- status'
```

## Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific crate
cargo test -p autoflow-data
```

## Commands

### Project Management
- `autoflow init` - Initialize new project
- `autoflow status` - Show sprint progress
- `autoflow analyze` - Analyze existing codebase

### Development
- `autoflow start` - Start autonomous development
- `autoflow start --parallel` - Execute sprints in parallel
- `autoflow start --sprint=5` - Run specific sprint

### Features & Bugs
- `autoflow add "description"` - Add new feature
- `autoflow fix "description"` - Fix bug autonomously

### Worktrees
- `autoflow worktree list` - List all worktrees
- `autoflow worktree merge <branch>` - Merge worktree
- `autoflow worktree delete <branch>` - Delete worktree

### Environment
- `autoflow env start` - Start Docker containers
- `autoflow env stop` - Stop containers
- `autoflow env health` - Check health

### Quality
- `autoflow validate --infrastructure` - Check infrastructure
- `autoflow validate --integration` - Check integration
- `autoflow validate --fix` - Auto-fix issues

## Roadmap

### Phase 1: Foundation (Weeks 1-2) ✅
- [x] Cargo workspace setup
- [x] Core data structures
- [x] CLI skeleton
- [ ] Install script
- [ ] Agent directory setup

### Phase 2: Orchestrator (Weeks 3-4)
- [ ] State machine implementation
- [ ] Phase transitions
- [ ] Agent executor
- [ ] Sprint runner

### Phase 3: Quality Gates (Weeks 5-6)
- [ ] Schema validation
- [ ] Format validation
- [ ] Blocker detection
- [ ] Auto-fix logic

### Phase 4: Git & Worktrees (Weeks 7-8)
- [ ] Worktree manager
- [ ] Branch operations
- [ ] Merge logic
- [ ] Rollback support

### Phase 5: Polish (Weeks 9-10)
- [ ] Progress bars
- [ ] Better error messages
- [ ] Documentation
- [ ] Examples

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Claude Code](https://claude.com/claude-code)
- Inspired by TDD best practices

---

**Built with ❤️ by the AutoFlow team**
