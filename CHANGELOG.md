# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Automatic agent and skill syncing to `~/.claude/` on every command
- Integration test runner with failure report generation
- E2E test runner with failure report generation
- Review agent with code review failure reports
- Failure reports now written to `.autoflow/.failures/` for structured debugging
- Infrastructure workflow support (Docker, Kubernetes, databases)
- Live streaming logs in JSONL format
- Git commit automation after sprint completion

### Changed
- Agents now loaded with priority: `./agents/` → `~/.claude/agents/`
- Test runners now create consolidated failure reports instead of verbose logs
- Fixer agents read from `.autoflow/.failures/` for focused debugging

### Fixed
- Agent updates now sync automatically without manual reinstall
- Tool access properly passed to claude CLI via `--allowedTools`

## [0.1.0] - TBD

### Added
- Initial release
- Sprint-based autonomous development
- Three workflow types: Implementation, Infrastructure, Full-Stack
- Multi-agent orchestration system
- Quality gates (unit tests, integration tests, E2E tests, code review)
- Git worktree management for parallel development
- Docker environment support
- MCP server integration
- Live log streaming
- Automatic sprint planning and task breakdown

### Infrastructure
- Rust-based CLI with async runtime
- Claude CLI integration for agent execution
- YAML-based sprint configuration
- JSON schema validation
- Comprehensive logging and debugging

[Unreleased]: https://github.com/autoflow/autoflow/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/autoflow/autoflow/releases/tag/v0.1.0
