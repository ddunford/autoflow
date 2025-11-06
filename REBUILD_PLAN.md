# AutoFlow Rebuild Plan: From Bash to Production-Grade Framework

**Status**: Planning Phase
**Date**: 2025-11-05
**Purpose**: Rebuild AutoFlow autonomous coding agent with modern architecture, type safety, and enterprise-grade quality gates

---

## Executive Summary

The current AutoFlow system is a sophisticated 62-script Bash orchestration engine managing a 12-phase TDD pipeline with Git worktree isolation, intelligent agent specialization, and memory integration. While functionally robust, it suffers from:

1. **Maintainability**: Shell script complexity makes debugging and extending difficult
2. **Type Safety**: No compile-time validation of data structures (SPRINTS.yml, schemas)
3. **Testing**: Limited unit test coverage for orchestration logic
4. **Error Handling**: Inconsistent error propagation across 62 scripts
5. **Observability**: Text-based logging without structured metrics
6. **Agent Output**: Inconsistent YAML/JSON parsing requires runtime validation

This document proposes a **complete rebuild** using a modern, type-safe, CLI-first framework with comprehensive quality gates to catch LLM mistakes before they cascade.

---

## Table of Contents

1. [Technology Stack Evaluation](#1-technology-stack-evaluation)
2. [Architecture Design](#2-architecture-design)
3. [Quality Gates & LLM Mistake Prevention](#3-quality-gates--llm-mistake-prevention)
4. [Development Process & Standards](#4-development-process--standards)
5. [Migration Strategy](#5-migration-strategy)
6. [Implementation Roadmap](#6-implementation-roadmap)
7. [Success Metrics](#7-success-metrics)

---

## 1. Technology Stack Evaluation

### Requirements

**Must Have**:
- CLI-first design (subcommands, flags, arguments)
- Type-safe data structures (SPRINTS.yml, schemas)
- Concurrent execution (parallel sprints)
- Process management (spawn agents, monitor output)
- Git operations (worktrees, branching, commits)
- File operations (read, edit, validate)
- Schema validation (JSON Schema)
- Structured logging
- Testing framework support
- Plugin/extension system

**Nice to Have**:
- Hot reload during development
- Interactive prompts (TUI)
- Progress bars and spinners
- Configuration management
- Metrics collection
- Dependency injection
- Error recovery patterns

### Language Comparison

| Criteria | **Rust** | **Go** | **TypeScript/Deno** | **Python** |
|----------|----------|--------|---------------------|------------|
| **Type Safety** | 🟢 Excellent (compile-time) | 🟢 Good (compile-time) | 🟡 Good (runtime with Zod) | 🔴 Weak (runtime only) |
| **CLI Libraries** | 🟢 clap, structopt | 🟢 cobra, cli | 🟡 commander, yargs | 🟢 click, typer |
| **Concurrency** | 🟢 Tokio (async/await) | 🟢 Goroutines (native) | 🟢 Native async/await | 🟡 asyncio (complex) |
| **Performance** | 🟢 Fastest | 🟢 Fast | 🟡 Medium | 🔴 Slow |
| **Error Handling** | 🟢 Result<T, E> | 🟢 error values | 🟡 try/catch | 🔴 exceptions |
| **Testing** | 🟢 Built-in + proptest | 🟢 Built-in + testify | 🟢 Deno built-in | 🟢 pytest |
| **Ecosystem** | 🟡 Growing | 🟢 Mature | 🟢 Massive (npm) | 🟢 Massive |
| **Deployment** | 🟢 Single binary | 🟢 Single binary | 🟡 Requires runtime | 🔴 Requires interpreter |
| **Learning Curve** | 🔴 Steep | 🟢 Gentle | 🟢 Gentle | 🟢 Very gentle |
| **DevEx** | 🟡 Cargo excellent | 🟢 Go tooling good | 🟢 Deno excellent | 🟢 Simple |
| **Parsing (YAML/JSON)** | 🟢 serde (zero-copy) | 🟢 encoding/json | 🟢 Native JSON | 🟢 pyyaml, pydantic |
| **Process Management** | 🟢 tokio::process | 🟢 os/exec | 🟢 Deno.Command | 🟡 subprocess |
| **Git Integration** | 🟡 git2-rs | 🟡 go-git | 🔴 shell out | 🔴 GitPython |

### Recommendation: **Rust with Tokio**

**Rationale**:

1. **Type Safety**: Compile-time validation of SPRINTS.yml structure prevents entire class of runtime errors
2. **Performance**: Handles concurrent sprint execution without GIL limitations
3. **Single Binary**: Deploy `autoflow` binary with zero dependencies
4. **Error Handling**: Result<T, E> forces explicit error handling (no silent failures)
5. **Serde**: Zero-copy YAML/JSON parsing with automatic validation
6. **Tokio Ecosystem**: Process spawning, async I/O, channels for agent communication
7. **CLI Excellence**: clap provides subcommands, validation, help generation
8. **Testing**: Built-in unit tests, integration tests, property-based testing
9. **Memory Safety**: No segfaults, no data races, no undefined behavior

**Trade-offs**:
- Steeper learning curve (offset by long-term maintainability)
- Smaller ecosystem than Go/TypeScript (but sufficient for our needs)
- Compile times moderate (acceptable for CLI tool)

**Alternative**: Go (if team has Go expertise, excellent choice with goroutines)

---

## 2. Architecture Design

### 2.1 System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Layer (clap)                        │
│  autoflow [start|init|status|metrics|rollback|clean] [flags]   │
└────────────────────┬────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────┐
│                    Command Handlers                             │
│  StartCommand | InitCommand | StatusCommand | ...              │
└────────────────────┬────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────┐
│                  Orchestrator (State Machine)                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  SprintState: Pending → WriteUnitTests → WriteCode →    │  │
│  │              Review → ReviewFix → RunTests → ...         │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────┬─────────────────────────┬────────────────────────────────┘
      │                         │
┌─────▼──────────┐    ┌─────────▼────────┐    ┌─────────────────┐
│ Agent Manager  │    │  Worktree Mgr    │    │  Quality Gates  │
│ - Spawn agents │    │  - Create/delete │    │  - Validation   │
│ - Monitor IO   │    │  - Merge/rebase  │    │  - Review       │
│ - Parse output │    │  - Rollback      │    │  - Tests        │
└────────┬───────┘    └──────────────────┘    └─────────────────┘
         │
┌────────▼──────────────────────────────────────────────────────┐
│                    Data Layer (serde)                         │
│  SprintsYaml | CodeReviewResults | TestResults | Config      │
└───────────────────────────────────────────────────────────────┘
         │
┌────────▼──────────────────────────────────────────────────────┐
│                Storage (filesystem + git)                     │
│  .autoflow/SPRINTS.yml | phase-N/sprints/sprint-X/           │
└───────────────────────────────────────────────────────────────┘
```

### 2.2 Module Structure

```rust
autoflow/
├── Cargo.toml                    // Dependencies
├── src/
│   ├── main.rs                   // CLI entry point
│   ├── cli/
│   │   ├── mod.rs               // CLI argument parsing
│   │   ├── commands/
│   │   │   ├── start.rs         // autoflow start
│   │   │   ├── init.rs          // autoflow init
│   │   │   ├── status.rs        // autoflow status
│   │   │   ├── metrics.rs       // autoflow metrics
│   │   │   └── rollback.rs      // autoflow rollback
│   │   └── output.rs            // Colored output, progress bars
│   ├── orchestrator/
│   │   ├── mod.rs               // Orchestrator trait
│   │   ├── state_machine.rs    // 12-phase TDD pipeline
│   │   ├── phase_transitions.rs // State transition logic
│   │   └── execution_loop.rs    // Main loop with iteration limits
│   ├── agents/
│   │   ├── mod.rs               // Agent trait
│   │   ├── executor.rs          // Spawn Claude Code with context
│   │   ├── selector.rs          // Automatic agent selection
│   │   ├── output_parser.rs    // Parse JSON stream from agents
│   │   └── definitions.rs       // Load agent frontmatter
│   ├── worktree/
│   │   ├── mod.rs               // Worktree operations
│   │   ├── manager.rs           // Create/delete worktrees
│   │   ├── docker.rs            // Docker container per worktree
│   │   └── mcp_config.rs        // MCP server reconfiguration
│   ├── quality/
│   │   ├── mod.rs               // Quality gate trait
│   │   ├── schema_validator.rs // JSON Schema validation
│   │   ├── code_reviewer.rs    // Code review gate
│   │   ├── test_runner.rs      // Test execution
│   │   └── blocker_detector.rs // Blocking issue detection
│   ├── data/
│   │   ├── mod.rs               // Data structure definitions
│   │   ├── sprints.rs           // SPRINTS.yml structure
│   │   ├── tasks.rs             // Task structure
│   │   ├── results.rs           // Test/review results
│   │   └── config.rs            // Configuration
│   ├── storage/
│   │   ├── mod.rs               // Storage trait
│   │   ├── filesystem.rs        // File operations
│   │   ├── git.rs               // Git operations
│   │   └── backup.rs            // Backup/restore
│   ├── memory/
│   │   ├── mod.rs               // Memory integration
│   │   ├── client.rs            // MCP memory server client
│   │   └── patterns.rs          // Pattern storage/retrieval
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── logging.rs           // Structured logging (tracing)
│   │   ├── signals.rs           // SIGINT/SIGTERM handling
│   │   └── metrics.rs           // Telemetry collection
│   └── error.rs                 // Error types
├── tests/
│   ├── integration/
│   │   ├── test_full_pipeline.rs
│   │   └── test_worktree_isolation.rs
│   └── fixtures/                // Test fixtures
└── README.md
```

### 2.3 Core Data Structures

```rust
// src/data/sprints.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintsYaml {
    pub project: ProjectMetadata,
    pub sprints: Vec<Sprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: u32,
    pub goal: String,
    pub status: SprintStatus,
    pub duration: String,
    pub total_effort: String,
    pub max_effort: String,
    pub started: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub deliverables: Vec<String>,
    pub tasks: Vec<Task>,
    pub blocked_count: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SprintStatus {
    Pending,
    WriteUnitTests,
    WriteCode,
    CodeReview,
    ReviewFix,
    RunUnitTests,
    UnitFix,
    WriteE2eTests,
    RunE2eTests,
    E2eFix,
    Blocked,
    Complete,
    Done,
}

impl SprintStatus {
    /// Get next status in TDD pipeline
    pub fn next(&self) -> Option<SprintStatus> {
        match self {
            SprintStatus::Pending => Some(SprintStatus::WriteUnitTests),
            SprintStatus::WriteUnitTests => Some(SprintStatus::WriteCode),
            SprintStatus::WriteCode => Some(SprintStatus::CodeReview),
            // ... rest of state transitions
            SprintStatus::Done => None,
            SprintStatus::Blocked => None, // Requires manual intervention
        }
    }

    /// Can this status be retried?
    pub fn is_retriable(&self) -> bool {
        matches!(self, SprintStatus::ReviewFix | SprintStatus::UnitFix | SprintStatus::E2eFix)
    }

    /// Maximum retry attempts before BLOCKED
    pub fn max_retries(&self) -> u32 {
        match self {
            SprintStatus::UnitFix | SprintStatus::E2eFix => 3,
            SprintStatus::ReviewFix => 5, // More attempts for review issues
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub effort: String,
    pub priority: Priority,
    pub feature: String,
    pub docs: Vec<String>,
    pub business_rules: Vec<String>,
    pub testing: TestingRequirements,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoFlowError {
    #[error("Failed to parse SPRINTS.yml: {0}")]
    SprintsParseError(#[from] serde_yaml::Error),

    #[error("Schema validation failed: {0}")]
    ValidationError(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("Agent execution failed: {0}")]
    AgentError(String),

    #[error("Sprint {0} is blocked: {1}")]
    SprintBlocked(u32, String),

    #[error("Maximum iterations ({0}) reached")]
    MaxIterationsExceeded(u32),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AutoFlowError>;
```

### 2.4 Orchestrator Implementation

```rust
// src/orchestrator/state_machine.rs
use crate::data::sprints::{Sprint, SprintStatus};
use crate::agents::AgentExecutor;
use crate::quality::QualityGate;
use crate::error::Result;
use tracing::{info, warn};

pub struct Orchestrator {
    agent_executor: AgentExecutor,
    quality_gates: Vec<Box<dyn QualityGate>>,
    max_iterations: u32,
}

impl Orchestrator {
    pub async fn run_sprint(&self, sprint: &mut Sprint) -> Result<()> {
        let mut iteration = 0;
        let mut retry_count = 0;

        while sprint.status != SprintStatus::Done && iteration < self.max_iterations {
            iteration += 1;
            info!("Sprint {} iteration {}: status={:?}", sprint.id, iteration, sprint.status);

            // Execute phase
            let result = self.execute_phase(sprint).await;

            match result {
                Ok(_) => {
                    // Phase succeeded, advance to next status
                    if let Some(next_status) = sprint.status.next() {
                        sprint.status = next_status;
                        retry_count = 0; // Reset retry counter
                        sprint.last_updated = chrono::Utc::now();
                    } else {
                        // No next status, we're done
                        sprint.status = SprintStatus::Done;
                    }
                }
                Err(e) => {
                    // Phase failed
                    warn!("Sprint {} phase failed: {}", sprint.id, e);

                    if sprint.status.is_retriable() {
                        retry_count += 1;
                        if retry_count >= sprint.status.max_retries() {
                            // Max retries exceeded, mark as BLOCKED
                            sprint.status = SprintStatus::Blocked;
                            sprint.blocked_count = Some(sprint.blocked_count.unwrap_or(0) + 1);
                            return Err(AutoFlowError::SprintBlocked(
                                sprint.id,
                                format!("Max retries ({}) exceeded for {:?}", retry_count, sprint.status)
                            ));
                        }
                        // Stay in same status for retry
                    } else {
                        // Non-retriable phase failed, mark as BLOCKED
                        sprint.status = SprintStatus::Blocked;
                        return Err(e);
                    }
                }
            }

            // Save progress
            self.save_sprint(sprint).await?;
        }

        if iteration >= self.max_iterations {
            return Err(AutoFlowError::MaxIterationsExceeded(self.max_iterations));
        }

        Ok(())
    }

    async fn execute_phase(&self, sprint: &Sprint) -> Result<()> {
        match sprint.status {
            SprintStatus::WriteUnitTests => {
                self.agent_executor.run("test-writer", sprint).await?;
            }
            SprintStatus::WriteCode => {
                self.agent_executor.run("code-implementer", sprint).await?;
            }
            SprintStatus::CodeReview => {
                self.agent_executor.run("reviewer", sprint).await?;
                self.run_quality_gates(sprint).await?;
            }
            SprintStatus::ReviewFix => {
                self.agent_executor.run("review-fixer", sprint).await?;
            }
            SprintStatus::RunUnitTests => {
                self.run_tests(sprint, TestType::Unit).await?;
            }
            SprintStatus::UnitFix => {
                self.agent_executor.run("unit-fixer", sprint).await?;
            }
            SprintStatus::WriteE2eTests => {
                self.agent_executor.run("e2e-writer", sprint).await?;
            }
            SprintStatus::RunE2eTests => {
                self.run_tests(sprint, TestType::E2e).await?;
            }
            SprintStatus::E2eFix => {
                self.agent_executor.run("e2e-fixer", sprint).await?;
            }
            SprintStatus::Complete => {
                // Final validation before marking DONE
                self.run_quality_gates(sprint).await?;
                sprint.status = SprintStatus::Done;
            }
            _ => {}
        }

        Ok(())
    }
}
```

### 2.5 Agent Executor

```rust
// src/agents/executor.rs
use tokio::process::Command;
use tokio::io::{BufReader, AsyncBufReadExt};
use serde_json::Value;
use crate::error::Result;

pub struct AgentExecutor {
    agents_dir: PathBuf,
    max_turns: u32,
}

impl AgentExecutor {
    pub async fn run(&self, agent_name: &str, sprint: &Sprint) -> Result<()> {
        // Load agent definition
        let agent = self.load_agent_definition(agent_name)?;

        // Prepare context
        let context = self.prepare_context(sprint, &agent)?;

        // Spawn Claude Code with agent
        let mut child = Command::new("claude")
            .arg("--agent")
            .arg(&agent.path)
            .arg("--max-turns")
            .arg(agent.max_turns.unwrap_or(self.max_turns).to_string())
            .arg("--json-output")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Write context to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(context.as_bytes()).await?;
        }

        // Read output stream
        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            // Parse JSON stream output
            if let Ok(json) = serde_json::from_str::<Value>(&line) {
                self.handle_agent_output(json, sprint).await?;
            }
        }

        // Wait for completion
        let status = child.wait().await?;
        if !status.success() {
            return Err(AutoFlowError::AgentError(
                format!("Agent {} failed with exit code {:?}", agent_name, status.code())
            ));
        }

        Ok(())
    }

    async fn handle_agent_output(&self, output: Value, sprint: &Sprint) -> Result<()> {
        match output["type"].as_str() {
            Some("tool_use") => {
                // Agent used a tool (Write, Edit, etc.)
                let tool = output["tool"].as_str().unwrap_or("unknown");
                info!("Agent used tool: {}", tool);

                // Run quality gates on tool output
                if tool == "Write" || tool == "Edit" {
                    self.validate_agent_output(&output)?;
                }
            }
            Some("error") => {
                let message = output["message"].as_str().unwrap_or("unknown error");
                return Err(AutoFlowError::AgentError(message.to_string()));
            }
            _ => {}
        }

        Ok(())
    }

    fn validate_agent_output(&self, output: &Value) -> Result<()> {
        // Extract file content from tool output
        let content = output["content"].as_str().unwrap_or("");

        // If it's YAML, validate schema
        if output["file_path"].as_str().unwrap_or("").ends_with(".yml") {
            // Validate against JSON schema
            self.validate_yaml_schema(content)?;
        }

        Ok(())
    }
}
```

---

## 3. Quality Gates & LLM Mistake Prevention

### 3.1 Problem: LLM Output Inconsistency

**Common Mistakes**:
1. Agent produces markdown instead of YAML
2. Schema violations (wrong field names, types)
3. Missing required fields
4. Invalid enum values (e.g., "in_progress" instead of "IN_PROGRESS")
5. Malformed JSON/YAML syntax
6. Inconsistent file paths (relative vs absolute)

### 3.2 Quality Gate Architecture

```rust
// src/quality/mod.rs
use async_trait::async_trait;
use crate::data::sprints::Sprint;
use crate::error::Result;

#[async_trait]
pub trait QualityGate: Send + Sync {
    /// Gate name for logging
    fn name(&self) -> &str;

    /// Run quality gate check
    async fn check(&self, sprint: &Sprint) -> Result<QualityGateResult>;

    /// Attempt automatic fix
    async fn fix(&self, sprint: &Sprint) -> Result<()>;
}

pub struct QualityGateResult {
    pub passed: bool,
    pub issues: Vec<QualityIssue>,
}

pub struct QualityIssue {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub auto_fixable: bool,
}
```

### 3.3 Schema Validation Gate

```rust
// src/quality/schema_validator.rs
use jsonschema::{JSONSchema, ValidationError};
use serde_yaml::Value;
use std::path::Path;

pub struct SchemaValidator {
    schemas: HashMap<String, JSONSchema>,
}

impl SchemaValidator {
    pub fn new(schemas_dir: &Path) -> Result<Self> {
        let mut schemas = HashMap::new();

        // Load all JSON schemas
        for entry in std::fs::read_dir(schemas_dir)? {
            let path = entry?.path();
            if path.extension() == Some("json".as_ref()) {
                let schema_name = path.file_stem().unwrap().to_str().unwrap();
                let schema_content = std::fs::read_to_string(&path)?;
                let schema_json: serde_json::Value = serde_json::from_str(&schema_content)?;
                let compiled = JSONSchema::compile(&schema_json)?;
                schemas.insert(schema_name.to_string(), compiled);
            }
        }

        Ok(Self { schemas })
    }

    pub fn validate_file(&self, file_path: &Path) -> Result<Vec<ValidationError>> {
        // Determine schema from file name
        let schema_name = self.infer_schema_name(file_path)?;
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| AutoFlowError::ValidationError(format!("No schema for {}", schema_name)))?;

        // Load file content
        let content = std::fs::read_to_string(file_path)?;

        // Parse YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;

        // Convert to JSON for validation
        let json_value: serde_json::Value = serde_json::to_value(yaml_value)?;

        // Validate
        let result = schema.validate(&json_value);
        match result {
            Ok(_) => Ok(vec![]),
            Err(errors) => Ok(errors.collect()),
        }
    }

    fn infer_schema_name(&self, file_path: &Path) -> Result<&str> {
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        match file_name {
            "SPRINTS.yml" => Ok("sprints"),
            "CODE_REVIEW_RESULTS.yml" => Ok("code_review_results"),
            "TEST_RESULTS.yml" => Ok("test_results"),
            "E2E_TEST_RESULTS.yml" => Ok("e2e_test_results"),
            _ => Err(AutoFlowError::ValidationError(format!("Unknown file: {}", file_name))),
        }
    }
}

#[async_trait]
impl QualityGate for SchemaValidator {
    fn name(&self) -> &str {
        "schema_validator"
    }

    async fn check(&self, sprint: &Sprint) -> Result<QualityGateResult> {
        let sprint_dir = Path::new(&format!(".autoflow/phase-{}/sprints/sprint-{}", 1, sprint.id));
        let mut issues = vec![];

        // Validate all YAML files in sprint directory
        for entry in std::fs::read_dir(sprint_dir)? {
            let path = entry?.path();
            if path.extension() == Some("yml".as_ref()) {
                let errors = self.validate_file(&path)?;
                for error in errors {
                    issues.push(QualityIssue {
                        severity: Severity::Critical,
                        category: "schema_violation".to_string(),
                        message: format!("Schema validation failed: {}", error),
                        file_path: Some(path.to_str().unwrap().to_string()),
                        line_number: None,
                        auto_fixable: false,
                    });
                }
            }
        }

        Ok(QualityGateResult {
            passed: issues.is_empty(),
            issues,
        })
    }

    async fn fix(&self, sprint: &Sprint) -> Result<()> {
        // Schema violations typically not auto-fixable
        // Log detailed error and require manual intervention
        Err(AutoFlowError::ValidationError(
            "Schema violations require manual fix or agent re-run".to_string()
        ))
    }
}
```

### 3.4 Output Format Validator

```rust
// src/quality/output_validator.rs
/// Detects when agent produces markdown instead of YAML
pub struct OutputFormatValidator;

impl OutputFormatValidator {
    pub fn validate_yaml_output(&self, file_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;

        // Check for markdown headers (agent mistake)
        if content.contains("```yaml") || content.contains("```yml") {
            return Err(AutoFlowError::ValidationError(
                format!("File {} contains markdown code blocks instead of raw YAML", file_path.display())
            ));
        }

        // Check for markdown headings
        if content.lines().any(|line| line.starts_with("# ") || line.starts_with("## ")) {
            return Err(AutoFlowError::ValidationError(
                format!("File {} contains markdown headings", file_path.display())
            ));
        }

        // Attempt to parse as YAML
        let _: serde_yaml::Value = serde_yaml::from_str(&content)?;

        Ok(())
    }

    pub fn auto_fix_markdown_yaml(&self, file_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;

        // Extract YAML from markdown code blocks
        let yaml_content = if content.contains("```yaml") {
            // Find content between ```yaml and ```
            let start = content.find("```yaml").unwrap() + 7;
            let end = content[start..].find("```").unwrap() + start;
            content[start..end].trim()
        } else {
            &content
        };

        // Write cleaned YAML back
        std::fs::write(file_path, yaml_content)?;

        Ok(())
    }
}
```

### 3.5 Blocking Issue Detector

```rust
// src/quality/blocker_detector.rs
/// Detects issues that will block test execution
pub struct BlockerDetector;

impl BlockerDetector {
    pub async fn analyze(&self, sprint: &Sprint) -> Result<Option<BlockingIssue>> {
        // Check for missing backend APIs
        if self.has_missing_backend_apis(sprint).await? {
            return Ok(Some(BlockingIssue {
                reason: "Frontend tasks call backend APIs that don't exist".to_string(),
                prerequisite_work: "Implement backend APIs first".to_string(),
                estimated_effort: "8-16 hours".to_string(),
            }));
        }

        // Check for missing dependencies
        if self.has_missing_dependencies(sprint).await? {
            return Ok(Some(BlockingIssue {
                reason: "Required dependencies not installed".to_string(),
                prerequisite_work: "Run npm install or composer install".to_string(),
                estimated_effort: "5 minutes".to_string(),
            }));
        }

        // Check for docker not running
        if self.requires_docker(sprint).await? && !self.is_docker_running().await? {
            return Ok(Some(BlockingIssue {
                reason: "Tests require Docker but containers not running".to_string(),
                prerequisite_work: "Start Docker containers".to_string(),
                estimated_effort: "1 minute".to_string(),
            }));
        }

        Ok(None)
    }

    async fn has_missing_backend_apis(&self, sprint: &Sprint) -> Result<bool> {
        // Parse frontend code for API calls
        // Check if corresponding backend endpoints exist
        // Return true if mismatch found
        Ok(false) // Placeholder
    }
}
```

### 3.6 Continuous Validation Pipeline

```rust
// src/quality/pipeline.rs
pub struct QualityPipeline {
    gates: Vec<Box<dyn QualityGate>>,
}

impl QualityPipeline {
    pub fn new() -> Self {
        Self {
            gates: vec![
                Box::new(SchemaValidator::new(Path::new("schemas")).unwrap()),
                Box::new(OutputFormatValidator),
                Box::new(BlockerDetector),
                Box::new(CodeReviewGate),
                Box::new(TestCoverageGate),
            ],
        }
    }

    pub async fn run_all(&self, sprint: &Sprint) -> Result<QualityReport> {
        let mut report = QualityReport::new();

        for gate in &self.gates {
            let result = gate.check(sprint).await?;
            report.add_result(gate.name(), result);

            // If critical gate fails, stop pipeline
            if !result.passed && gate.is_critical() {
                report.status = QualityStatus::Failed;
                break;
            }
        }

        Ok(report)
    }

    pub async fn auto_fix(&self, sprint: &Sprint) -> Result<()> {
        for gate in &self.gates {
            let result = gate.check(sprint).await?;
            if !result.passed {
                // Attempt auto-fix
                if let Err(e) = gate.fix(sprint).await {
                    warn!("Auto-fix failed for {}: {}", gate.name(), e);
                } else {
                    info!("Auto-fix succeeded for {}", gate.name());
                }
            }
        }

        Ok(())
    }
}
```

---

## 4. Development Process & Standards

### 4.1 TDD Approach

**Red-Green-Refactor for AutoFlow itself**:

1. **RED**: Write tests for orchestrator logic
   ```rust
   #[tokio::test]
   async fn test_sprint_status_transitions() {
       let mut sprint = Sprint {
           id: 1,
           status: SprintStatus::Pending,
           // ...
       };

       let orchestrator = Orchestrator::new();
       orchestrator.run_sprint(&mut sprint).await.unwrap();

       assert_eq!(sprint.status, SprintStatus::WriteUnitTests);
   }
   ```

2. **GREEN**: Implement minimal code
   ```rust
   impl Orchestrator {
       pub async fn run_sprint(&self, sprint: &mut Sprint) -> Result<()> {
           if sprint.status == SprintStatus::Pending {
               sprint.status = SprintStatus::WriteUnitTests;
           }
           Ok(())
       }
   }
   ```

3. **REFACTOR**: Improve design
   ```rust
   impl SprintStatus {
       pub fn next(&self) -> Option<SprintStatus> {
           match self {
               SprintStatus::Pending => Some(SprintStatus::WriteUnitTests),
               // ...
           }
       }
   }
   ```

### 4.2 Testing Strategy

**Unit Tests** (80%+ coverage):
- State machine transitions
- Schema validation logic
- Agent output parsing
- Error handling paths
- Git operations (with mocked repository)

**Integration Tests**:
- Full pipeline with mock agents
- Worktree creation/deletion
- File operations
- Quality gate execution

**Property-Based Tests** (proptest):
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_sprint_status_transitions_always_valid(
        initial_status in any::<SprintStatus>()
    ) {
        let mut sprint = Sprint {
            status: initial_status,
            // ...
        };

        let orchestrator = Orchestrator::new();
        let result = orchestrator.run_sprint(&mut sprint).await;

        // Invariant: status only transitions to valid next state
        if let Ok(_) = result {
            assert!(is_valid_transition(initial_status, sprint.status));
        }
    }
}
```

**E2E Tests**:
- Real project initialization
- Real agent execution (with Claude Code)
- Full TDD pipeline on sample project

### 4.3 Code Quality Standards

**Mandatory**:
- `cargo fmt` - Consistent formatting
- `cargo clippy` - Lint errors must be fixed
- `cargo test` - All tests pass
- `cargo doc` - Documentation generation succeeds
- 80%+ test coverage (tarpaulin)

**CI Pipeline** (GitHub Actions):
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test --all-features
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v1
```

### 4.4 Error Handling Standards

**Always use Result<T>**:
```rust
// ❌ Bad: panic on error
fn parse_sprints(path: &Path) -> SprintsYaml {
    let content = std::fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content).unwrap()
}

// ✅ Good: return Result
fn parse_sprints(path: &Path) -> Result<SprintsYaml> {
    let content = std::fs::read_to_string(path)?;
    let sprints = serde_yaml::from_str(&content)?;
    Ok(sprints)
}
```

**Use thiserror for error types**:
```rust
#[derive(Error, Debug)]
pub enum AutoFlowError {
    #[error("Failed to parse SPRINTS.yml: {0}")]
    SprintsParseError(#[from] serde_yaml::Error),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
}
```

**Propagate errors with ?**:
```rust
pub async fn run(&self) -> Result<()> {
    let sprints = self.load_sprints()?; // Auto-convert error type
    self.validate_sprints(&sprints)?;
    self.execute_sprints(&sprints).await?;
    Ok(())
}
```

### 4.5 Documentation Standards

**Module-level docs**:
```rust
//! # Orchestrator
//!
//! Manages the 12-phase TDD pipeline for autonomous code generation.
//!
//! ## State Machine
//!
//! Sprints transition through the following states:
//! - PENDING → WRITE_UNIT_TESTS
//! - WRITE_UNIT_TESTS → WRITE_CODE
//! - WRITE_CODE → CODE_REVIEW
//! ...
```

**Function docs**:
```rust
/// Executes a single sprint through the TDD pipeline.
///
/// # Arguments
///
/// * `sprint` - Mutable reference to sprint to execute
///
/// # Returns
///
/// * `Ok(())` - Sprint completed successfully
/// * `Err(AutoFlowError::SprintBlocked)` - Sprint blocked after max retries
/// * `Err(AutoFlowError::MaxIterationsExceeded)` - Safety limit reached
///
/// # Examples
///
/// ```rust
/// let mut sprint = Sprint::new(1, "User Auth");
/// orchestrator.run_sprint(&mut sprint).await?;
/// assert_eq!(sprint.status, SprintStatus::Done);
/// ```
pub async fn run_sprint(&self, sprint: &mut Sprint) -> Result<()> {
    // ...
}
```

---

## 5. Migration Strategy

### 5.1 Phased Approach

**Phase 1: Foundation (Weeks 1-2)**
- Set up Rust project structure
- Implement core data structures (Sprint, Task)
- Schema validation (serde + jsonschema)
- File operations (read/write SPRINTS.yml)
- Git basic operations (branch, commit)
- CLI skeleton (clap subcommands)

**Phase 2: Orchestrator (Weeks 3-4)**
- State machine implementation
- Phase transition logic
- Iteration limits
- Error handling
- Logging (tracing)
- Unit tests

**Phase 3: Agent Integration (Weeks 5-6)**
- Agent executor (spawn Claude Code)
- Output parser (JSON stream)
- Agent selector (automatic specialization)
- Context injection
- Tool restrictions
- Integration tests

**Phase 4: Quality Gates (Weeks 7-8)**
- Schema validator
- Output format validator
- Blocker detector
- Code review integration
- Test runner integration
- Auto-fix logic

**Phase 5: Worktree & Advanced (Weeks 9-10)**
- Worktree manager
- Docker integration
- MCP reconfiguration
- Rollback mechanism
- Parallel execution

**Phase 6: Polish & Migration (Weeks 11-12)**
- CLI polish (progress bars, colors)
- Documentation
- Migration scripts (bash → rust)
- Backward compatibility
- Performance optimization

### 5.2 Backward Compatibility

**Dual Mode Support**:
```bash
# Run with Rust orchestrator (default)
autoflow start

# Run with legacy Bash orchestrator
autoflow start --legacy
```

**Data Format Compatibility**:
- SPRINTS.yml format unchanged
- Schema validation ensures compatibility
- File paths remain the same
- Agent definitions unchanged (markdown frontmatter)

### 5.3 Migration Checklist

- [ ] All unit tests passing (Rust)
- [ ] All integration tests passing (Rust)
- [ ] E2E tests passing with sample project
- [ ] Performance benchmarks (compare Bash vs Rust)
- [ ] Memory usage profiling
- [ ] Documentation complete
- [ ] Migration guide written
- [ ] Backward compatibility verified
- [ ] Beta testing with 3+ real projects
- [ ] Production deployment plan

---

## 6. Implementation Roadmap

### Milestone 1: MVP (Weeks 1-4)

**Goal**: Replace orchestrator_v2.sh with Rust equivalent

**Deliverables**:
- [ ] `autoflow start` spawns single sprint
- [ ] State machine transitions (12 phases)
- [ ] SPRINTS.yml read/write with validation
- [ ] Agent executor (spawn Claude Code)
- [ ] Basic error handling
- [ ] Unit tests (50%+ coverage)

**Success Criteria**:
- Single sprint completes PENDING → DONE
- SPRINTS.yml updates correctly
- Errors handled gracefully

### Milestone 2: Quality Gates (Weeks 5-8)

**Goal**: Prevent LLM mistakes with validation pipeline

**Deliverables**:
- [ ] Schema validator (JSON Schema)
- [ ] Output format validator (catch markdown in YAML)
- [ ] Blocker detector (missing APIs, dependencies)
- [ ] Auto-fix logic (format corrections)
- [ ] Quality gate pipeline
- [ ] Integration tests

**Success Criteria**:
- Agent markdown output auto-fixed
- Schema violations caught before merge
- Blocking issues detected early
- 80%+ test coverage

### Milestone 3: Advanced Features (Weeks 9-12)

**Goal**: Feature parity with Bash version

**Deliverables**:
- [ ] Git worktree integration
- [ ] Docker container management
- [ ] MCP reconfiguration
- [ ] Rollback mechanism
- [ ] Parallel sprint execution
- [ ] Metrics collection
- [ ] CLI polish (colors, progress bars)
- [ ] Documentation

**Success Criteria**:
- All Bash features implemented
- Performance equal or better
- User experience improved
- Production-ready

### Milestone 4: Production (Weeks 13-16)

**Goal**: Beta testing and production deployment

**Deliverables**:
- [ ] Beta testing (5+ projects)
- [ ] Bug fixes from beta
- [ ] Performance optimization
- [ ] Migration guide
- [ ] Training materials
- [ ] Production deployment

**Success Criteria**:
- Zero critical bugs in beta
- 50% faster than Bash version
- User satisfaction score 4+/5
- Smooth migration path

---

## 7. Success Metrics

### 7.1 Performance Metrics

| Metric | Current (Bash) | Target (Rust) | Measurement |
|--------|----------------|---------------|-------------|
| **Sprint Execution Time** | 15-30 min | 10-20 min | Wall clock time |
| **Orchestrator Overhead** | 10-20% | <5% | CPU time |
| **Memory Usage** | 200-500 MB | <100 MB | RSS |
| **SPRINTS.yml Parse Time** | 100-200 ms | <10 ms | serde benchmark |
| **Agent Spawn Time** | 2-3 sec | <1 sec | tokio::time |
| **Parallel Speedup** | N/A | 2-3x | 3 sprints parallel |

### 7.2 Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Test Coverage** | 80%+ | cargo tarpaulin |
| **Schema Violations Caught** | 100% | Quality gate tests |
| **LLM Output Errors** | <5% | Agent run success rate |
| **Blocked Sprints** | <10% | BLOCKED status count |
| **Auto-Fix Success Rate** | 70%+ | Fix pipeline metrics |

### 7.3 Developer Experience Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Time to First Sprint** | <5 min | `autoflow init` to sprint done |
| **CLI Response Time** | <100 ms | `autoflow status` latency |
| **Error Message Clarity** | 4+/5 | User survey |
| **Documentation Completeness** | 100% | All commands documented |
| **Setup Time** | <10 min | Fresh install to first run |

---

## 8. Risk Analysis

### 8.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Rust learning curve delays project** | Medium | High | Pair programming, training |
| **Agent output parsing breaks** | Low | Critical | Extensive integration tests |
| **Performance worse than Bash** | Low | Medium | Benchmark early, optimize |
| **Git worktree bugs** | Medium | High | Comprehensive git tests |
| **Schema validation false positives** | Medium | Medium | Tunable strictness levels |

### 8.2 Migration Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Backward incompatibility** | Low | Critical | Dual-mode support, beta testing |
| **User resistance to change** | Medium | Medium | Clear migration guide, benefits |
| **Data loss during migration** | Low | Critical | Backup system, rollback plan |
| **Extended migration period** | High | Low | Gradual rollout, opt-in beta |

### 8.3 Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Production bugs** | Medium | High | Extensive testing, canary deployment |
| **Performance degradation** | Low | Medium | Monitoring, profiling |
| **Maintenance burden** | Low | Low | Clean architecture, documentation |

---

## 9. Alternative Frameworks Considered

### 9.1 Go with Cobra

**Pros**:
- Simpler than Rust
- Excellent CLI library (cobra)
- Fast compilation
- Goroutines for concurrency

**Cons**:
- No compile-time YAML validation
- Error handling verbose (if err != nil)
- Nil pointer risks

**Verdict**: Good alternative if team has Go expertise

### 9.2 TypeScript with Deno

**Pros**:
- Familiar to web developers
- Native TypeScript support
- Zod for runtime validation
- Excellent ecosystem

**Cons**:
- Requires Deno runtime
- Runtime validation overhead
- No compile-time safety for YAML
- Performance concerns

**Verdict**: Consider for rapid prototyping

### 9.3 Python with Click

**Pros**:
- Simplest to write
- Excellent libraries (pydantic, pyyaml)
- Fast development

**Cons**:
- Slow execution
- No type safety (even with mypy)
- GIL limits concurrency
- Deployment complexity

**Verdict**: Not recommended for production

---

## 10. Next Steps

### Immediate Actions (Week 1)

1. **Set up Rust project**:
   ```bash
   cargo new autoflow-rust
   cd autoflow-rust
   cargo add tokio --features full
   cargo add serde --features derive
   cargo add serde_yaml
   cargo add serde_json
   cargo add clap --features derive
   cargo add thiserror
   cargo add tracing
   cargo add tracing-subscriber
   cargo add jsonschema
   cargo add git2
   ```

2. **Create module structure** (see section 2.2)

3. **Implement core data structures** (see section 2.3)

4. **Write first unit tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_sprint_status_next() {
           assert_eq!(
               SprintStatus::Pending.next(),
               Some(SprintStatus::WriteUnitTests)
           );
       }

       #[test]
       fn test_parse_sprints_yml() {
           let yaml = r#"
           project:
             name: "Test"
           sprints:
             - id: 1
               goal: "Test sprint"
               status: PENDING
           "#;
           let sprints: SprintsYaml = serde_yaml::from_str(yaml).unwrap();
           assert_eq!(sprints.sprints.len(), 1);
       }
   }
   ```

5. **Set up CI/CD** (GitHub Actions)

### Weekly Milestones (Weeks 2-12)

See section 6 for detailed roadmap.

### Decision Points

**Week 4**: MVP Demo
- If successful: Continue to Milestone 2
- If issues: Reassess technology choice

**Week 8**: Quality Gates Complete
- If validation working: Continue to Milestone 3
- If too many false positives: Adjust schema strictness

**Week 12**: Feature Parity
- If stable: Begin beta testing
- If unstable: Extend development 2-4 weeks

---

## Appendix A: Dependencies

```toml
[package]
name = "autoflow"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# CLI
clap = { version = "4.4", features = ["derive", "color", "suggestions"] }
colored = "2.1"
indicatif = "0.17" # Progress bars

# Error handling
thiserror = "1.0"
anyhow = "1.0" # For application-level errors

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Validation
jsonschema = "0.17"
regex = "1.10"

# Git operations
git2 = "0.18"

# File watching (for live reload)
notify = "6.1"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Configuration
config = "0.13"

# Testing
[dev-dependencies]
proptest = "1.4" # Property-based testing
tempfile = "3.8" # Temp directories for tests
mockito = "1.2" # HTTP mocking
```

## Appendix B: Project Structure

```
autoflow-rust/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli/
│   ├── orchestrator/
│   ├── agents/
│   ├── worktree/
│   ├── quality/
│   ├── data/
│   ├── storage/
│   ├── memory/
│   ├── utils/
│   └── error.rs
├── tests/
│   ├── integration/
│   └── fixtures/
├── benches/
│   └── orchestrator_benchmark.rs
├── docs/
│   ├── architecture.md
│   ├── migration.md
│   └── api.md
└── examples/
    └── simple_sprint.rs
```

---

## Conclusion

Rebuilding AutoFlow in Rust with comprehensive quality gates will:

1. **Eliminate entire classes of bugs** (compile-time validation)
2. **Catch LLM mistakes early** (schema validation, format checking)
3. **Improve performance** (2-3x faster execution)
4. **Enhance maintainability** (type safety, clear architecture)
5. **Enable advanced features** (parallel execution, hot reload)
6. **Provide production-grade reliability** (error handling, observability)

The investment in Rust's learning curve pays dividends through:
- Fewer runtime errors
- Faster execution
- Better developer experience
- Easier debugging
- Scalable architecture

**Recommendation**: Proceed with Rust implementation following phased migration strategy with backward compatibility during transition period.
