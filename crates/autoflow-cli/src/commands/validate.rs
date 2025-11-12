use anyhow::{Context, Result};
use autoflow_data::SprintsYaml;
use colored::*;
use std::fs;
use std::path::Path;

pub async fn run(infrastructure: bool, integration: bool, fix: bool) -> Result<()> {
    // If no specific validation type is specified, validate SPRINTS.yml
    if !infrastructure && !integration {
        return validate_sprints(fix);
    }

    // TODO: Add infrastructure and integration validation
    if infrastructure {
        println!("{}", "Infrastructure validation not yet implemented".yellow());
    }
    if integration {
        println!("{}", "Integration validation not yet implemented".yellow());
    }

    Ok(())
}

fn validate_sprints(fix: bool) -> Result<()> {
    let sprints_path = ".autoflow/SPRINTS.yml";

    if !Path::new(sprints_path).exists() {
        anyhow::bail!("SPRINTS.yml not found. Run 'autoflow create' first.");
    }

    println!("{}", "🔍 Validating SPRINTS.yml...".bright_cyan());
    println!();

    // Read the current file
    let content = fs::read_to_string(sprints_path)
        .context("Failed to read SPRINTS.yml")?;

    // Try to parse as-is
    match SprintsYaml::load(sprints_path) {
        Ok(_) => {
            println!("{}", "✅ SPRINTS.yml is valid!".bright_green());
            println!();
            Ok(())
        }
        Err(e) => {
            println!("{}", "❌ Validation failed:".bright_red());
            println!("   {}", e.to_string().red());
            println!();

            if fix {
                println!("{}", "🔧 Attempting to fix...".bright_yellow());

                // Try to validate and fix
                match SprintsYaml::validate_and_fix(&content) {
                    Ok(fixed_sprints) => {
                        // Backup the original
                        let backup_path = format!("{}.backup-{}", sprints_path, chrono::Utc::now().format("%Y%m%d-%H%M%S"));
                        fs::copy(sprints_path, &backup_path)?;
                        println!("   {} Backed up to {}", "✓".green(), backup_path.bright_blue());

                        // Save the fixed version
                        fixed_sprints.save(sprints_path)?;
                        println!("   {} Fixed and saved", "✓".green());
                        println!();
                        println!("{}", "✅ SPRINTS.yml has been fixed!".bright_green());
                        println!();
                        Ok(())
                    }
                    Err(fix_err) => {
                        println!();
                        println!("{}", "❌ Could not automatically fix the file.".bright_red());
                        println!("   Error: {}", fix_err.to_string().red());
                        println!();
                        println!("Suggestions:");
                        println!("  1. Check the schema in crates/autoflow-data/src/sprints.rs");
                        println!("  2. Ensure all required fields are present:");
                        println!("     - project.last_updated (DateTime)");
                        println!("     - project.current_sprint (Option)");
                        println!("     - sprint.last_updated (DateTime)");
                        println!("     - sprint.started (Option)");
                        println!("     - sprint.completed_at (Option)");
                        println!();
                        Err(fix_err.into())
                    }
                }
            } else {
                println!("Run {} to automatically fix the file.", "autoflow validate --fix".bright_blue());
                println!();
                Err(e.into())
            }
        }
    }
}
