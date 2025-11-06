# Contributing to AutoFlow

Thank you for your interest in contributing to AutoFlow! This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Code Quality Standards](#code-quality-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Project Structure](#project-structure)

## Code of Conduct

Be respectful, inclusive, and professional. We're building something amazing together.

## Getting Started

### Prerequisites

- Rust 1.70+
- Git 2.20+
- Claude CLI
- Docker & Docker Compose (optional)

### Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/autoflow
cd autoflow

# Add upstream remote
git remote add upstream https://github.com/autoflow/autoflow
```

## Development Setup

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test --all

# Run AutoFlow from source
cargo run -- --help

# Watch mode (install cargo-watch first)
cargo install cargo-watch
cargo watch -x 'run -- status'
```

## Making Changes

### 1. Create a Branch

```bash
# Update your fork
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/my-new-feature
```

### 2. Make Your Changes

Follow these principles:

**SOLID Principles**
- **Single Responsibility**: Each module/function does one thing well
- **Open/Closed**: Extend behavior without modifying existing code
- **Liskov Substitution**: Subtypes must be substitutable for base types
- **Interface Segregation**: Many specific interfaces > one general interface
- **Dependency Inversion**: Depend on abstractions, not concretions

**DRY (Don't Repeat Yourself)**
- Extract common code into utilities (`autoflow-utils`)
- Use centralized constants (`paths.rs`)
- Reuse existing patterns

**KISS (Keep It Simple)**
- Favor clarity over cleverness
- Functions < 50 lines
- Clear variable names
- No premature optimization

### 3. Code Quality Standards

#### Rust Code Style

```rust
// Good: Clear, documented, single responsibility
/// Extracts YAML content from agent output
/// Handles both ```yaml and ```yml code blocks
pub fn extract_yaml_from_output(output: &str) -> String {
    if output.contains("```yaml") {
        extract_code_block(output, "yaml")
    } else if output.contains("```yml") {
        extract_code_block(output, "yml")
    } else {
        output.trim().to_string()
    }
}

// Bad: Unclear, no docs, doing too much
pub fn process(s: &str) -> String {
    if s.contains("```yaml") {
        s.split("```yaml").nth(1).unwrap().split("```").next().unwrap().trim().to_string()
    } else { s.trim().to_string() }
}
```

#### Error Handling

```rust
// Good: Descriptive errors with context
use anyhow::{Context, Result};

pub fn load_sprints(path: &str) -> Result<SprintsYaml> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read SPRINTS.yml at {}", path))?;

    let sprints = serde_yaml::from_str(&content)
        .context("Failed to parse SPRINTS.yml - check YAML syntax")?;

    Ok(sprints)
}

// Bad: Generic errors
pub fn load_sprints(path: &str) -> Result<SprintsYaml> {
    let content = fs::read_to_string(path)?;
    let sprints = serde_yaml::from_str(&content)?;
    Ok(sprints)
}
```

#### User-Facing Messages

```rust
// Good: Clear, actionable, helpful
println!("{}", "❌ Project not initialized".red());
println!("\n{}", "To initialize:".bright_cyan());
println!("  {}", "autoflow init".bright_blue());
println!("\n{}", "Or create new project:".bright_cyan());
println!("  {}", "autoflow create my-app --idea IDEA.md".bright_blue());

// Bad: Technical, unclear
eprintln!("Error: .autoflow directory not found");
```

#### Documentation

```rust
/// Load sprint configuration from YAML file
///
/// # Arguments
///
/// * `path` - Path to SPRINTS.yml file
///
/// # Returns
///
/// * `Ok(SprintsYaml)` - Parsed sprint configuration
/// * `Err(AutoFlowError)` - Parse error or file not found
///
/// # Examples
///
/// ```
/// let sprints = SprintsYaml::load(".autoflow/SPRINTS.yml")?;
/// ```
pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    // Implementation
}
```

### 4. Testing Requirements

**All new code must have tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_yaml_from_code_block() {
        let output = r#"Here's the YAML:
```yaml
project:
  name: test
```"#;

        let yaml = extract_yaml_from_output(output);
        assert_eq!(yaml, "project:\n  name: test");
    }

    #[test]
    fn test_extract_yaml_no_code_block() {
        let yaml = "key: value";
        assert_eq!(extract_yaml_from_output(yaml), "key: value");
    }
}
```

**Test Coverage Goals:**
- Core logic: 80%+
- Utilities: 90%+
- CLI commands: 60%+ (harder to test)

### 5. Commit Messages

Follow conventional commits:

