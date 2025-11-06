# Changelog

All notable changes to AutoFlow will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-06

### Added

#### Core Features
- **Complete TDD Pipeline**: IDEA → Docs → Sprints → Tests → Code → Review → Deploy
- **13 Production Agents**
- **19 Skills** (6 new production-ready skills)
- **Non-invasive installation** with .agent.md suffix

### Commands
- `autoflow init` - Initialize project
- `autoflow create <name> --idea <file>` - Create from IDEA.md
- `autoflow start` - Execute sprint pipeline
- `autoflow status` - Show status
- `autoflow fix` - Fix bugs autonomously

### Fixed
- SPRINTS.yml now generates valid YAML
- Docs at project root (not .autoflow/docs/)
- Skill directory structure support

## [Unreleased]

### Planned
- `autoflow update` CLI command
- Self-updating binary
- Plugin marketplace
