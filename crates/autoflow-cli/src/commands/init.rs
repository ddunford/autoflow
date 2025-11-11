use autoflow_utils::{check_for_updates, should_check_for_updates, prompt_and_update, update_check_timestamp};
use colored::*;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

pub async fn run(template: Option<String>) -> anyhow::Result<()> {
    println!("{}", "📦 Initializing AutoFlow project...".bright_cyan().bold());

    // Check for updates (if enabled and interval has passed)
    if should_check_for_updates().unwrap_or(false) {
        match check_for_updates() {
            Ok(info) if info.has_updates() => {
                // Prompt user and update if they accept
                prompt_and_update(&info)?;
            }
            Ok(_) => {
                // No updates, just update timestamp
                update_check_timestamp()?;
            }
            Err(_) => {
                // Silently ignore update check failures
            }
        }
    }

    // Check if already initialized
    if Path::new(".autoflow").exists() {
        warn!("Project already initialized!");
        println!("\n{}", "Already initialized. Remove .autoflow/ to reinitialize.".yellow());
        return Ok(());
    }

    if let Some(t) = &template {
        info!("Using template: {}", t);
        println!("Template: {}", t.bright_blue());
    }

    // Create directory structure
    println!("\n{}", "Creating directory structure...".bright_white());
    fs::create_dir_all(".autoflow/docs")?;
    fs::create_dir_all(".claude")?;
    info!("✓ Directories created");

    // Copy SPRINTS.yml template
    println!("{}", "Creating SPRINTS.yml...".bright_white());
    let sprints_template = include_str!("../../../../templates/SPRINTS.template.yml");
    fs::write(".autoflow/SPRINTS.yml", sprints_template)?;
    info!("✓ SPRINTS.yml created");

    // Copy CLAUDE.md template
    println!("{}", "Creating CLAUDE.md...".bright_white());
    let claude_template = include_str!("../../../../templates/CLAUDE.template.md");
    fs::write(".claude/CLAUDE.md", claude_template)?;
    info!("✓ CLAUDE.md created");

    // Copy settings.json template
    println!("{}", "Creating .claude/settings.json...".bright_white());
    let settings_template = include_str!("../../../../templates/.claude/settings.json.template");
    fs::write(".claude/settings.json", settings_template)?;
    info!("✓ settings.json created");

    // Create .gitignore if it doesn't exist
    if !Path::new(".gitignore").exists() {
        println!("{}", "Creating .gitignore...".bright_white());
        let gitignore = r#"
# AutoFlow
.autoflow/.debug/
.autoflow/.failures/

# Environment
.env
.env.local

# Dependencies
node_modules/
vendor/
target/

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
Thumbs.db
"#;
        fs::write(".gitignore", gitignore.trim_start())?;
        info!("✓ .gitignore created");
    }

    // Success!
    println!("\n{}", "✅ Project initialized successfully!".green().bold());
    println!("\n{}", "Next steps:".bright_white().bold());
    println!("  1. Create {} with your project requirements", "BUILD_SPEC.md".bright_blue());
    println!("  2. Run {} to generate design docs and sprints", "autoflow start".bright_blue());
    println!("\n{}", "Directory structure:".bright_white());
    println!("  {}  - Sprint definitions", ".autoflow/SPRINTS.yml".bright_cyan());
    println!("  {}      - Design documentation", ".autoflow/docs/".bright_cyan());
    println!("  {}       - Claude configuration", ".claude/CLAUDE.md".bright_cyan());

    Ok(())
}
