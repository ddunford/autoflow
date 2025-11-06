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

## [0.1.3] - 2025-01-06

### Added

#### Pivot Command - Refine Documentation
- **`autoflow pivot` command**: Update documentation and regenerate sprints based on feedback
- **Smart state preservation**: Keeps completed and in-progress sprints when regenerating
- **Full doc rewrite**: Reads all 10 documentation files and updates based on your instruction
- **Sprint regeneration**: Automatically regenerates sprints from updated docs
- **Use cases**: Fix missed requirements, adjust architecture, change API design, update database schema

#### Auto-Update System
- **Automatic Update Checks**: Checks for agent/skill updates on `autoflow start` and `autoflow init`
- **Smart Update Detection**: Compares installed agents/skills with templates based on modification time
- **User-Friendly Prompts**: Shows what's updated/new with [Y/n/skip] options
- **24-Hour Interval**: Checks once per day by default to avoid interruptions
- **Configuration**: `[updates]` section in config.toml to control behavior
- **Non-Intrusive**: Silent on failures, skips gracefully if no updates

#### Enhanced Documentation System
- **6 New Documentation Types**:
  - `DATA_MODEL.md` - Complete database schema, relationships, indexes
  - `TESTING_STRATEGY.md` - Testing approach, coverage requirements, patterns
  - `ERROR_HANDLING.md` - Error codes, response formats, logging strategy
  - `STATE_MANAGEMENT.md` - Frontend state patterns (conditional)
  - `SECURITY.md` - Auth/security implementation (conditional)
  - `DEPLOYMENT.md` - Deployment, CI/CD, infrastructure
- **Conditional Generation**: Docs generated based on project type detection
- **Cross-referencing**: Sprint tasks now link to specific doc sections

#### Agent Execution Logging
- **Real-time Feedback**: Filtered console output showing important actions
- **JSON Logs**: Structured logs at `.autoflow/sprints/sprint-XXX/logs/*.json`
- **Text Logs**: Human-readable logs at `.autoflow/sprints/sprint-XXX/logs/*.log`
- **Debug Mode**: Set `AUTOFLOW_DEBUG=1` for full output visibility
- **Sprint-centric**: All logs organized by sprint for easy debugging

#### Optimized Model Usage
- **Opus 4** for complex reasoning (5 agents):
  - `debug-blocker`, `reviewer`, `review-fixer`, `unit-fixer`, `e2e-fixer`
- **Sonnet 4.5** for standard tasks (8 agents):
  - `make-docs`, `make-sprints`, `code-implementer`, test writers/runners
- **Cost Efficiency**: 60% of agent calls use faster, cheaper Sonnet
- **Quality**: Opus handles hard problems requiring deep reasoning

### Improved
- Documentation is now comprehensive and cross-referenced
- Sprint tasks link to relevant documentation sections
- Agent output is now visible and debuggable
- Better cost/quality balance with optimized model selection

## [Unreleased]

### Planned
- `autoflow update` CLI command
- Self-updating binary
- Plugin marketplace
