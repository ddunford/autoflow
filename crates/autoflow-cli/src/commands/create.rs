use anyhow::{Context, Result};
use autoflow_agents::execute_agent;
use autoflow_utils::{extract_yaml_from_output, Paths};
use colored::*;
use std::fs;
use std::path::Path;

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

    // 6. Generate comprehensive documentation
    println!("{}", "📚 Generating project documentation...".bright_cyan());
    println!("  Spawning make-docs agent...");

    let docs_context = format!(r#"Generate comprehensive project documentation from this idea:

{}

Create the following files in .autoflow/docs/:
1. .autoflow/docs/BUILD_SPEC.md - Detailed technical specification
2. .autoflow/docs/ARCHITECTURE.md - System architecture and design
3. .autoflow/docs/API_SPEC.md - API endpoints and data models (if backend)
4. .autoflow/docs/UI_SPEC.md - UI/UX specifications and wireframes (if frontend)

Use the IDEA content to infer:
- Tech stack (or recommend best choices)
- Architecture patterns
- Database schema
- API design
- Component structure
- Testing strategy

IMPORTANT: All documentation files MUST be created in .autoflow/docs/ directory, NOT in the project root.
"#, idea_content);

    match execute_agent("make-docs", &docs_context, 15).await {
        Ok(result) => {
            if result.success {
                println!("  {} Documentation generated", "✓".green());
            } else {
                println!("  {} Agent completed with warnings", "⚠".yellow());
            }
        }
        Err(e) => {
            println!("  {} Failed to generate docs: {}", "⚠".yellow(), e);
            println!("  Creating minimal BUILD_SPEC.md...");

            // Fallback: create minimal BUILD_SPEC.md in .autoflow/docs/
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
    println!();

    // 7. Create project directory structure based on tech stack
    println!("{}", "📂 Creating project structure...".bright_cyan());

    // Determine tech stack from BUILD_SPEC
    let build_spec_content = fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
    let has_backend = build_spec_content.to_lowercase().contains("backend")
        || build_spec_content.to_lowercase().contains("laravel")
        || build_spec_content.to_lowercase().contains("php")
        || build_spec_content.to_lowercase().contains("node.js")
        || build_spec_content.to_lowercase().contains("api");
    let has_frontend = build_spec_content.to_lowercase().contains("frontend")
        || build_spec_content.to_lowercase().contains("react")
        || build_spec_content.to_lowercase().contains("vue")
        || build_spec_content.to_lowercase().contains("ui");

    if has_backend && has_frontend {
        // Monorepo structure
        fs::create_dir_all("backend/src")?;
        fs::create_dir_all("backend/tests")?;
        fs::create_dir_all("frontend/src")?;
        fs::create_dir_all("frontend/tests")?;
        println!("  {} Created backend/ and frontend/ directories", "✓".green());
    } else if has_backend {
        // Backend-only
        fs::create_dir_all("src")?;
        fs::create_dir_all("tests")?;
        println!("  {} Created src/ and tests/ directories", "✓".green());
    } else if has_frontend {
        // Frontend-only
        fs::create_dir_all("src")?;
        fs::create_dir_all("tests")?;
        println!("  {} Created src/ and tests/ directories", "✓".green());
    } else {
        // Generic structure
        fs::create_dir_all("src")?;
        fs::create_dir_all("tests")?;
        println!("  {} Created src/ and tests/ directories", "✓".green());
    }
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
    let integration_guide = fs::read_to_string(".autoflow/INTEGRATION_GUIDE.md").unwrap_or_default();

    let sprints_context = format!(r#"Generate a complete sprint plan from the following project documentation:

# BUILD_SPEC.md
{}

# ARCHITECTURE.md
{}

# API_SPEC.md
{}

# UI_SPEC.md
{}

# INTEGRATION_GUIDE.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Output ONLY raw YAML - no markdown fences, no explanations

The agent definition already contains the full YAML format. Just output the actual YAML content.
"#, build_spec, architecture, api_spec, ui_spec, integration_guide);

    match execute_agent("make-sprints", &sprints_context, 20).await {
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
    if has_backend && has_frontend {
        println!("  ├── backend/");
        println!("  │   ├── src/                # Backend source code");
        println!("  │   └── tests/              # Backend tests");
        println!("  ├── frontend/");
        println!("  │   ├── src/                # Frontend source code");
        println!("  │   └── tests/              # Frontend tests");
    } else {
        println!("  ├── src/                    # Source code");
        println!("  ├── tests/                  # Tests");
    }
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
