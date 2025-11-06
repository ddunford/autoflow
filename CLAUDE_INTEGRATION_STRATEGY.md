# AutoFlow Claude Code Integration Strategy

**Date**: 2025-11-05
**Purpose**: Define how AutoFlow reuses Claude Code features (agents, skills, plugins) and handles installation, versioning, and updates

---

## Table of Contents

1. [Claude Code Feature Usage](#1-claude-code-feature-usage)
2. [Installation Strategy](#2-installation-strategy)
3. [Directory Structure](#3-directory-structure)
4. [Versioning & Updates](#4-versioning--updates)
5. [Plugin Distribution](#5-plugin-distribution)
6. [Migration from Current Setup](#6-migration-from-current-setup)

---

## 1. Claude Code Feature Usage

### 1.1 What We Reuse from Claude Code

**Agents** (`~/.claude/agents/` and `.claude/agents/`)
- 25 existing agent definitions (code-implementer, reviewer, test-writer, etc.)
- Frontmatter configuration (tools, model, description)
- Automatic delegation based on sprint type
- Tool restrictions for security

**Skills** (`~/.claude/skills/`)
- 13 existing skills for E2E test debugging
- Framework integration checks (react-vite-integration, laravel-react-integration)
- Test diagnostic patterns (playwright-wait-strategies, async-race-conditions)
- Auto-invoked by Claude when relevant

**MCP Servers** (`.mcp.json`)
- memory - Persistent knowledge graph
- context7 - Up-to-date framework documentation
- serena - Semantic code intelligence
- sequential-thinking - Deep analysis

**Hooks** (`.claude/hooks/`)
- Validation hooks (schema validation, output format checking)
- Quality hooks (code review, test execution)
- Security hooks (credential scanning, OWASP checks)

**Settings** (`.claude/settings.json`)
- Model configuration
- Tool permissions
- Sensitive file exclusions
- Hook configuration

### 1.2 What AutoFlow Rust Adds

**Orchestration Layer**:
- 12-phase TDD state machine
- Sprint-level execution (batch processing)
- Git worktree management
- Parallel sprint execution
- Rollback mechanism
- Quality gate pipeline

**Data Management**:
- Type-safe SPRINTS.yml parsing
- Schema validation
- Phase tracking
- Metrics collection

**CLI Commands**:
```bash
autoflow init          # Initialize project with AutoFlow
autoflow start         # Start autonomous development
autoflow status        # Show sprint progress
autoflow rollback      # Rollback to previous state
autoflow update        # Update AutoFlow components
```

---

## 2. Installation Strategy

### 2.1 Recommended: Dual-Layer Installation

**Global Layer** (`~/.claude/`):
- Core AutoFlow agents (25 agents)
- Core AutoFlow skills (13 skills)
- Shared across all projects
- Updated via `autoflow update`

**Project Layer** (`.claude/`):
- Project-specific agents (optional)
- Project-specific skills (optional)
- Custom hooks
- MCP server configuration
- Settings overrides

**Rationale**:
- **Agents**: Install globally (reused across projects, version controlled in AutoFlow repo)
- **Skills**: Install globally (diagnostic skills apply to any project)
- **Settings**: Project-specific (each project has unique config)
- **Hooks**: Mix (validation hooks global, project hooks project-specific)

### 2.2 Installation Command

```bash
# Install AutoFlow globally
autoflow install

# What it does:
# 1. Copy agents to ~/.claude/agents/
# 2. Copy skills to ~/.claude/skills/
# 3. Install global hooks to ~/.claude/hooks/
# 4. Set up global MCP servers in ~/.claude/.mcp.json
# 5. Add AutoFlow CLI to PATH
```

```bash
# Initialize project with AutoFlow
cd /path/to/project
autoflow init

# What it does:
# 1. Create .claude/ directory
# 2. Generate .claude/settings.json
# 3. Create .autoflow/ directory structure
# 4. Initialize SPRINTS.yml template
# 5. Set up project-specific MCP servers (serena, filesystem)
# 6. Configure worktree settings
```

### 2.3 Installation Flow

```
┌─────────────────────────────────────────────────────────┐
│  autoflow install (one-time global setup)              │
└──────────────────┬──────────────────────────────────────┘
                   │
        ┌──────────▼──────────┐
        │ ~/.claude/          │
        │  ├── agents/        │ ← 25 AutoFlow agents
        │  ├── skills/        │ ← 13 AutoFlow skills
        │  ├── hooks/         │ ← Validation/quality hooks
        │  └── .mcp.json      │ ← memory, context7, sequential-thinking
        └──────────┬──────────┘
                   │
        ┌──────────▼──────────┐
        │ /usr/local/bin/     │
        │  └── autoflow       │ ← Rust binary
        └─────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  autoflow init (per-project setup)                      │
└──────────────────┬────────────────────────────────────┘
                   │
        ┌──────────▼──────────┐
        │ .claude/            │
        │  ├── settings.json  │ ← Project config
        │  ├── .mcp.json      │ ← serena, filesystem (project-scoped)
        │  └── CLAUDE.md      │ ← Project instructions
        └──────────┬──────────┘
                   │
        ┌──────────▼──────────┐
        │ .autoflow/          │
        │  ├── SPRINTS.yml    │ ← Sprint definitions
        │  ├── docs/          │ ← Generated documentation
        │  ├── phase-1/       │ ← Phase-specific work
        │  └── schemas/       │ ← JSON schemas
        └─────────────────────┘
```

---

## 3. Directory Structure

### 3.1 Global AutoFlow Installation

```
~/.claude/
├── agents/                         # AutoFlow agents (global)
│   ├── code-implementer.agent.md
│   ├── reviewer.agent.md
│   ├── test-writer.agent.md
│   ├── e2e-writer.agent.md
│   ├── unit-fixer.agent.md
│   ├── e2e-fixer.agent.md
│   ├── review-fixer.agent.md
│   ├── debug-blocker.agent.md
│   ├── health-check.agent.md
│   ├── health-check-fixer.agent.md
│   ├── make-docs.agent.md
│   ├── make-sprints.agent.md
│   ├── review-sprints.agent.md
│   ├── link-sprint-docs.agent.md
│   ├── frontend-react.agent.md
│   ├── backend-laravel.agent.md
│   ├── devops-setup.agent.md
│   └── autoflow-learn.agent.md
│
├── skills/                         # AutoFlow skills (global)
│   ├── react-vite-integration/
│   │   ├── SKILL.md
│   │   └── .gitignore
│   ├── laravel-react-integration/
│   ├── playwright-wait-strategies/
│   ├── react-state-timing/
│   ├── async-race-conditions/
│   ├── playwright-pointer-interception/
│   ├── frontend-integration-check/
│   ├── e2e-task-validation/
│   ├── sprint-validation/
│   ├── tailwind-v4-setup/
│   ├── vue-vite-integration/
│   └── vue-reactivity-timing/
│
├── hooks/                          # AutoFlow validation hooks (global)
│   ├── validation/
│   │   ├── schema_validator.py
│   │   ├── output_format_validator.py
│   │   └── yaml_to_markdown_converter.py
│   ├── quality/
│   │   ├── code_review_checker.py
│   │   └── test_coverage_checker.py
│   ├── security/
│   │   ├── credential_scanner.py
│   │   └── owasp_checker.py
│   └── hooks.json
│
├── .mcp.json                       # Global MCP servers
│   # memory, context7, sequential-thinking
│
└── autoflow/                       # AutoFlow reference materials
    ├── reference/
    │   ├── STANDARDS.md
    │   ├── CODE_REVIEW_GUIDE.md
    │   └── TEST_ERROR_PATTERNS.md
    ├── instructions/
    │   ├── core-standards.md
    │   └── path-resolution.md
    ├── schemas/
    │   ├── sprints.schema.json
    │   ├── code_review_results.schema.json
    │   └── test_results.schema.json
    └── templates/
        ├── SPRINTS.template.yml
        └── CLAUDE.template.md
```

### 3.2 Project-Specific Configuration

```
<project>/
├── .claude/
│   ├── settings.json               # Project-specific settings
│   │   # - Model preferences
│   │   # - Tool permissions
│   │   # - Sensitive file exclusions
│   │   # - Hook configuration
│   │
│   ├── .mcp.json                   # Project-scoped MCP servers
│   │   # - serena (code intelligence)
│   │   # - filesystem (project files only)
│   │   # - git (project repo only)
│   │
│   ├── CLAUDE.md                   # Project instructions
│   │   # - CODE_ROOT, TEST_ROOT, CONFIG_ROOT
│   │   # - Project-specific conventions
│   │   # - Framework guidelines
│   │
│   ├── agents/                     # Project-specific agents (optional)
│   │   └── custom-validator.agent.md
│   │
│   ├── skills/                     # Project-specific skills (optional)
│   │   └── domain-specific-checks/
│   │
│   └── hooks/                      # Project-specific hooks (optional)
│       └── hooks.json
│
└── .autoflow/
    ├── SPRINTS.yml                 # Sprint definitions
    ├── docs/                       # Generated design docs
    │   ├── PRODUCT.md
    │   ├── FUNCTIONAL.md
    │   ├── ARCHITECTURE.md
    │   ├── API.md
    │   ├── DATABASE.md
    │   ├── SECURITY.md
    │   └── TESTING.md
    ├── phase-1/                    # Phase-specific work
    │   ├── sprints/
    │   │   ├── sprint-1/
    │   │   │   ├── CODE_REVIEW_RESULTS.yml
    │   │   │   ├── TEST_RESULTS.yml
    │   │   │   └── logs/
    │   │   └── sprint-2/
    │   └── CURRENT_PHASE
    └── schemas/                    # Project can override schemas
        └── custom_task.schema.json
```

---

## 4. Versioning & Updates

### 4.1 Semantic Versioning

AutoFlow follows semantic versioning for all components:

```toml
# AutoFlow package metadata
[package]
name = "autoflow"
version = "0.1.0"  # MAJOR.MINOR.PATCH

[components]
agents_version = "1.2.0"
skills_version = "1.1.0"
schemas_version = "1.0.1"
```

**Version Compatibility**:
- **MAJOR**: Breaking changes (agent frontmatter format, schema structure)
- **MINOR**: New features (new agents, new skills, enhanced capabilities)
- **PATCH**: Bug fixes (agent instruction improvements, schema fixes)

### 4.2 Update Strategy

```bash
# Check for updates
autoflow update --check

# Output:
# AutoFlow CLI:    0.1.0 → 0.2.0 (new features available)
# Agents:          1.2.0 → 1.3.0 (new debug-blocker improvements)
# Skills:          1.1.0 → 1.1.1 (playwright-wait-strategies fix)
# Schemas:         1.0.1 (up to date)

# Update all components
autoflow update

# Update specific component
autoflow update agents
autoflow update skills
```

### 4.3 Update Implementation

```rust
// src/cli/commands/update.rs
pub async fn run_update(component: Option<String>) -> Result<()> {
    let current_versions = get_installed_versions()?;
    let latest_versions = fetch_latest_versions().await?;

    let updates = match component.as_deref() {
        Some("agents") => vec![Component::Agents],
        Some("skills") => vec![Component::Skills],
        Some("schemas") => vec![Component::Schemas],
        None => check_all_updates(&current_versions, &latest_versions),
        _ => return Err(AutoFlowError::InvalidComponent(component.unwrap())),
    };

    for component in updates {
        info!("Updating {}...", component);

        match component {
            Component::Agents => {
                backup_directory("~/.claude/agents")?;
                download_and_install_agents(&latest_versions.agents).await?;
            }
            Component::Skills => {
                backup_directory("~/.claude/skills")?;
                download_and_install_skills(&latest_versions.skills).await?;
            }
            Component::Schemas => {
                backup_directory("~/.claude/autoflow/schemas")?;
                download_and_install_schemas(&latest_versions.schemas).await?;
            }
            Component::Cli => {
                self_update::update_binary().await?;
            }
        }

        success!("{} updated successfully", component);
    }

    Ok(())
}
```

### 4.4 Version Manifest

AutoFlow maintains a version manifest:

```json
// ~/.claude/autoflow/manifest.json
{
  "autoflow_version": "0.1.0",
  "installed_at": "2025-11-05T12:00:00Z",
  "last_updated": "2025-11-05T12:00:00Z",
  "components": {
    "agents": {
      "version": "1.2.0",
      "checksum": "sha256:abc123...",
      "files": [
        "code-implementer.agent.md",
        "reviewer.agent.md",
        "test-writer.agent.md"
      ]
    },
    "skills": {
      "version": "1.1.0",
      "checksum": "sha256:def456...",
      "directories": [
        "react-vite-integration",
        "playwright-wait-strategies"
      ]
    },
    "schemas": {
      "version": "1.0.1",
      "checksum": "sha256:ghi789...",
      "files": [
        "sprints.schema.json",
        "code_review_results.schema.json"
      ]
    }
  },
  "update_channel": "stable"
}
```

### 4.5 Rollback Support

```bash
# Rollback to previous version
autoflow update rollback

# Output:
# Rolling back to AutoFlow 0.1.0...
# - Agents: 1.3.0 → 1.2.0
# - Skills: 1.1.1 → 1.1.0
# Rollback complete!

# List available rollback points
autoflow update history
```

**Implementation**:
```rust
pub async fn rollback_update() -> Result<()> {
    let backup_dir = PathBuf::from("~/.claude/autoflow/.backups");
    let backups = list_backups(&backup_dir)?;

    if backups.is_empty() {
        return Err(AutoFlowError::NoBackupsFound);
    }

    let latest_backup = backups.first().unwrap();
    info!("Rolling back to backup from {}", latest_backup.timestamp);

    restore_from_backup(latest_backup)?;

    Ok(())
}
```

---

## 5. Plugin Distribution

### 5.1 AutoFlow as a Claude Code Plugin

Package AutoFlow as an official Claude Code plugin for easy distribution:

```
autoflow-plugin/
├── .claude-plugin/
│   └── plugin.json                 # Plugin manifest
├── agents/                         # All 25 agents
├── skills/                         # All 13 skills
├── hooks/                          # Validation hooks
│   └── hooks.json
├── .mcp.json                       # MCP server definitions
├── README.md
└── CHANGELOG.md
```

**plugin.json**:
```json
{
  "name": "autoflow",
  "version": "1.0.0",
  "description": "Autonomous TDD coding agent with sprint-driven development",
  "author": "AutoFlow Team",
  "homepage": "https://github.com/autoflow/autoflow",
  "license": "MIT",
  "engines": {
    "claude-code": ">=1.0.0"
  },
  "dependencies": {
    "mcp-memory": "^1.0.0",
    "mcp-context7": "^1.0.0",
    "mcp-serena": "^1.0.0"
  },
  "keywords": ["tdd", "autonomous", "sprints", "agents", "testing"]
}
```

### 5.2 Installation via Plugin System

Users can install AutoFlow via Claude Code's plugin system:

```bash
# Install from marketplace
/plugin install autoflow

# Or install from git repository
/plugin install git+https://github.com/autoflow/autoflow-plugin

# Enable the plugin
/plugin enable autoflow
```

### 5.3 Plugin Marketplace

Create an AutoFlow marketplace for extensions:

```bash
# Install AutoFlow marketplace
/marketplace add autoflow https://github.com/autoflow/marketplace

# Browse available AutoFlow extensions
/marketplace list autoflow

# Example extensions:
# - autoflow-golang: Go-specific agents and skills
# - autoflow-python: Python/Django agents
# - autoflow-kubernetes: DevOps agents for K8s
# - autoflow-mobile: React Native / Flutter agents
```

### 5.4 Custom Agent/Skill Distribution

Teams can distribute custom agents/skills:

**Option 1: Plugin Bundle**
```bash
# Create custom plugin
autoflow plugin create my-company-agents

# Package agents/skills
autoflow plugin add-agent custom-validator.agent.md
autoflow plugin add-skill company-standards/

# Publish to private repository
autoflow plugin publish git+https://github.com/mycompany/autoflow-agents
```

**Option 2: Git Submodule**
```bash
# Add custom agents as submodule
cd <project>/.claude
git submodule add https://github.com/mycompany/agents agents/
git submodule add https://github.com/mycompany/skills skills/
```

**Option 3: Direct Copy**
```bash
# Copy agents to project
cp -r ~/company-agents/* .claude/agents/
```

---

## 6. Migration from Current Setup

### 6.1 Current State Analysis

**Current Installation**:
- AutoFlow lives in `/home/dan/.claude/autoflow/`
- Agents in `/home/dan/.claude/agents/` (25 agents)
- Skills in `/home/dan/.claude/skills/` (13 skills)
- Bash scripts in `/home/dan/.claude/autoflow/scripts/`
- Reference materials in `/home/dan/.claude/autoflow/reference/`

**Current Workflow**:
```bash
# Current: Bash-based
cd /home/dan/.claude/autoflow
./scripts/autoflow.sh start
```

### 6.2 Migration Path

**Phase 1: Parallel Installation (Week 1)**
```bash
# Install Rust AutoFlow alongside Bash version
cargo install autoflow --git https://github.com/autoflow/autoflow-rust

# Both versions coexist:
autoflow-bash start    # Old Bash orchestrator
autoflow start         # New Rust orchestrator
```

**Phase 2: Gradual Adoption (Weeks 2-4)**
```bash
# Opt-in to Rust version per project
cd <project>
autoflow init --rust

# Or stick with Bash
autoflow init --bash
```

**Phase 3: Full Migration (Week 5+)**
```bash
# Rust version becomes default
autoflow start         # Uses Rust

# Legacy mode still available
autoflow start --legacy
```

**Phase 4: Deprecation (Month 3+)**
```bash
# Remove Bash scripts
rm -rf /home/dan/.claude/autoflow/scripts/

# Only Rust binary remains
which autoflow
# /usr/local/bin/autoflow
```

### 6.3 Backward Compatibility

**Data Format**:
- SPRINTS.yml format unchanged
- Schema validation ensures compatibility
- Agent frontmatter format unchanged
- Skill structure unchanged

**Agent Definitions**:
- All 25 agent markdown files reused as-is
- No changes required to agent content
- Frontmatter parsing compatible

**Skills**:
- All 13 skills work without changes
- SKILL.md format unchanged
- Auto-invocation logic compatible

**MCP Servers**:
- Same MCP servers (memory, context7, serena)
- Same `.mcp.json` format
- No reconfiguration needed

### 6.4 Migration Script

```rust
// src/cli/commands/migrate.rs
pub async fn migrate_from_bash() -> Result<()> {
    info!("Migrating from Bash AutoFlow to Rust AutoFlow...");

    // 1. Detect Bash installation
    let bash_dir = PathBuf::from("~/.claude/autoflow");
    if !bash_dir.exists() {
        return Err(AutoFlowError::NoBashInstallation);
    }

    // 2. Backup current state
    backup_directory(&bash_dir)?;

    // 3. Verify agents (should already be in place)
    let agents_dir = PathBuf::from("~/.claude/agents");
    verify_agents(&agents_dir)?;

    // 4. Verify skills (should already be in place)
    let skills_dir = PathBuf::from("~/.claude/skills");
    verify_skills(&skills_dir)?;

    // 5. Copy reference materials
    copy_reference_materials(&bash_dir)?;

    // 6. Copy schemas
    copy_schemas(&bash_dir)?;

    // 7. Update manifest
    create_manifest()?;

    // 8. Test Rust orchestrator
    test_orchestrator().await?;

    success!("Migration complete! You can now use 'autoflow start'");
    info!("Your Bash installation is backed up at: {}", bash_dir.join(".backup"));

    Ok(())
}
```

---

## 7. Recommended Final Structure

### 7.1 Global Installation

```
~/.claude/
├── agents/                         # ← AutoFlow agents (version controlled)
│   └── *.agent.md                 # 25 agents
├── skills/                         # ← AutoFlow skills (version controlled)
│   └── */SKILL.md                 # 13 skills
├── hooks/                          # ← AutoFlow validation hooks
│   └── hooks.json
├── autoflow/                       # ← AutoFlow reference materials
│   ├── reference/
│   ├── instructions/
│   ├── schemas/
│   ├── templates/
│   └── manifest.json              # ← Version tracking
└── .mcp.json                       # ← Global MCP servers

/usr/local/bin/
└── autoflow                        # ← Rust binary
```

### 7.2 Per-Project Structure

```
<project>/
├── .claude/
│   ├── settings.json               # Project settings
│   ├── .mcp.json                   # Project-scoped MCP
│   └── CLAUDE.md                   # Project instructions
└── .autoflow/
    ├── SPRINTS.yml                 # Sprint definitions
    ├── docs/                       # Generated docs
    └── phase-N/                    # Phase work
```

---

## 8. Implementation Checklist

### Core Installation
- [ ] Rust binary installation (`autoflow install`)
- [ ] Agent installation to `~/.claude/agents/`
- [ ] Skill installation to `~/.claude/skills/`
- [ ] Hook installation to `~/.claude/hooks/`
- [ ] Reference material setup in `~/.claude/autoflow/`
- [ ] Manifest creation (`~/.claude/autoflow/manifest.json`)

### Project Initialization
- [ ] `autoflow init` command
- [ ] `.claude/settings.json` generation
- [ ] `.autoflow/` directory structure
- [ ] SPRINTS.yml template
- [ ] MCP server configuration

### Update System
- [ ] Version checking (`autoflow update --check`)
- [ ] Component updates (agents, skills, schemas)
- [ ] Backup before update
- [ ] Rollback support
- [ ] Update manifest tracking

### Plugin System Integration
- [ ] Plugin manifest creation
- [ ] Claude Code marketplace listing
- [ ] Installation via `/plugin install autoflow`
- [ ] Custom plugin creation tools

### Migration
- [ ] Detection of existing Bash installation
- [ ] Backup existing configuration
- [ ] Verification of agents/skills
- [ ] Parallel installation support
- [ ] Deprecation timeline

---

## 9. Decision Summary

### Installation Location: **Dual-Layer (Global + Project)**

**Global** (`~/.claude/`):
- ✅ Agents (shared, version controlled by AutoFlow)
- ✅ Skills (shared, diagnostic skills apply everywhere)
- ✅ Hooks (validation logic shared)
- ✅ Reference materials (STANDARDS.md, CODE_REVIEW_GUIDE.md)
- ✅ Schemas (JSON schemas for validation)

**Project** (`.claude/`):
- ✅ Settings (project-specific configuration)
- ✅ MCP servers (project-scoped: serena, filesystem)
- ✅ Custom agents (optional, project-specific logic)
- ✅ Custom skills (optional, domain-specific checks)

**Rationale**:
- Reduces duplication (agents/skills shared across projects)
- Centralized updates (update once, all projects benefit)
- Project customization (override with `.claude/agents/` if needed)
- Version control friendly (no large binary files in project repos)

### Update Strategy: **Semantic Versioning with Rollback**

- AutoFlow CLI, agents, skills, and schemas independently versioned
- `autoflow update` checks for updates and installs
- Automatic backups before updates
- `autoflow update rollback` for safety
- Manifest tracking in `~/.claude/autoflow/manifest.json`

### Distribution: **Native Binary + Claude Code Plugin**

- Primary: Rust binary (`cargo install autoflow`)
- Secondary: Claude Code plugin (`/plugin install autoflow`)
- Enables both standalone usage and Claude Code integration
- Plugin system for custom agent/skill distribution

---

## 10. Next Steps

1. **Implement `autoflow install` command** (Week 1)
   - Copy agents to `~/.claude/agents/`
   - Copy skills to `~/.claude/skills/`
   - Create manifest

2. **Implement `autoflow init` command** (Week 1)
   - Generate `.claude/settings.json`
   - Create `.autoflow/` structure
   - Configure MCP servers

3. **Implement `autoflow update` command** (Week 2)
   - Version checking
   - Component updates
   - Rollback support

4. **Create plugin package** (Week 3)
   - `plugin.json` manifest
   - Test with `/plugin install`
   - Submit to Claude Code marketplace

5. **Migration tooling** (Week 4)
   - `autoflow migrate` command
   - Backward compatibility testing
   - Documentation

---

**Recommendation**: Proceed with dual-layer installation (global agents/skills, project settings) with semantic versioning and plugin distribution support.
