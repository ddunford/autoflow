use anyhow::{bail, Context};
use autoflow_core::CodebaseAnalyzer;
use colored::*;
use std::path::Path;

pub async fn run() -> anyhow::Result<()> {
    println!("{}", "🔍 Analyzing codebase...".bright_cyan().bold());

    // Check if project is initialized
    if !Path::new(".autoflow").exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    // Get current directory
    let project_root = std::env::current_dir()
        .context("Failed to get current directory")?;

    println!("\nProject: {}", project_root.display().to_string().bright_blue());

    // Create analyzer
    let analyzer = CodebaseAnalyzer::new(&project_root);

    println!("\n{}", "Scanning project structure...".bright_cyan());

    // Run analysis
    let analysis = analyzer.analyze()
        .context("Failed to analyze codebase")?;

    // Display results
    println!("\n{}", "Tech Stack".bright_green().bold());
    println!("  Language: {}", analysis.tech_stack.language.bright_blue());
    if let Some(version) = &analysis.tech_stack.version {
        println!("  Version: {}", version.bright_blue());
    }
    println!("  Package Manager: {}", analysis.tech_stack.package_manager.bright_blue());

    if !analysis.frameworks.is_empty() {
        println!("\n{}", "Frameworks".bright_green().bold());
        for framework in &analysis.frameworks {
            print!("  • {} ({})", framework.name.bright_blue(), framework.framework_type);
            if let Some(version) = &framework.version {
                print!(" - v{}", version);
            }
            println!();
        }
    }

    println!("\n{}", "Project Structure".bright_green().bold());
    if let Some(src) = &analysis.structure.source_dir {
        println!("  Source: {}", src.bright_blue());
    }
    if let Some(tests) = &analysis.structure.test_dir {
        println!("  Tests: {}", tests.bright_blue());
    }
    if let Some(config) = &analysis.structure.config_dir {
        println!("  Config: {}", config.bright_blue());
    }
    if !analysis.structure.entry_points.is_empty() {
        println!("  Entry Points:");
        for entry in &analysis.structure.entry_points {
            println!("    - {}", entry.bright_blue());
        }
    }

    if !analysis.integration_points.is_empty() {
        println!("\n{}", "Integration Points".bright_green().bold());
        for point in &analysis.integration_points {
            println!("  • {} ({})", point.name.bright_blue(), point.point_type);
            println!("    Files: {}", point.files.len());
        }
    }

    // Save to INTEGRATION_GUIDE.md
    let guide_path = ".autoflow/INTEGRATION_GUIDE.md";
    println!("\n{}", "Saving analysis...".bright_cyan());
    analysis.save(guide_path)
        .context("Failed to save integration guide")?;

    println!("{} {}", "✅".green(), format!("Analysis saved to {}", guide_path).bright_green());

    println!("\n{}", "Next steps:".bright_cyan());
    println!("  1. Review {} for integration patterns", guide_path.bright_blue());
    println!("  2. Use {} to add new features", "autoflow add".bright_blue());

    Ok(())
}
