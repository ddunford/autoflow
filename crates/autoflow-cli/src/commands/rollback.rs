use anyhow::{bail, Context};
use autoflow_data::{SprintsYaml, SprintStatus};
use autoflow_git::WorktreeManager;
use colored::*;
use std::path::Path;

pub async fn run(sprint: Option<u32>) -> anyhow::Result<()> {
    println!("{}", "⏪ Rolling back sprint...".bright_cyan().bold());

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
    let mut sprints_data = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

    // Determine which sprint to rollback
    let sprint_id = if let Some(id) = sprint {
        id
    } else {
        // Find the most recent non-done sprint
        sprints_data
            .sprints
            .iter()
            .filter(|s| s.status != SprintStatus::Done && s.status != SprintStatus::Pending)
            .max_by_key(|s| s.id)
            .map(|s| s.id)
            .ok_or_else(|| anyhow::anyhow!("No active sprint to rollback"))?
    };

    println!("Sprint ID: {}", sprint_id.to_string().bright_blue());

    // Find the sprint
    let sprint = sprints_data
        .sprints
        .iter_mut()
        .find(|s| s.id == sprint_id)
        .ok_or_else(|| anyhow::anyhow!("Sprint {} not found", sprint_id))?;

    println!("Sprint goal: {}", sprint.goal.bright_blue());
    println!("Current status: {}", format!("{:?}", sprint.status).bright_yellow());

    // Check if git repository exists
    if Path::new(".git").exists() {
        println!("\n{}", "Checking for worktree...".bright_cyan());

        let manager = WorktreeManager::new(".")
            .context("Failed to open git repository")?;

        let worktree_name = format!("sprint-{}", sprint_id);

        // Try to delete the worktree
        match manager.delete_worktree(&worktree_name) {
            Ok(_) => {
                println!("  {} Worktree deleted: {}", "✓".green(), worktree_name.bright_blue());
            }
            Err(e) => {
                println!("  {} No worktree found: {}", "ℹ".blue(), e);
            }
        }

        // Prune any stale worktree references
        let _ = manager.prune_worktrees();
    }

    // Reset sprint status
    println!("\n{}", "Resetting sprint status...".bright_cyan());
    sprint.status = SprintStatus::Pending;
    sprint.started = None;
    sprint.completed_at = None;
    sprint.last_updated = chrono::Utc::now();

    // Reset task statuses
    for task in &mut sprint.tasks {
        task.status = autoflow_data::TaskStatus::Pending;
        task.committed_at = None;
        task.reviewed_at = None;
        task.tested_at = None;
        task.done_at = None;
        task.git_commit = None;
    }

    // Save updated sprints
    sprints_data.save(sprints_path)
        .context("Failed to save SPRINTS.yml")?;

    println!("\n{} {}", "✅".green(), "Sprint rolled back successfully!".bright_green());

    println!("\n{}", "Summary:".bright_cyan());
    println!("  Sprint status: {} → {}", "In Progress".yellow(), "Pending".green());
    println!("  Worktree: {}", "Deleted (if existed)".bright_blue());
    println!("  Tasks: {}", "Reset to Pending".bright_blue());

    println!("\n{}", "Next steps:".bright_cyan());
    println!("  1. Review the reset sprint in {}", sprints_path.bright_blue());
    println!("  2. Run {} to restart development", "autoflow start".bright_blue());
    println!("  3. Or modify the sprint before starting");

    Ok(())
}
