# AutoFlow V2 Architecture - Best-in-Class Autonomous Coding

**Date**: 2025-11-05
**Purpose**: Design the world's best autonomous coding agent - no backward compatibility constraints
**Build Location**: `/opt/workspaces/autoflow/` (this directory)

---

## Philosophy: Best-in-Class First

**Core Principles**:
1. **Zero backward compatibility** - Design optimal architecture from scratch
2. **Single source of truth** - One canonical installation, project references it
3. **Built here** - Everything developed in `/opt/workspaces/autoflow/`
4. **Type safety first** - Rust with compile-time guarantees
5. **Quality gates everywhere** - Catch LLM mistakes at every layer
6. **Observable by default** - Structured logging, metrics, tracing
7. **Fast by default** - Parallel execution, incremental builds, smart caching

---

## Table of Contents

1. [Installation Architecture](#1-installation-architecture)
2. [Project Structure](#2-project-structure)
3. [Technology Stack](#3-technology-stack)
4. [Core Components](#4-core-components)
5. [Quality Gate System](#5-quality-gate-system)
6. [CLI Design](#6-cli-design)
7. [Development Workflow](#7-development-workflow)
8. [Implementation Plan](#8-implementation-plan)

---

## 1. Installation Architecture

### 1.1 Single Canonical Installation

**Decision**: Install AutoFlow **once globally**, projects reference it

```
~/.autoflow/                        # ← Single installation location
├── bin/
│   └── autoflow                    # Rust binary
├── lib/                            # Core libraries (if needed)
├── agents/                         # 25+ core agents
│   ├── code-implementer.agent.md
│   ├── reviewer.agent.md
│   ├── test-writer.agent.md
│   └── ...
├── skills/                         # 13+ core skills
│   ├── react-vite-integration/
│   ├── playwright-wait-strategies/
│   └── ...
├── reference/                      # Standards & guides
│   ├── STANDARDS.md
│   ├── CODE_REVIEW_GUIDE.md
│   └── TEST_ERROR_PATTERNS.md
├── schemas/                        # JSON schemas
│   ├── sprints.schema.json
│   └── ...
├── templates/                      # Project templates
│   ├── .autoflow/
│   └── .claude/
└── config.toml                     # Global configuration
```

**Projects reference global installation**:
```
<project>/
├── .autoflow/
│   ├── config.toml                 # Links to ~/.autoflow/
│   ├── SPRINTS.yml                 # Project sprints
│   └── docs/                       # Project docs
└── .claude/
    ├── settings.json               # Project Claude config
    └── CLAUDE.md                   # Project instructions
```

### 1.2 Installation Commands

**Build from source** (this directory):
```bash
cd /opt/workspaces/autoflow

# Build Rust binary
cargo build --release

# Install globally
cargo install --path .

# Or run install script
./scripts/install.sh
```

**What `install.sh` does**:
```bash
#!/bin/bash
# scripts/install.sh

# 1. Build Rust binary
cargo build --release

# 2. Create ~/.autoflow/ directory
mkdir -p ~/.autoflow/{bin,agents,skills,reference,schemas,templates}

# 3. Copy binary
cp target/release/autoflow ~/.autoflow/bin/

# 4. Symlink to PATH
ln -sf ~/.autoflow/bin/autoflow /usr/local/bin/autoflow

# 5. Copy agents
cp -r agents/* ~/.autoflow/agents/

# 6. Copy skills
cp -r skills/* ~/.autoflow/skills/

# 7. Copy reference materials
cp -r reference/* ~/.autoflow/reference/

# 8. Copy schemas
cp -r schemas/* ~/.autoflow/schemas/

# 9. Copy templates
cp -r templates/* ~/.autoflow/templates/

# 10. Create global config
cat > ~/.autoflow/config.toml <<EOF
[autoflow]
version = "2.0.0"
install_date = "$(date -Iseconds)"

[paths]
agents_dir = "~/.autoflow/agents"
skills_dir = "~/.autoflow/skills"
reference_dir = "~/.autoflow/reference"
schemas_dir = "~/.autoflow/schemas"

[defaults]
model = "claude-sonnet-4-5-20250929"
max_iterations = 50
parallel_sprints = true
auto_commit = true
EOF

echo "✅ AutoFlow installed to ~/.autoflow/"
echo "✅ Binary available: autoflow"
```

**Setup new project**:
```bash
cd <project>

# Initialize project with AutoFlow
autoflow init

# What it does:
# 1. Create .autoflow/ directory
# 2. Copy templates from ~/.autoflow/templates/
# 3. Create .claude/ directory
# 4. Generate settings.json
# 5. Create SPRINTS.yml template
# 6. Initialize git worktree config
```

### 1.3 Update Strategy

```bash
# Update AutoFlow (rebuild from source)
cd /opt/workspaces/autoflow
git pull
./scripts/install.sh

# All projects automatically use new version
# (they reference ~/.autoflow/)
```

**No per-project updates needed** - single installation means single update point.

---

## 2. Project Structure (This Directory)

```
/opt/workspaces/autoflow/           # ← Build everything here
├── Cargo.toml                      # Rust workspace
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
│
├── crates/                         # Rust crates (modules)
│   ├── autoflow-cli/              # CLI application
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── commands/
│   │           ├── init.rs
│   │           ├── start.rs
│   │           ├── status.rs
│   │           └── ...
│   │
│   ├── autoflow-core/             # Core orchestration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── orchestrator/
│   │       ├── state_machine/
│   │       └── phase_manager/
│   │
│   ├── autoflow-agents/           # Agent management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── executor.rs
│   │       ├── selector.rs
│   │       └── parser.rs
│   │
│   ├── autoflow-quality/          # Quality gates
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── schema_validator.rs
│   │       ├── code_reviewer.rs
│   │       └── blocker_detector.rs
│   │
│   ├── autoflow-data/             # Data structures
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── sprints.rs
│   │       ├── tasks.rs
│   │       └── config.rs
│   │
│   ├── autoflow-git/              # Git operations
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── worktree.rs
│   │       └── operations.rs
│   │
│   └── autoflow-utils/            # Utilities
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── logging.rs
│           └── metrics.rs
│
├── agents/                         # Agent definitions (markdown)
│   ├── code-implementer.agent.md
│   ├── reviewer.agent.md
│   ├── test-writer.agent.md
│   ├── e2e-writer.agent.md
│   ├── unit-fixer.agent.md
│   ├── e2e-fixer.agent.md
│   ├── review-fixer.agent.md
│   ├── debug-blocker.agent.md
│   ├── health-check.agent.md
│   ├── make-docs.agent.md
│   ├── make-sprints.agent.md
│   ├── review-sprints.agent.md
│   ├── link-sprint-docs.agent.md
│   ├── frontend-react.agent.md
│   ├── backend-laravel.agent.md
│   ├── backend-nodejs.agent.md
│   ├── backend-golang.agent.md
│   ├── backend-python.agent.md
│   ├── devops-setup.agent.md
│   ├── database-design.agent.md
│   ├── api-design.agent.md
│   ├── security-audit.agent.md
│   ├── performance-optimizer.agent.md
│   └── autoflow-learn.agent.md
│
├── skills/                         # Skill definitions
│   ├── react-vite-integration/
│   │   ├── SKILL.md
│   │   └── examples/
│   ├── vue-vite-integration/
│   ├── laravel-react-integration/
│   ├── playwright-wait-strategies/
│   ├── playwright-pointer-interception/
│   ├── react-state-timing/
│   ├── vue-reactivity-timing/
│   ├── async-race-conditions/
│   ├── frontend-integration-check/
│   ├── e2e-task-validation/
│   ├── sprint-validation/
│   ├── tailwind-v4-setup/
│   └── typescript-strict-mode/
│
├── reference/                      # Standards & guides
│   ├── STANDARDS.md
│   ├── CODE_REVIEW_GUIDE.md
│   ├── TEST_ERROR_PATTERNS.md
│   ├── DOCUMENTATION_WORKFLOW.md
│   ├── TEST_EXECUTION.md
│   └── agent-context/
│       ├── code-implementer.md
│       ├── reviewer.md
│       └── ...
│
├── schemas/                        # JSON schemas
│   ├── sprints.schema.json
│   ├── task.schema.json
│   ├── code_review_results.schema.json
│   ├── test_results.schema.json
│   ├── e2e_test_results.schema.json
│   └── config.schema.json
│
├── templates/                      # Project templates
│   ├── .autoflow/
│   │   ├── config.toml.template
│   │   └── SPRINTS.yml.template
│   ├── .claude/
│   │   ├── settings.json.template
│   │   └── CLAUDE.md.template
│   └── BUILD_SPEC.md.template
│
├── scripts/                        # Build & install scripts
│   ├── install.sh
│   ├── uninstall.sh
│   ├── dev.sh                     # Local development
│   └── test.sh                    # Run all tests
│
├── tests/                          # Integration tests
│   ├── integration/
│   │   ├── test_full_pipeline.rs
│   │   ├── test_parallel_sprints.rs
│   │   └── test_quality_gates.rs
│   └── fixtures/
│       └── sample-project/
│
├── docs/                           # Documentation
│   ├── ARCHITECTURE.md            # This file
│   ├── REBUILD_PLAN.md
│   ├── CLAUDE_INTEGRATION_STRATEGY.md
│   ├── getting-started.md
│   ├── agent-development.md
│   ├── skill-development.md
│   └── api/                       # Rust API docs
│
└── examples/                       # Example projects
    ├── todo-app/
    ├── blog-platform/
    └── e-commerce/
```

---

## 3. Technology Stack

### 3.1 Core Technologies

**Language**: Rust (2021 edition)
- Type safety at compile time
- Zero-cost abstractions
- Memory safety without GC
- Excellent concurrency (Tokio)

**CLI Framework**: clap v4
- Subcommands (init, start, status, etc.)
- Argument parsing with validation
- Help generation
- Shell completion

**Async Runtime**: Tokio
- Process spawning (Claude Code agents)
- Concurrent sprint execution
- File I/O
- Network requests

**Serialization**: serde
- Zero-copy YAML/JSON parsing
- Automatic validation
- Derive macros

**Schema Validation**: jsonschema
- Validate SPRINTS.yml against schema
- Catch agent output errors
- Runtime validation

**Git**: git2
- Worktree operations
- Branch management
- Commit automation

**Logging**: tracing + tracing-subscriber
- Structured logging
- Spans for request tracing
- JSON output for metrics

**Error Handling**: thiserror + anyhow
- Custom error types (thiserror)
- Context propagation (anyhow)
- Result<T> everywhere

### 3.2 Dependencies

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/autoflow-cli",
    "crates/autoflow-core",
    "crates/autoflow-agents",
    "crates/autoflow-quality",
    "crates/autoflow-data",
    "crates/autoflow-git",
    "crates/autoflow-utils",
]

[workspace.dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# CLI
clap = { version = "4.4", features = ["derive", "color", "suggestions"] }
colored = "2.1"
indicatif = "0.17"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Validation
jsonschema = "0.17"
validator = { version = "0.16", features = ["derive"] }

# Git
git2 = "0.18"

# File operations
walkdir = "2.4"
glob = "0.3"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Configuration
config = "0.13"
toml = "0.8"

# Testing
[dev-dependencies]
proptest = "1.4"
tempfile = "3.8"
```

---

## 4. Core Components

### 4.1 Orchestrator (State Machine)

**Purpose**: Manage 12-phase TDD pipeline with automatic progression

```rust
// crates/autoflow-core/src/orchestrator/mod.rs

pub struct Orchestrator {
    config: Config,
    agent_executor: AgentExecutor,
    quality_pipeline: QualityPipeline,
    worktree_manager: WorktreeManager,
    metrics: MetricsCollector,
}

impl Orchestrator {
    pub async fn run(&self) -> Result<()> {
        // Load SPRINTS.yml
        let mut sprints = self.load_sprints().await?;

        // Get pending sprints
        let pending = sprints.filter_by_status(SprintStatus::Pending);

        if self.config.parallel_sprints {
            // Execute sprints in parallel
            self.run_parallel(pending).await?;
        } else {
            // Execute sprints sequentially
            for sprint in pending {
                self.run_sprint(&mut sprint).await?;
            }
        }

        Ok(())
    }

    async fn run_sprint(&self, sprint: &mut Sprint) -> Result<()> {
        let mut iteration = 0;

        while !sprint.is_done() && iteration < self.config.max_iterations {
            iteration += 1;

            // Log current state
            info!("Sprint {} - Iteration {} - Status: {:?}",
                sprint.id, iteration, sprint.status);

            // Execute current phase
            match self.execute_phase(sprint).await {
                Ok(_) => {
                    // Advance to next phase
                    sprint.advance()?;
                    self.save_sprint(sprint).await?;
                }
                Err(e) => {
                    // Handle errors (retry, block, or fail)
                    self.handle_error(sprint, e).await?;
                }
            }
        }

        Ok(())
    }

    async fn execute_phase(&self, sprint: &Sprint) -> Result<()> {
        match sprint.status {
            SprintStatus::WriteUnitTests => {
                self.agent_executor.run("test-writer", sprint).await?;
                self.quality_pipeline.validate_tests(sprint).await?;
            }
            SprintStatus::WriteCode => {
                self.agent_executor.run("code-implementer", sprint).await?;
                self.quality_pipeline.validate_code(sprint).await?;
            }
            SprintStatus::CodeReview => {
                self.agent_executor.run("reviewer", sprint).await?;
                self.quality_pipeline.check_review(sprint).await?;
            }
            SprintStatus::RunUnitTests => {
                self.run_tests(sprint, TestType::Unit).await?;
            }
            // ... other phases
            _ => {}
        }

        Ok(())
    }
}
```

### 4.2 Agent Executor

**Purpose**: Spawn and manage Claude Code agents with context injection

```rust
// crates/autoflow-agents/src/executor.rs

pub struct AgentExecutor {
    agents_dir: PathBuf,
    config: AgentConfig,
}

impl AgentExecutor {
    pub async fn run(&self, agent_name: &str, sprint: &Sprint) -> Result<AgentOutput> {
        // 1. Load agent definition
        let agent = self.load_agent(agent_name)?;

        // 2. Prepare context (inject docs, business rules, etc.)
        let context = self.prepare_context(sprint, &agent)?;

        // 3. Spawn Claude Code with agent
        let mut child = Command::new("claude")
            .arg("--agent")
            .arg(agent.path)
            .arg("--max-turns")
            .arg(agent.max_turns.to_string())
            .arg("--json-output")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // 4. Write context to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(context.as_bytes()).await?;
            stdin.flush().await?;
        }

        // 5. Monitor output in real-time
        let stdout = child.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout).lines();
        let mut output = AgentOutput::new();

        while let Some(line) = reader.next_line().await? {
            // Parse JSON stream
            if let Ok(event) = serde_json::from_str::<AgentEvent>(&line) {
                // Handle tool calls, validate output
                self.handle_event(&event, &mut output)?;
            }
        }

        // 6. Wait for completion
        let status = child.wait().await?;
        if !status.success() {
            return Err(AutoFlowError::AgentFailed {
                agent: agent_name.to_string(),
                exit_code: status.code(),
            });
        }

        Ok(output)
    }

    fn handle_event(&self, event: &AgentEvent, output: &mut AgentOutput) -> Result<()> {
        match event.event_type {
            EventType::ToolUse => {
                // Validate tool output immediately
                self.validate_tool_output(event)?;
                output.add_tool_use(event);
            }
            EventType::FileWrite => {
                // Run quality gates on written files
                self.run_quality_gates_on_file(event)?;
                output.add_file_write(event);
            }
            EventType::Error => {
                return Err(AutoFlowError::AgentError {
                    message: event.message.clone(),
                });
            }
            _ => {}
        }
        Ok(())
    }
}
```

### 4.3 Quality Gate System

**Purpose**: Multi-layer validation to catch LLM mistakes early

```rust
// crates/autoflow-quality/src/lib.rs

pub struct QualityPipeline {
    gates: Vec<Box<dyn QualityGate>>,
}

impl QualityPipeline {
    pub fn new() -> Self {
        Self {
            gates: vec![
                // Layer 1: Syntax validation
                Box::new(SchemaValidator::new()),
                Box::new(YamlSyntaxValidator::new()),
                Box::new(JsonSyntaxValidator::new()),

                // Layer 2: Format validation
                Box::new(OutputFormatValidator::new()),
                Box::new(MarkdownInYamlDetector::new()),

                // Layer 3: Semantic validation
                Box::new(BusinessRuleValidator::new()),
                Box::new(TestCoverageValidator::new()),
                Box::new(CodeQualityValidator::new()),

                // Layer 4: Integration validation
                Box::new(BackendFrontendSync::new()),
                Box::new(DependencyValidator::new()),
                Box::new(BlockerDetector::new()),

                // Layer 5: Security validation
                Box::new(CredentialScanner::new()),
                Box::new(OwaspValidator::new()),
                Box::new(VulnerabilityScanner::new()),
            ],
        }
    }

    pub async fn validate(&self, sprint: &Sprint) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();

        for gate in &self.gates {
            let span = tracing::info_span!("quality_gate", gate = gate.name());
            let _enter = span.enter();

            let result = gate.check(sprint).await?;
            report.add_result(gate.name(), result);

            // Stop on critical failures
            if result.has_critical_issues() {
                report.status = ValidationStatus::Failed;
                break;
            }
        }

        Ok(report)
    }

    pub async fn auto_fix(&self, sprint: &Sprint) -> Result<FixReport> {
        let mut report = FixReport::new();

        for gate in &self.gates {
            if gate.can_auto_fix() {
                match gate.fix(sprint).await {
                    Ok(fixes) => report.add_fixes(gate.name(), fixes),
                    Err(e) => report.add_failure(gate.name(), e),
                }
            }
        }

        Ok(report)
    }
}
```

### 4.4 Git Worktree Manager

**Purpose**: Isolated sprint workspaces with perfect rollback

```rust
// crates/autoflow-git/src/worktree.rs

pub struct WorktreeManager {
    repo_path: PathBuf,
    worktree_base: PathBuf,
}

impl WorktreeManager {
    pub async fn create_for_sprint(&self, sprint: &Sprint) -> Result<Worktree> {
        let branch_name = format!("sprint-{}", sprint.id);
        let worktree_path = self.worktree_base.join(&branch_name);

        // Create branch
        let repo = Repository::open(&self.repo_path)?;
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        repo.branch(&branch_name, &commit, false)?;

        // Create worktree
        repo.worktree(&branch_name, &worktree_path, None)?;

        // Set up Docker containers for this worktree
        self.setup_docker(&worktree_path).await?;

        // Configure MCP servers for this worktree
        self.configure_mcp(&worktree_path).await?;

        Ok(Worktree {
            branch: branch_name,
            path: worktree_path,
            sprint_id: sprint.id,
        })
    }

    pub async fn merge_and_cleanup(&self, worktree: &Worktree) -> Result<()> {
        let repo = Repository::open(&self.repo_path)?;

        // Merge sprint branch to main
        self.merge_branch(&repo, &worktree.branch)?;

        // Remove worktree
        repo.worktree_prune(&worktree.branch, None)?;

        // Delete branch
        let mut branch = repo.find_branch(&worktree.branch, BranchType::Local)?;
        branch.delete()?;

        // Cleanup Docker containers
        self.cleanup_docker(&worktree.path).await?;

        Ok(())
    }

    pub async fn rollback(&self, sprint_id: u32) -> Result<()> {
        let branch_name = format!("sprint-{}", sprint_id);
        let worktree_path = self.worktree_base.join(&branch_name);

        // Simply delete the worktree and branch
        let repo = Repository::open(&self.repo_path)?;
        repo.worktree_prune(&branch_name, None)?;

        let mut branch = repo.find_branch(&branch_name, BranchType::Local)?;
        branch.delete()?;

        // Perfect rollback - main branch unchanged
        info!("Sprint {} rolled back successfully", sprint_id);

        Ok(())
    }
}
```

---

## 5. Quality Gate System

### 5.1 Multi-Layer Validation

```
┌─────────────────────────────────────────────────────────────┐
│                    Agent Output                             │
│                        ↓                                    │
│  Layer 1: Syntax Validation                                 │
│  - YAML/JSON syntax correct?                                │
│  - Schema compliance?                                       │
│  - Required fields present?                                 │
│                        ↓                                    │
│  Layer 2: Format Validation                                 │
│  - Markdown in YAML? (auto-fix)                            │
│  - Enum values correct?                                     │
│  - File paths absolute?                                     │
│                        ↓                                    │
│  Layer 3: Semantic Validation                               │
│  - Business rules implemented?                              │
│  - Test coverage adequate?                                  │
│  - Code quality standards met?                              │
│                        ↓                                    │
│  Layer 4: Integration Validation                            │
│  - Frontend/backend sync?                                   │
│  - Dependencies exist?                                      │
│  - Blocking issues?                                         │
│                        ↓                                    │
│  Layer 5: Security Validation                               │
│  - Credentials exposed?                                     │
│  - OWASP Top 10 violations?                                 │
│  - Known vulnerabilities?                                   │
│                        ↓                                    │
│  ✅ PASS → Advance to next phase                           │
│  ❌ FAIL → Auto-fix or block                               │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Auto-Fix Capabilities

```rust
// Example: Markdown in YAML auto-fix
pub struct MarkdownInYamlDetector;

impl QualityGate for MarkdownInYamlDetector {
    async fn check(&self, sprint: &Sprint) -> Result<GateResult> {
        let yaml_files = find_yaml_files(&sprint.dir)?;
        let mut issues = vec![];

        for file in yaml_files {
            let content = fs::read_to_string(&file)?;
            if content.contains("```yaml") || content.contains("```yml") {
                issues.push(Issue {
                    severity: Severity::High,
                    file: file.clone(),
                    message: "YAML file contains markdown code blocks".into(),
                    auto_fixable: true,
                });
            }
        }

        Ok(GateResult {
            passed: issues.is_empty(),
            issues,
        })
    }

    async fn fix(&self, sprint: &Sprint) -> Result<Vec<Fix>> {
        let yaml_files = find_yaml_files(&sprint.dir)?;
        let mut fixes = vec![];

        for file in yaml_files {
            let content = fs::read_to_string(&file)?;
            if let Some(extracted) = extract_yaml_from_markdown(&content) {
                fs::write(&file, extracted)?;
                fixes.push(Fix {
                    file: file.clone(),
                    action: "Extracted YAML from markdown code block".into(),
                });
            }
        }

        Ok(fixes)
    }
}
```

---

## 6. CLI Design

### 6.1 Commands

```bash
# Installation & Setup
autoflow install                    # Install to ~/.autoflow/
autoflow init                       # Initialize project
autoflow config                     # Show/edit configuration

# Development
autoflow start                      # Start autonomous development
autoflow start --parallel           # Execute sprints in parallel
autoflow start --sprint=5           # Run specific sprint

# Monitoring
autoflow status                     # Show sprint progress
autoflow logs                       # View orchestrator logs
autoflow logs --sprint=5            # View sprint-specific logs
autoflow metrics                    # Show performance metrics

# Quality Gates
autoflow validate                   # Run quality gates
autoflow validate --fix             # Run with auto-fix
autoflow review                     # Manual code review

# Git Operations
autoflow rollback                   # Rollback last sprint
autoflow rollback --sprint=5        # Rollback specific sprint
autoflow merge                      # Merge completed sprints

# Documentation
autoflow docs generate              # Generate design docs
autoflow docs link                  # Link docs to tasks

# Sprints
autoflow sprints list               # List all sprints
autoflow sprints show 5             # Show sprint details
autoflow sprints create             # Create new sprint

# Agents & Skills
autoflow agents list                # List available agents
autoflow agents test code-implementer  # Test agent
autoflow skills list                # List available skills
```

### 6.2 CLI Implementation

```rust
// crates/autoflow-cli/src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "autoflow")]
#[command(about = "Best-in-class autonomous coding agent", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install AutoFlow globally
    Install {
        #[arg(long)]
        force: bool,
    },

    /// Initialize new project
    Init {
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Start autonomous development
    Start {
        #[arg(short, long)]
        parallel: bool,

        #[arg(short, long)]
        sprint: Option<u32>,
    },

    /// Show sprint progress
    Status {
        #[arg(short, long)]
        json: bool,
    },

    /// Rollback sprint
    Rollback {
        #[arg(short, long)]
        sprint: Option<u32>,
    },

    // ... other commands
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    tracing_subscriber::fmt()
        .with_env_filter(if cli.verbose {
            "debug"
        } else {
            "info"
        })
        .init();

    match cli.command {
        Commands::Install { force } => {
            commands::install::run(force).await?;
        }
        Commands::Init { template } => {
            commands::init::run(template).await?;
        }
        Commands::Start { parallel, sprint } => {
            commands::start::run(parallel, sprint).await?;
        }
        // ... other commands
    }

    Ok(())
}
```

---

## 7. Development Workflow

### 7.1 Building Locally

```bash
cd /opt/workspaces/autoflow

# Run tests
cargo test --all

# Build debug version
cargo build

# Build release version
cargo build --release

# Run local version (without installing)
cargo run -- start --parallel

# Watch mode (auto-rebuild on changes)
cargo watch -x run
```

### 7.2 Testing Strategy

```rust
// Unit tests (within each crate)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprint_status_transitions() {
        let status = SprintStatus::Pending;
        assert_eq!(status.next(), Some(SprintStatus::WriteUnitTests));
    }

    #[tokio::test]
    async fn test_agent_executor() {
        let executor = AgentExecutor::new(test_config());
        let output = executor.run("test-writer", &test_sprint()).await;
        assert!(output.is_ok());
    }
}

// Integration tests (in tests/ directory)
#[tokio::test]
async fn test_full_pipeline() {
    let temp_dir = tempfile::tempdir()?;
    let project = setup_test_project(&temp_dir)?;

    let orchestrator = Orchestrator::new(test_config());
    orchestrator.run().await?;

    // Verify sprints completed
    let sprints = load_sprints(&project)?;
    assert!(sprints.all_done());
}

// Property-based tests
proptest! {
    #[test]
    fn test_sprint_invariants(status in any::<SprintStatus>()) {
        // Invariant: next() always returns valid state
        if let Some(next) = status.next() {
            assert!(is_valid_transition(status, next));
        }
    }
}
```

---

## 8. Implementation Plan

### Phase 1: Foundation (Weeks 1-2)

**Deliverables**:
- [ ] Cargo workspace setup (7 crates)
- [ ] Core data structures (Sprint, Task, Config)
- [ ] Schema validation (serde + jsonschema)
- [ ] File operations (read/write SPRINTS.yml)
- [ ] CLI skeleton (clap commands)
- [ ] Install script (`scripts/install.sh`)
- [ ] Unit tests (50%+ coverage)

**Success Criteria**:
- `cargo build` succeeds
- `cargo test` passes
- `autoflow install` works
- `autoflow init` creates project structure

### Phase 2: Orchestrator (Weeks 3-4)

**Deliverables**:
- [ ] State machine (12 phases)
- [ ] Phase transitions
- [ ] Iteration limits
- [ ] Error handling
- [ ] Logging (tracing)
- [ ] Integration tests

**Success Criteria**:
- State machine transitions correctly
- Errors handled gracefully
- Logs structured and readable

### Phase 3: Agent Integration (Weeks 5-6)

**Deliverables**:
- [ ] Agent executor (spawn Claude Code)
- [ ] Output parser (JSON stream)
- [ ] Agent selector (automatic)
- [ ] Context injection
- [ ] Tool restriction enforcement

**Success Criteria**:
- Agents spawn successfully
- Output parsed correctly
- Context injected properly

### Phase 4: Quality Gates (Weeks 7-8)

**Deliverables**:
- [ ] 5-layer validation pipeline
- [ ] Auto-fix logic
- [ ] Schema validator
- [ ] Format validator
- [ ] Blocker detector

**Success Criteria**:
- LLM mistakes caught early
- Auto-fix reduces manual intervention
- Validation reports actionable

### Phase 5: Git & Worktrees (Weeks 9-10)

**Deliverables**:
- [ ] Worktree manager
- [ ] Branch operations
- [ ] Merge logic
- [ ] Rollback support
- [ ] Docker integration

**Success Criteria**:
- Worktrees isolate sprints
- Rollback works perfectly
- Merges conflict-free

### Phase 6: Parallel Execution (Weeks 11-12)

**Deliverables**:
- [ ] Parallel sprint execution
- [ ] Resource management
- [ ] Progress tracking
- [ ] Metrics collection

**Success Criteria**:
- 2-3x speedup with parallel mode
- No race conditions
- Resources managed efficiently

### Phase 7: Polish (Weeks 13-14)

**Deliverables**:
- [ ] CLI polish (colors, progress bars)
- [ ] Documentation (docs/)
- [ ] Examples (examples/)
- [ ] Performance optimization
- [ ] 80%+ test coverage

**Success Criteria**:
- User experience excellent
- Documentation complete
- Performance targets met

---

## 9. Key Decisions

### ✅ **Single Global Installation**
- **Location**: `~/.autoflow/`
- **Benefits**: Single update point, no duplication, consistent versions
- **Projects**: Reference global installation, only store project data

### ✅ **Build in This Directory**
- **Location**: `/opt/workspaces/autoflow/`
- **Benefits**: Version controlled, easy development, single source of truth

### ✅ **Rust with Tokio**
- **Benefits**: Type safety, performance, concurrency, memory safety
- **Trade-off**: Learning curve acceptable for long-term benefits

### ✅ **Multi-Layer Quality Gates**
- **Benefits**: Catch LLM mistakes early, auto-fix common issues
- **Layers**: Syntax → Format → Semantic → Integration → Security

### ✅ **Git Worktree Isolation**
- **Benefits**: Perfect rollback, parallel execution, no conflicts
- **Pattern**: One worktree per sprint, merge when done

---

## Next Steps

1. **Set up Cargo workspace** (Day 1)
2. **Implement core data structures** (Days 2-3)
3. **Create install script** (Day 4)
4. **Build CLI skeleton** (Day 5)
5. **Start orchestrator implementation** (Week 2)

**Ready to build the best autonomous coding agent!** 🚀