```bash
# Format
<type>(<scope>): <description>

[optional body]

[optional footer]

# Types
feat: New feature
fix: Bug fix
docs: Documentation only
style: Formatting, missing semicolons, etc.
refactor: Code change that neither fixes a bug nor adds a feature
perf: Performance improvement
test: Adding missing tests
chore: Updating build tasks, package manager configs, etc.

# Examples
feat(cli): add doctor command for diagnostics
fix(worktree): handle branch name conflicts
docs(user-guide): add troubleshooting section
refactor(utils): extract YAML parsing to utils crate
```

## Testing

### Run All Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# All tests
cargo test --all

# With output
cargo test -- --nocapture

# Specific crate
cargo test -p autoflow-data
```

### Manual Testing

```bash
# Test installation
./scripts/install.sh

# Test project creation
cd /tmp
autoflow create test-app --idea IDEA.md
cd test-app
autoflow status

# Test bug fix workflow
autoflow fix "Test bug"
autoflow worktree list

# Test cleanup
cd ..
rm -rf test-app
```

## Submitting Changes

### 1. Run Checks

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all

# Build release
cargo build --release
```

### 2. Update Documentation

- Update relevant `.md` files
- Add entries to `CHANGELOG.md`
- Update code documentation

### 3. Create Pull Request

```bash
# Commit changes
git add .
git commit -m "feat(scope): description"

# Push to your fork
git push origin feature/my-new-feature
```

On GitHub:
1. Go to your fork
2. Click "New Pull Request"
3. Fill in PR template:
   - **Description**: What does this PR do?
   - **Motivation**: Why is this change needed?
   - **Testing**: How was this tested?
   - **Breaking Changes**: Any breaking changes?
   - **Screenshots**: If UI changes

### 4. PR Review Process

- Address review feedback promptly
- Keep commits atomic and well-described
- Rebase on main if needed
- Update PR description if scope changes

## Project Structure

```
autoflow/
├── crates/
│   ├── autoflow-cli/       # CLI commands and main entry point
│   ├── autoflow-core/      # Orchestrator and state machine
│   ├── autoflow-agents/    # Agent execution and management
│   ├── autoflow-quality/   # Quality gates and validation
│   ├── autoflow-data/      # Data structures and schemas
│   ├── autoflow-git/       # Git worktree operations
│   └── autoflow-utils/     # Shared utilities (paths, yaml, etc.)
├── agents/                 # Agent definitions (*.agent.md)
├── skills/                 # Skill definitions (*.md)
├── reference/              # Standards and guides
├── schemas/                # JSON schemas for validation
├── templates/              # Project templates
└── scripts/                # Installation and utility scripts
```

### Adding a New Command

1. Create command module in `crates/autoflow-cli/src/commands/`
2. Add command to `Commands` enum in `main.rs`
3. Implement `run()` function
4. Add tests
5. Update documentation

Example:

```rust
// crates/autoflow-cli/src/commands/doctor.rs
use anyhow::Result;
use colored::*;

pub async fn run() -> Result<()> {
    println!("{}", "🏥 Running diagnostics...".bright_cyan());

    // Check Rust
    check_rust()?;

    // Check Claude CLI
    check_claude()?;

    // Check git
    check_git()?;

    println!("{}", "✅ All checks passed!".bright_green());
    Ok(())
}

fn check_rust() -> Result<()> {
    // Implementation
}
```

### Adding a New Utility

1. Create module in `crates/autoflow-utils/src/`
2. Add to `lib.rs` exports
3. Add comprehensive tests
4. Document public APIs

Example:

```rust
// crates/autoflow-utils/src/validation.rs

/// Validate sprint YAML structure
pub fn validate_sprint_yaml(content: &str) -> Result<()> {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml() {
        let yaml = "project:\n  name: test";
        assert!(validate_sprint_yaml(yaml).is_ok());
    }
}
```

## Common Tasks

### Adding a New Agent

1. Create agent definition: `agents/my-agent.agent.md`
2. Update agent count in README
3. Add agent mapping in `executor.rs` if needed
4. Document in USER_GUIDE.md

### Fixing a Bug

1. Write a failing test that reproduces the bug
2. Fix the bug
3. Verify test passes
4. Add regression test if needed

### Improving Performance

1. Profile first: `cargo build --release && cargo flamegraph`
2. Identify bottleneck
3. Optimize
4. Measure improvement
5. Add benchmark if significant

## Questions?

- Check [USER_GUIDE.md](USER_GUIDE.md) for usage
- Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
- Check [ARCHITECTURE.md](ARCHITECTURE.md) for design
- Open a GitHub issue for help

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to AutoFlow!** 🚀
