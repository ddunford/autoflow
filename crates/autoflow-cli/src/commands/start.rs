use anyhow::{bail, Context};
use autoflow_core::Orchestrator;
use autoflow_data::{SprintsYaml, SprintStatus};
use colored::*;
use std::path::Path;

pub async fn run(parallel: bool, sprint: Option<u32>) -> anyhow::Result<()> {
    println!("{}", "🚀 Starting AutoFlow...".bright_cyan().bold());

    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";

    // If SPRINTS.yml doesn't exist but IDEA.md does, initialize with empty sprints
    if !Path::new(sprints_path).exists() {
        if Path::new("IDEA.md").exists() {
            println!("\n{}", "Initializing project from IDEA.md...".bright_cyan());
            std::fs::create_dir_all(".autoflow/docs")?;

            let current_dir = std::env::current_dir()?;
            let project_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Project")
                .to_string();

            let empty_sprints = format!(
                "project:\n  name: \"{}\"\n  total_sprints: 0\n  current_sprint: null\n  last_updated: \"{}\"
\nsprints: []",
                project_name,
                chrono::Utc::now().to_rfc3339()
            );
            std::fs::write(sprints_path, empty_sprints)?;
        } else {
            bail!(
                "{}\nRun {} or {} first",
                "Project not initialized.".red(),
                "autoflow init".bright_blue(),
                "autoflow create".bright_blue()
            );
        }
    }

    // Load sprints
    println!("\n{}", "Checking project status...".bright_cyan());
    let mut sprints_data = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

    // Check if sprints are empty and IDEA.md exists - offer to generate
    if sprints_data.sprints.is_empty() {
        println!("{}", "No sprints found in SPRINTS.yml".yellow());

        if Path::new("IDEA.md").exists() {
            println!("{}", "Found IDEA.md - generating project setup...".bright_cyan());
            println!();

            // Check if docs exist
            let docs_exist = Path::new(".autoflow/docs/BUILD_SPEC.md").exists();

            if !docs_exist {
                println!("{}", "📚 Generating project documentation...".bright_cyan());
                println!("  Spawning make-docs agent...");

                let idea_content = std::fs::read_to_string("IDEA.md")?;
                let docs_context = format!(r#"Generate comprehensive project documentation from this idea:

{}

Create the following files in .autoflow/docs/:
1. .autoflow/docs/BUILD_SPEC.md - Detailed technical specification
2. .autoflow/docs/ARCHITECTURE.md - System architecture and design
3. .autoflow/docs/API_SPEC.md - API endpoints and data models (if backend)
4. .autoflow/docs/UI_SPEC.md - UI/UX specifications and wireframes (if frontend)

IMPORTANT: All documentation files MUST be created in the .autoflow/docs/ directory, NOT in the project root.
"#, idea_content);

                match autoflow_agents::execute_agent("make-docs", &docs_context, 15).await {
                    Ok(result) if result.success => {
                        println!("  {} Documentation generated", "✓".green());
                    }
                    _ => {
                        bail!("Failed to generate documentation. Please run 'autoflow create' instead.");
                    }
                }
                println!();
            }

            // Generate sprints from docs
            println!("{}", "📋 Generating sprint plan...".bright_cyan());
            println!("  Spawning make-sprints agent...");

            let build_spec = std::fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
            let architecture = std::fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
            let api_spec = std::fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
            let ui_spec = std::fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();

            let sprints_context = format!(r#"Generate a complete sprint plan from the following project documentation:

# BUILD_SPEC.md
{}

# ARCHITECTURE.md
{}

# API_SPEC.md
{}

# UI_SPEC.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Output ONLY raw YAML - no markdown fences, no explanations
"#, build_spec, architecture, api_spec, ui_spec);

            match autoflow_agents::execute_agent("make-sprints", &sprints_context, 20).await {
                Ok(result) if result.success => {
                    println!("  {} Sprint plan generated", "✓".green());

                    // Save sprints
                    let yaml_content = autoflow_utils::extract_yaml_from_output(&result.output);
                    std::fs::write(sprints_path, yaml_content)?;
                    println!("  {} Saved to {}", "✓".green(), sprints_path.bright_blue());

                    // Reload sprints
                    sprints_data = SprintsYaml::load(sprints_path)?;
                }
                _ => {
                    bail!("Failed to generate sprints. Please run 'autoflow create' instead.");
                }
            }
            println!();
        } else {
            bail!(
                "{}\n{}\n\nRun one of:\n  {} - If you have IDEA.md\n  {} - Manual setup",
                "No sprints found and no IDEA.md".red(),
                "Cannot continue without project requirements.".red(),
                "autoflow create".bright_blue(),
                "autoflow init".bright_blue()
            );
        }
    }

    println!("{}", "Loading sprints...".bright_cyan());

    // Filter sprints based on flags - get indices instead of refs
    let sprint_indices: Vec<usize> = if let Some(sprint_id) = sprint {
        // Run specific sprint
        println!("Running sprint: {}", sprint_id.to_string().bright_blue());

        let idx = sprints_data
            .sprints
            .iter()
            .position(|s| s.id == sprint_id)
            .context(format!("Sprint {} not found", sprint_id))?;

        vec![idx]
    } else {
        // Run all pending or in-progress sprints
        let runnable: Vec<usize> = sprints_data
            .sprints
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.status == SprintStatus::Pending
                    || (s.status != SprintStatus::Done && s.status != SprintStatus::Blocked)
            })
            .map(|(idx, _)| idx)
            .collect();

        if runnable.is_empty() {
            println!(
                "\n{}",
                "No runnable sprints found. All sprints are either complete or blocked."
                    .yellow()
            );
            return Ok(());
        }

        println!(
            "Running {} sprint(s)",
            runnable.len().to_string().bright_green()
        );
        runnable
    };

    // Create orchestrator
    let max_iterations = 50;
    let orchestrator = Orchestrator::new(max_iterations).with_save_callback({
        move |_sprint| {
            // TODO: Save just the updated sprint, not whole file
            // For now, we'll save after all iterations complete
            Ok(())
        }
    });

    // Execute sprints
    if parallel && sprint_indices.len() > 1 {
        println!("\n{}", "Mode: Parallel execution".bright_green());

        // Extract sprints to run
        let mut sprints_to_run: Vec<_> = sprint_indices
            .iter()
            .map(|&idx| sprints_data.sprints[idx].clone())
            .collect();

        // Run in parallel
        let results = orchestrator.run_parallel(&mut sprints_to_run).await?;

        // Update original sprints with results
        for (i, &idx) in sprint_indices.iter().enumerate() {
            sprints_data.sprints[idx] = sprints_to_run[i].clone();
        }

        // Check results
        for (i, result) in results.iter().enumerate() {
            match result {
                Ok(_) => {
                    println!(
                        "{} Sprint {} completed",
                        "✅".green(),
                        sprints_to_run[i].id
                    );
                }
                Err(e) => {
                    println!(
                        "{} Sprint {} failed: {}",
                        "❌".red(),
                        sprints_to_run[i].id,
                        e
                    );
                }
            }
        }
    } else {
        // Run sequentially
        println!("\n{}", "Mode: Sequential execution".bright_green());

        for &idx in &sprint_indices {
            let sprint = &mut sprints_data.sprints[idx];

            println!(
                "\n{} {} - {}",
                "Running Sprint".bright_cyan(),
                sprint.id.to_string().bright_blue(),
                sprint.goal.bright_white()
            );

            let sprint_id = sprint.id;
            match orchestrator.run_sprint(sprint).await {
                Ok(_) => {
                    println!(
                        "{} Sprint {} completed successfully",
                        "✅".green(),
                        sprint_id
                    );
                }
                Err(e) => {
                    println!(
                        "{} Sprint {} failed: {}",
                        "❌".red(),
                        sprint_id,
                        e
                    );

                    // Continue with next sprint unless it's a critical error
                    if sprint.status == SprintStatus::Blocked {
                        println!(
                            "{} Sprint {} is blocked, skipping remaining sprints",
                            "⚠️".yellow(),
                            sprint_id
                        );
                    }
                }
            }
        }
    }

    // Save updated sprints
    println!("\n{}", "Saving progress...".bright_cyan());
    sprints_data.save(sprints_path)
        .context("Failed to save SPRINTS.yml")?;

    // Display summary
    println!("\n{}", "Summary".bright_cyan().bold());
    println!("────────────────────────────────────────");

    let done = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status == SprintStatus::Done)
        .count();
    let blocked = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status == SprintStatus::Blocked)
        .count();
    let in_progress = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status != SprintStatus::Done
                 && s.status != SprintStatus::Blocked
                 && s.status != SprintStatus::Pending)
        .count();

    println!("{}: {}/{}", "Completed".green(), done, sprints_data.sprints.len());
    if in_progress > 0 {
        println!("{}: {}", "In Progress".blue(), in_progress);
    }
    if blocked > 0 {
        println!("{}: {}", "Blocked".red(), blocked);
    }

    println!("\n{}", "✨ AutoFlow session complete!".bright_green().bold());

    Ok(())
}
