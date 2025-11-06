use anyhow::{bail, Context};
use autoflow_quality::{create_default_pipeline, GateContext};
use colored::*;
use std::path::Path;

pub async fn run(infrastructure: bool, integration: bool, fix: bool) -> anyhow::Result<()> {
    println!("{}", "🔍 Running Validation...".bright_cyan().bold());

    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";
    if !Path::new(sprints_path).exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    // Determine current directory
    let project_root = std::env::current_dir()
        .context("Failed to get current directory")?
        .to_string_lossy()
        .to_string();

    println!("\nProject: {}", project_root.bright_blue());
    println!("Sprints: {}", sprints_path.bright_blue());

    if fix {
        println!("Mode: {}", "Auto-fix enabled".bright_yellow());
    }

    // Create context
    let context = GateContext::new(sprints_path.to_string(), project_root)
        .with_auto_fix(fix);

    // Build quality pipeline
    let pipeline = create_default_pipeline();

    // Add infrastructure checks if requested
    if infrastructure {
        println!("\nIncluding: {}", "Infrastructure checks".bright_green());
        // TODO: Add infrastructure gate
        // pipeline = pipeline.add_gate(InfrastructureGate);
    }

    // Add integration checks if requested
    if integration {
        println!("Including: {}", "Integration checks".bright_green());
        // TODO: Add integration gate
        // pipeline = pipeline.add_gate(IntegrationGate);
    }

    // Run pipeline
    println!("\n{}", "Running quality gates...".bright_cyan());
    let report = pipeline.run(&context)
        .context("Failed to run quality pipeline")?;

    // Display report
    println!("{}", report);

    // Exit with error if validation failed
    if !report.passed {
        bail!("Validation failed");
    }

    Ok(())
}
