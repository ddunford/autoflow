use anyhow::{bail, Context};
use autoflow_core::Orchestrator;
use autoflow_data::{SprintsYaml, SprintStatus};
use colored::*;
use std::path::Path;

pub async fn run(parallel: bool, sprint: Option<u32>) -> anyhow::Result<()> {
    println!("{}", "🚀 Starting AutoFlow...".bright_cyan().bold());

    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";
    if !Path::new(sprints_path).exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    // Load sprints
    println!("\n{}", "Loading sprints...".bright_cyan());
    let mut sprints_data = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

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
