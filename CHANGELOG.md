# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.7] - 2025-11-12

### Added
- **Auto-Update System**: Complete binary, agent, and skill auto-update via GitHub releases
- **Version Display**: Shows version number on `autoflow start` command
- **Cargo Install Support**: Published to crates.io for easy installation
- **Manual Update Command**: `autoflow update` to check and install updates manually
- **Update Notifications**: Checks for updates every 24h automatically

### Fixed
- Crates.io packaging with schema files properly included
- All workspace dependencies with explicit version specifications
- Binary update system with atomic replacement and backup
- Make-sprints agent output token limit (switched to Write tool)
- Blocker-resolver workflow-specific status validation

### Infrastructure
- GitHub Actions release workflow with multi-platform support
- Crates.io publishing in correct dependency order
- Schema files copied into autoflow-data crate
- Clean git history with single comprehensive commit

## [0.1.0] - 2025-11-08

### Added
- Sprint-based autonomous development from IDEA.md to production code
- 13 production agents for docs, sprints, code, tests, and review
- Three workflow types: IMPLEMENTATION, INFRASTRUCTURE, DOCUMENTATION
- Multi-agent orchestration system with TDD pipeline
- Quality gates (unit tests, integration tests, E2E tests, code review)
- Git worktree management for parallel development
- Docker environment support
- MCP server integration (memory, playwright, github, postgres)
- Live log streaming in JSONL format
- Automatic sprint planning and task breakdown
- Autonomous bug fixing with investigation
- Feature addition to existing codebases
- Documentation pivot/refinement command

### Infrastructure
- Rust workspace with 7 crates (cli, core, agents, quality, data, git, utils)
- 15 CLI commands for project management
- Claude CLI integration for agent execution
- YAML-based sprint configuration with JSON schema validation
- Comprehensive logging and debugging

[0.1.7]: https://github.com/ddunford/autoflow/releases/tag/v0.1.7
[0.1.0]: https://github.com/ddunford/autoflow/releases/tag/v0.1.0
