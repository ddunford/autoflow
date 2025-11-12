use anyhow::{Context, Result};
use autoflow_agents::{execute_agent, execute_agent_with_retry};
use autoflow_utils::{extract_yaml_from_output, Paths};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn run(project_name: Option<String>, idea_file: Option<String>) -> Result<()> {
    println!("{}", "🚀 Creating new AutoFlow project...".bright_cyan().bold());
    println!();

    // Determine if we're creating a new directory or using current directory
    let (project_dir, project_name, use_current_dir) = if let Some(name) = project_name {
        // Create new directory with given name
        println!("{}", "📁 Creating project directory...".bright_cyan());
        if Path::new(&name).exists() {
            anyhow::bail!("Directory '{}' already exists", name);
        }
        fs::create_dir(&name)?;
        println!("  {} Created: {}", "✓".green(), name.bright_blue());
        println!();
        (name.clone(), name, false)
    } else {
        // Use current directory
        let current_dir = std::env::current_dir()?;
        let dir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        println!("{}", "📁 Using current directory...".bright_cyan());
        println!("  {} Project: {}", "✓".green(), dir_name.bright_blue());
        println!();
        (".".to_string(), dir_name, true)
    };

    // 2. Read IDEA.md (from file, current directory, or create template)
    let idea_content = if let Some(idea_path) = idea_file {
        println!("{}", "📖 Reading IDEA.md...".bright_cyan());
        let content = fs::read_to_string(&idea_path)
            .context(format!("Failed to read {}", idea_path))?;
        println!("  {} Read from: {}", "✓".green(), idea_path.bright_blue());
        content
    } else if use_current_dir {
        // Current directory mode - IDEA.md must exist
        if !Path::new("IDEA.md").exists() {
            anyhow::bail!(
                "IDEA.md not found in current directory.\n\n\
                Run one of:\n\
                  autoflow create <project-name>  # Create new project\n\
                  touch IDEA.md                    # Create IDEA.md first, then run 'autoflow create'"
            );
        }
        println!("{}", "📖 Reading IDEA.md...".bright_cyan());
        let content = fs::read_to_string("IDEA.md")
            .context("Failed to read IDEA.md")?;
        println!("  {} Found: {}", "✓".green(), "IDEA.md".bright_blue());
        content
    } else {
        // Create template IDEA.md for user to fill out
        let idea_template = r#"# Project Idea

## Overview
Describe your project in 2-3 sentences. What does it do? Who is it for?

## Features
List the main features you want:
- Feature 1
- Feature 2
- Feature 3

## Tech Stack (optional)
Specify your preferred technologies, or leave blank for AI to choose:
- Frontend: React / Vue / Angular / etc
- Backend: Node.js / Python / Go / Rust / etc
- Database: PostgreSQL / MySQL / MongoDB / etc
- Other: WebSockets, Redis, etc

## Requirements
Any specific requirements or constraints:
- Must work on mobile
- Need offline support
- Integration with existing API
- etc
"#;

        let idea_path = if use_current_dir {
            "IDEA.md".to_string()
        } else {
            format!("{}/IDEA.md", project_dir)
        };
        fs::write(&idea_path, idea_template)?;

        println!("{}", "📝 Created IDEA.md template".bright_cyan());
        println!("  {} Please edit: {}", "→".yellow(), idea_path.bright_blue());
        if use_current_dir {
            println!("  {} Then run: {}", "→".yellow(), "autoflow create".bright_blue());
        } else {
            println!("  {} Then run: {} {} {}",
                "→".yellow(),
                "autoflow create".bright_blue(),
                project_name.bright_blue(),
                "--idea IDEA.md".bright_blue()
            );
        }
        println!();
        return Ok(());
    };

    println!();

    // 3. Change to project directory (if we created a new one)
    if !use_current_dir {
        std::env::set_current_dir(&project_dir)?;
    }

    // 4. Initialize git repository
    println!("{}", "🔧 Initializing git repository...".bright_cyan());
    std::process::Command::new("git")
        .args(&["init"])
        .output()
        .context("Failed to initialize git repository")?;
    println!("  {} Git initialized", "✓".green());
    println!();

    // 5. Initialize AutoFlow
    println!("{}", "⚙️  Initializing AutoFlow...".bright_cyan());
    super::init::run(None).await?;
    println!();

    // 5.5. Create project-level Claude configuration
    println!("{}", "🤖 Setting up Claude configuration...".bright_cyan());
    fs::create_dir_all(".claude")?;

    let claude_config = r#"# AutoFlow Project Configuration

## Project Structure

**CRITICAL - Code Organization:**
```
project-root/
├── src/                    # ALL source code goes here
│   ├── backend/           # Backend services
│   ├── frontend/          # Frontend application
│   ├── shared/            # Shared utilities/types
│   └── ...                # Other source modules
├── tests/                 # ALL tests go here
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── .autoflow/             # AutoFlow metadata (DO NOT MODIFY)
│   ├── docs/             # Project specifications
│   └── SPRINTS.yml       # Sprint definitions
└── .claude/              # This configuration
```

**File Placement Rules:**
1. ✅ Code files → `src/`
2. ✅ Test files → `tests/`
3. ✅ Config files → project root (package.json, tsconfig.json, etc.)
4. ❌ NEVER create top-level code directories (`backend/`, `frontend/`, etc.)
5. ❌ NEVER create summary/report files unless explicitly required

## Development Guidelines

### Code Quality
- Write clean, maintainable code following language best practices
- Include inline comments for complex logic
- Follow consistent naming conventions
- Implement proper error handling

### Testing
- Write tests FIRST (TDD approach)
- Test files mirror source structure in `tests/` directory
- Aim for high coverage of critical paths
- Include unit, integration, and E2E tests as appropriate

### Documentation
- **In-code documentation**: Use JSDoc/docstrings for functions/classes
- **Technical specs**: Already in `.autoflow/docs/` - reference them
- **DO NOT CREATE**:
  - DEPLOYMENT_SUMMARY.md
  - SUMMARY.md, REPORT.md
  - README.md (unless explicitly required in sprint)
  - Any file with "summary" or "report" in the name

### Sprint Workflow
1. Read the current sprint from `.autoflow/SPRINTS.yml`
2. Reference specifications in `.autoflow/docs/` for implementation details
3. Create tests in `tests/` directory first
4. Implement code in `src/` directory
5. Ensure all tests pass before marking complete

## AutoFlow Integration

This project uses AutoFlow for autonomous development:
- Sprint definitions: `.autoflow/SPRINTS.yml`
- Technical specs: `.autoflow/docs/`
- Do not manually modify AutoFlow files unless debugging

## File Creation Checklist

Before creating any file, verify:
- [ ] Is it source code? → Must go in `src/`
- [ ] Is it a test? → Must go in `tests/`
- [ ] Is it explicitly required in the current sprint deliverables?
- [ ] Am I about to create a summary/report? → STOP, don't create it

## Your Role

You are implementing features defined in the sprint plan. Focus on:
1. **Working code** that passes tests
2. **Proper structure** following the rules above
3. **Quality implementation** that meets acceptance criteria

Not on creating documentation, summaries, or organizing files outside `src/` and `tests/`.
"#;

    fs::write(".claude/CLAUDE.md", claude_config)?;
    println!("  {} Created .claude/CLAUDE.md with project structure rules", "✓".green());
    println!();

    // 6. Generate comprehensive documentation (split into specialized agents)
    println!("{}", "📚 Generating project documentation...".bright_cyan());

    let base_context = format!(r#"Generate comprehensive project documentation from this idea:

{}

IMPORTANT: All documentation files MUST be created in .autoflow/docs/ directory, NOT in the project root.
"#, idea_content);

    // 6.1 Generate foundation docs (BUILD_SPEC, ARCHITECTURE with error handling)
    println!("  Spawning make-docs-foundation agent...");
    match execute_agent("make-docs-foundation", &base_context, 15, None).await {
        Ok(result) => {
            if result.success {
                println!("  {} Foundation docs generated (BUILD_SPEC, ARCHITECTURE)", "✓".green());
            } else {
                println!("  {} Foundation agent completed with warnings", "⚠".yellow());
            }
        }
        Err(e) => {
            println!("  {} Failed to generate foundation docs: {}", "⚠".yellow(), e);
            // Create minimal BUILD_SPEC as fallback
            let minimal_spec = format!(r#"# Build Specification

## Original Idea

{}

## Tech Stack
To be determined during sprint planning.

## Architecture
To be determined during implementation.
"#, idea_content);
            fs::write(".autoflow/docs/BUILD_SPEC.md", minimal_spec)?;
        }
    }

    // 6.2 Generate API docs (API_SPEC with data model and security)
    println!("  Spawning make-docs-api agent...");
    match execute_agent("make-docs-api", &base_context, 15, None).await {
        Ok(result) => {
            if result.success {
                println!("  {} API docs generated (API_SPEC with data model and security)", "✓".green());
            } else {
                println!("  {} API agent completed with warnings", "⚠".yellow());
            }
        }
        Err(e) => {
            println!("  {} API docs generation failed (may not be applicable): {}", "⚠".yellow(), e);
        }
    }

    // 6.3 Generate UI docs (UI_SPEC with state management, TESTING_STRATEGY)
    println!("  Spawning make-docs-ui agent...");
    match execute_agent_with_retry("make-docs-ui", &base_context, 15, None).await {
        Ok(result) => {
            if result.success {
                println!("  {} UI docs generated (UI_SPEC, TESTING_STRATEGY)", "✓".green());
            } else {
                println!("  {} UI agent failed - check .autoflow/.debug/ logs for details", "⚠".yellow());
                // Create minimal fallback if UI_SPEC doesn't exist
                if !Path::new(".autoflow/docs/UI_SPEC.md").exists() {
                    let minimal_ui = format!(r#"# UI Specification

## Original Idea

{}

## UI Pages
To be determined during implementation.

## Design System
To be determined during implementation.
"#, idea_content);
                    fs::write(".autoflow/docs/UI_SPEC.md", minimal_ui)?;
                }
                if !Path::new(".autoflow/docs/TESTING_STRATEGY.md").exists() {
                    let minimal_testing = r#"# Testing Strategy

## Framework Choices
- Unit: Vitest/Jest with React Testing Library
- E2E: Playwright/Cypress
- Backend: PHPUnit/Pest

## Coverage Requirements
- Overall: 80% minimum
- Critical paths: 100%

## What to Test
- Unit: Business logic, utilities, components, hooks
- Integration: API endpoints, database ops, auth flows
- E2E: Critical user flows
"#;
                    fs::write(".autoflow/docs/TESTING_STRATEGY.md", minimal_testing)?;
                }
            }
        }
        Err(e) => {
            println!("  {} UI docs generation failed: {}", "⚠".yellow(), e);
            println!("  {} Creating minimal fallback docs...", "→".yellow());
            // Create minimal fallback docs
            let minimal_ui = format!(r#"# UI Specification

## Original Idea

{}

## UI Pages
To be determined during implementation.

## Design System
To be determined during implementation.
"#, idea_content);
            fs::write(".autoflow/docs/UI_SPEC.md", minimal_ui)?;

            let minimal_testing = r#"# Testing Strategy

## Framework Choices
- Unit: Vitest/Jest with React Testing Library
- E2E: Playwright/Cypress
- Backend: PHPUnit/Pest

## Coverage Requirements
- Overall: 80% minimum
- Critical paths: 100%

## What to Test
- Unit: Business logic, utilities, components, hooks
- Integration: API endpoints, database ops, auth flows
- E2E: Critical user flows
"#;
            fs::write(".autoflow/docs/TESTING_STRATEGY.md", minimal_testing)?;
        }
    }

    println!();

    // 7. Create project directory structure
    println!("{}", "📂 Creating project structure...".bright_cyan());

    // Always use a simple src/ and tests/ structure
    fs::create_dir_all("src")?;
    fs::create_dir_all("tests")?;
    println!("  {} Created src/ and tests/ directories", "✓".green());
    println!();

    // 8. Analyze and create integration guide (if applicable)
    println!("{}", "🔍 Analyzing project structure...".bright_cyan());
    super::analyze::run().await?;
    println!();

    // 9. Generate sprint plan FROM DOCUMENTATION
    println!("{}", "📋 Generating sprint plan from documentation...".bright_cyan());
    println!("  Spawning make-sprints agent...");

    // Read the generated documentation files from .autoflow/docs/
    let build_spec = fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
    let architecture = fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
    let api_spec = fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
    let ui_spec = fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();
    let data_model = fs::read_to_string(".autoflow/docs/DATA_MODEL.md").unwrap_or_default();
    let testing_strategy = fs::read_to_string(".autoflow/docs/TESTING_STRATEGY.md").unwrap_or_default();
    let error_handling = fs::read_to_string(".autoflow/docs/ERROR_HANDLING.md").unwrap_or_default();
    let state_management = fs::read_to_string(".autoflow/docs/STATE_MANAGEMENT.md").unwrap_or_default();
    let security = fs::read_to_string(".autoflow/docs/SECURITY.md").unwrap_or_default();
    let deployment = fs::read_to_string(".autoflow/docs/DEPLOYMENT.md").unwrap_or_default();
    let integration_guide = fs::read_to_string(".autoflow/INTEGRATION_GUIDE.md").unwrap_or_default();

    // Load JSON schema - try global location first, then embedded
    let json_schema = {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let schema_path = PathBuf::from(home).join(".autoflow/schemas/sprints.schema.json");

        if let Ok(content) = fs::read_to_string(&schema_path) {
            content
        } else {
            // Fall back to embedded schema (compiled into binary)
            include_str!("../../schemas/sprints.schema.json").to_string()
        }
    };

    let sprints_context = format!(r#"Generate a complete sprint plan from the following project documentation:

# JSON SCHEMA (CRITICAL - MUST FOLLOW EXACTLY)

Your output MUST validate against this JSON schema:

```json
{}
```

IMPORTANT SCHEMA REQUIREMENTS:
- All required fields MUST be present
- All enum values must match EXACTLY (case-sensitive, use SCREAMING_SNAKE_CASE)
- Valid task types: IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR, BUGFIX
- Valid workflow types: IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR
- Valid sprint statuses: PENDING, WRITE_UNIT_TESTS, WRITE_CODE, CODE_REVIEW, REVIEW_FIX, RUN_UNIT_TESTS, UNIT_FIX, WRITE_E2E_TESTS, RUN_E2E_TESTS, E2E_FIX, COMPLETE, DONE, BLOCKED
- All sprints must start with status: PENDING
- Include last_updated timestamp in ISO 8601 format
- CRITICAL: dependencies MUST be an array of strings (sprint IDs), NOT maps/objects
  CORRECT:   dependencies: ["1", "2"]
  WRONG:     dependencies: [{{Sprint 1: Infrastructure}}]
  WRONG:     dependencies: [Sprint 1: Infrastructure]

# PROJECT DOCUMENTATION

Generate a complete sprint plan from the following project documentation:

# BUILD_SPEC.md
{}

# ARCHITECTURE.md
{}

# API_SPEC.md
{}

# UI_SPEC.md
{}

# DATA_MODEL.md
{}

# TESTING_STRATEGY.md
{}

# ERROR_HANDLING.md
{}

# STATE_MANAGEMENT.md
{}

# SECURITY.md
{}

# DEPLOYMENT.md
{}

# INTEGRATION_GUIDE.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Reference specific sections from the docs (e.g., "See DATA_MODEL.md#UserSchema")
6. Output ONLY raw YAML - no markdown fences, no explanations

The agent definition already contains the full YAML format. Just output the actual YAML content.
"#, json_schema, build_spec, architecture, api_spec, ui_spec, data_model, testing_strategy, error_handling, state_management, security, deployment, integration_guide);

    match execute_agent("make-sprints", &sprints_context, 20, None).await {
        Ok(result) => {
            if result.success {
                println!("  {} Sprint plan generated", "✓".green());

                // Save the output to SPRINTS.yml (parse and validate)
                let yaml_content = extract_yaml_from_output(&result.output);
                fs::write(Paths::SPRINTS_YML, yaml_content)?;
                println!("  {} Saved to {}", "✓".green(), Paths::SPRINTS_YML.bright_blue());
            } else {
                println!("  {} Failed to generate sprints", "✗".red());
                anyhow::bail!("Sprint generation failed");
            }
        }
        Err(e) => {
            println!("  {} Failed to generate sprints: {}", "✗".red(), e);
            anyhow::bail!("Sprint generation failed");
        }
    }
    println!();

    // 10. Summary
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  ✅ Project Created Successfully!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    println!("{}", "📂 Project Structure:".bright_cyan());
    println!("  {}/", project_name.bright_blue());
    println!("  ├── src/                    # Source code");
    println!("  ├── tests/                  # Tests");
    println!("  ├── .autoflow/");
    println!("  │   ├── docs/");
    println!("  │   │   ├── BUILD_SPEC.md   # Technical specification");
    println!("  │   │   ├── ARCHITECTURE.md # System architecture");
    println!("  │   │   ├── API_SPEC.md     # API documentation");
    println!("  │   │   └── UI_SPEC.md      # UI specifications");
    println!("  │   ├── SPRINTS.yml         # Sprint plan");
    println!("  │   └── CLAUDE.md           # Project context");
    println!("  └── .git/                   # Git repository");
    println!();

    // Count sprints
    let sprints_content = fs::read_to_string(Paths::SPRINTS_YML)?;
    let sprint_count = sprints_content.matches("- id:").count();

    println!("{}", "📊 Project Stats:".bright_cyan());
    println!("  Total Sprints: {}", sprint_count.to_string().bright_blue());
    println!("  Status: {}", "Ready to start".bright_green());
    println!();

    println!("{}", "🚀 Next Steps:".bright_cyan());
    println!("  1. Review the generated files:");
    println!("     {} {}", "cd".bright_blue(), project_name.bright_blue());
    println!("     {} .autoflow/docs/BUILD_SPEC.md", "cat".bright_blue());
    println!("     {} .autoflow/docs/ARCHITECTURE.md", "cat".bright_blue());
    println!("     {} .autoflow/SPRINTS.yml", "cat".bright_blue());
    println!();
    println!("  2. Start autonomous development:");
    println!("     {} {}", "autoflow start --parallel".bright_green(), "(recommended)");
    println!("     {} {}", "autoflow start".bright_blue(), "(sequential)");
    println!();
    println!("  3. Monitor progress:");
    println!("     {}", "autoflow status".bright_blue());
    println!("     {}", "autoflow sprints list".bright_blue());
    println!();

    Ok(())
}
