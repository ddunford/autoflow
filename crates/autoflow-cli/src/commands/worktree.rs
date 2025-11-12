use anyhow::{bail, Context};
use autoflow_git::WorktreeManager;
use colored::*;
use std::path::Path;

pub async fn run(cmd: crate::WorktreeCommands) -> anyhow::Result<()> {
    // Check if we're in a git repository
    if !Path::new(".git").exists() {
        bail!("{}", "Not a git repository. Run 'git init' first.".red());
    }

    let manager = WorktreeManager::new(".")
        .context("Failed to open git repository")?;

    match cmd {
        crate::WorktreeCommands::List { r#type } => list_worktrees(&manager, r#type).await,
        crate::WorktreeCommands::Create { branch } => create_worktree(&manager, &branch).await,
        crate::WorktreeCommands::Merge { branch } => merge_worktree(&manager, &branch).await,
        crate::WorktreeCommands::Delete { branch, force } => delete_worktree(&manager, &branch, force).await,
        crate::WorktreeCommands::Prune => prune_worktrees(&manager).await,
    }
}

async fn list_worktrees(manager: &WorktreeManager, filter_type: Option<String>) -> anyhow::Result<()> {
    println!("{}", "🌳 Listing worktrees...".bright_cyan().bold());

    let mut worktrees = manager.list_worktrees()
        .context("Failed to list worktrees")?;

    // Filter by type if specified
    if let Some(wtype) = filter_type {
        worktrees.retain(|w| {
            match wtype.as_str() {
                "sprint" => w.name.starts_with("sprint-"),
                "bugfix" => w.name.starts_with("bugfix-"),
                _ => true,
            }
        });
    }

    if worktrees.is_empty() {
        println!("\n{}", "No worktrees found.".yellow());
        return Ok(());
    }

    println!("\n{} worktree(s) found:\n", worktrees.len());

    for worktree in worktrees {
        println!("{}", "────────────────────────────────────────".bright_black());
        println!("{}: {}", "Name".bold(), worktree.name.bright_blue());
        println!("{}: {}", "Branch".bold(), worktree.branch.bright_green());
        println!("{}: {}", "Path".bold(), worktree.display_path());
        println!("{}: {}", "Port".bold(), worktree.port.to_string().bright_yellow());
        println!();
    }

    Ok(())
}

async fn create_worktree(manager: &WorktreeManager, branch: &str) -> anyhow::Result<()> {
    println!("{}", "🌳 Creating worktree...".bright_cyan().bold());
    println!("Branch: {}", branch.bright_blue());

    // Extract sprint ID from branch name (e.g., "sprint-2" -> 2)
    let sprint_id = if branch.starts_with("sprint-") {
        branch.trim_start_matches("sprint-")
            .parse::<u32>()
            .context("Invalid sprint ID in branch name")?
    } else {
        bail!("Branch name must start with 'sprint-' (e.g., 'sprint-2')");
    };

    let worktree = manager.create_worktree(sprint_id, branch)
        .context("Failed to create worktree")?;

    // Setup environment
    println!("\n{}", "Setting up environment...".bright_cyan());
    manager.setup_worktree_env(&worktree)
        .context("Failed to setup worktree environment")?;

    println!("\n{}", "✅ Worktree created successfully!".green().bold());
    println!("\n{}", "Worktree Details:".bold());
    println!("  Name: {}", worktree.name.bright_blue());
    println!("  Path: {}", worktree.display_path());
    println!("  Branch: {}", worktree.branch.bright_green());
    println!("  Port: {}", worktree.port.to_string().bright_yellow());

    println!("\n{}", "Next steps:".bright_cyan());
    println!("  1. cd {}", worktree.display_path().bright_blue());
    println!("  2. Start development on isolated branch");
    println!("  3. Use {} when done", "autoflow worktree merge".bright_blue());

    Ok(())
}

async fn merge_worktree(manager: &WorktreeManager, branch: &str) -> anyhow::Result<()> {
    println!("{}", "🔀 Merging worktree...".bright_cyan().bold());
    println!("Branch: {}", branch.bright_blue());

    manager.merge_worktree(branch)
        .context("Failed to merge worktree")?;

    println!("\n{}", "✅ Worktree merged successfully!".green().bold());
    println!("\n{}", "Next steps:".bright_cyan());
    println!("  1. Delete worktree: {}", format!("autoflow worktree delete {}", branch).bright_blue());
    println!("  2. Or keep it for further development");

    Ok(())
}

async fn delete_worktree(manager: &WorktreeManager, branch: &str, _force: bool) -> anyhow::Result<()> {
    println!("{}", "🗑️  Deleting worktree...".bright_cyan().bold());

    // Extract worktree name from branch
    let worktree_name = if branch.starts_with("sprint-") {
        branch.to_string()
    } else {
        format!("sprint-{}", branch)
    };

    manager.delete_worktree(&worktree_name)
        .context("Failed to delete worktree")?;

    println!("\n{}", "✅ Worktree deleted successfully!".green().bold());

    Ok(())
}

async fn prune_worktrees(manager: &WorktreeManager) -> anyhow::Result<()> {
    println!("{}", "🧹 Pruning worktrees...".bright_cyan().bold());

    manager.prune_worktrees()
        .context("Failed to prune worktrees")?;

    println!("\n{}", "✅ Worktrees pruned successfully!".green().bold());

    Ok(())
}
