use anyhow::{bail, Context};
use autoflow_data::{SprintsYaml, SprintStatus};
use colored::*;
use std::path::Path;

pub async fn run(cmd: crate::SprintsCommands) -> anyhow::Result<()> {
    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";
    if !Path::new(sprints_path).exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    match cmd {
        crate::SprintsCommands::List => list_sprints(sprints_path).await,
        crate::SprintsCommands::Show { id, integration } => show_sprint(sprints_path, id, integration).await,
        crate::SprintsCommands::Create => create_sprint().await,
    }
}

async fn list_sprints(sprints_path: &str) -> anyhow::Result<()> {
    println!("{}", "📋 Listing sprints...".bright_cyan().bold());

    let sprints = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

    println!("\n{}: {}", "Project".bold(), sprints.project.name.bright_blue());
    println!("{}: {}", "Total Sprints".bold(), sprints.project.total_sprints);
    println!();

    if sprints.sprints.is_empty() {
        println!("{}", "No sprints found.".yellow());
        return Ok(());
    }

    // Group sprints by status
    let done: Vec<_> = sprints.sprints.iter().filter(|s| s.status == SprintStatus::Done).collect();
    let in_progress: Vec<_> = sprints.sprints.iter().filter(|s|
        s.status != SprintStatus::Done &&
        s.status != SprintStatus::Pending &&
        s.status != SprintStatus::Blocked
    ).collect();
    let blocked: Vec<_> = sprints.sprints.iter().filter(|s| s.status == SprintStatus::Blocked).collect();
    let pending: Vec<_> = sprints.sprints.iter().filter(|s| s.status == SprintStatus::Pending).collect();

    if !in_progress.is_empty() {
        println!("{}", "In Progress:".bright_green().bold());
        for sprint in &in_progress {
            println!("  {} - {} ({})",
                format!("Sprint {}", sprint.id).bright_blue(),
                sprint.goal,
                format!("{:?}", sprint.status).bright_yellow()
            );
        }
        println!();
    }

    if !pending.is_empty() {
        println!("{}", "Pending:".yellow().bold());
        for sprint in &pending {
            println!("  {} - {}",
                format!("Sprint {}", sprint.id).bright_blue(),
                sprint.goal
            );
        }
        println!();
    }

    if !blocked.is_empty() {
        println!("{}", "Blocked:".red().bold());
        for sprint in &blocked {
            println!("  {} - {} (retries: {})",
                format!("Sprint {}", sprint.id).bright_blue(),
                sprint.goal,
                sprint.blocked_count.unwrap_or(0)
            );
        }
        println!();
    }

    if !done.is_empty() {
        println!("{}", "Completed:".green().bold());
        for sprint in &done {
            println!("  {} - {}",
                format!("Sprint {}", sprint.id).bright_blue(),
                sprint.goal
            );
        }
        println!();
    }

    // Summary
    println!("{}", "Summary:".bold());
    println!("  {}: {}/{}", "Completed".green(), done.len(), sprints.sprints.len());
    println!("  {}: {}", "In Progress".blue(), in_progress.len());
    println!("  {}: {}", "Pending".yellow(), pending.len());
    println!("  {}: {}", "Blocked".red(), blocked.len());

    Ok(())
}

async fn show_sprint(sprints_path: &str, id: u32, integration: bool) -> anyhow::Result<()> {
    println!("{}", format!("📋 Sprint {} Details", id).bright_cyan().bold());

    let sprints = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

    let sprint = sprints.sprints.iter()
        .find(|s| s.id == id)
        .context(format!("Sprint {} not found", id))?;

    println!();
    println!("{}: {}", "ID".bold(), sprint.id.to_string().bright_blue());
    println!("{}: {}", "Goal".bold(), sprint.goal.bright_white());

    let status_colored = match sprint.status {
        SprintStatus::Done => format!("{:?}", sprint.status).green(),
        SprintStatus::Blocked => format!("{:?}", sprint.status).red(),
        SprintStatus::Pending => format!("{:?}", sprint.status).yellow(),
        _ => format!("{:?}", sprint.status).bright_blue(),
    };
    println!("{}: {}", "Status".bold(), status_colored);

    println!("{}: {}", "Duration".bold(), sprint.duration.as_ref().unwrap_or(&"N/A".to_string()));
    println!("{}: {}", "Total Effort".bold(), sprint.total_effort.bright_blue());
    println!("{}: {}", "Max Effort".bold(), sprint.max_effort.bright_blue());

    if let Some(started) = sprint.started {
        println!("{}: {}", "Started".bold(), started.format("%Y-%m-%d %H:%M"));
    }

    if let Some(completed) = sprint.completed_at {
        println!("{}: {}", "Completed".bold(), completed.format("%Y-%m-%d %H:%M").to_string().green());
    }

    if let Some(blocked_count) = sprint.blocked_count {
        println!("{}: {}", "Blocked Count".bold(), blocked_count.to_string().red());
    }

    println!();
    println!("{}", "Deliverables:".bold());
    for deliverable in &sprint.deliverables {
        println!("  • {}", deliverable);
    }

    println!();
    println!("{} ({}):", "Tasks".bold(), sprint.tasks.len());
    for task in &sprint.tasks {
        println!("  {} - {} ({})",
            task.id.bright_blue(),
            task.title,
            task.effort
        );
        println!("    Priority: {:?}", task.priority);
        println!("    Business Rules: {}", task.business_rules.len());
    }

    if !sprint.dependencies.is_empty() {
        println!();
        println!("{}", "Dependencies:".bold());
        for dep in &sprint.dependencies {
            println!("  • {}", dep.bright_yellow());
        }
    }

    if integration {
        if let Some(points) = &sprint.integration_points {
            println!();
            println!("{}", "Integration Points:".bold());

            if !points.modifies.is_empty() {
                println!("  Modifies:");
                for file in &points.modifies {
                    println!("    • {}", file.bright_blue());
                }
            }

            if !points.creates.is_empty() {
                println!("  Creates:");
                for file in &points.creates {
                    println!("    • {}", file.bright_green());
                }
            }

            if !points.tests_existing.is_empty() {
                println!("  Tests (existing):");
                for test in &points.tests_existing {
                    println!("    • {}", test.bright_yellow());
                }
            }

            if !points.patterns.is_empty() {
                println!("  Patterns:");
                for pattern in &points.patterns {
                    println!("    • {}", pattern);
                }
            }
        } else {
            println!();
            println!("{}", "No integration points defined.".yellow());
        }
    }

    Ok(())
}

async fn create_sprint() -> anyhow::Result<()> {
    println!("{}", "📋 Creating sprint...".bright_cyan().bold());
    println!("\n{}", "💡 Tip:".bright_yellow());
    println!("  Use {} to add a new feature", "autoflow add \"Feature Description\"".bright_blue());
    println!("  This will automatically create a sprint");
    Ok(())
}
