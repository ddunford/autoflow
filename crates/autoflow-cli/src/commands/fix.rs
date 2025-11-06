use anyhow::{bail, Context};
use autoflow_git::WorktreeManager;
use autoflow_utils::{sanitize_branch_name, Paths};
use colored::*;
use std::path::Path;

pub async fn run(description: String, auto_fix: bool, _playwright_headed: bool) -> anyhow::Result<()> {
    println!("{}", "🐛 Investigating bug...".bright_cyan().bold());
    println!("Bug: {}", description.bright_blue());

    // Check if we're in a git repository
    if !Path::new(".git").exists() {
        bail!("{}", "Not a git repository. Initialize git first.".red());
    }

    // Check if project is initialized
    if !Path::new(Paths::AUTOFLOW_DIR).exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    // Create bugfix worktree
    println!("\n{}", "Creating bugfix worktree...".bright_cyan());
    let branch_name = format!("bugfix-{}", sanitize_branch_name(&description, 5));

    let manager = WorktreeManager::new(".")
        .context("Failed to open git repository")?;

    // Use a high sprint ID for bugfix worktrees to avoid conflicts
    let bugfix_sprint_id = 900;

    match manager.create_worktree(bugfix_sprint_id, &branch_name) {
        Ok(worktree) => {
            println!("  {} Worktree created: {}", "✓".green(), worktree.path.display().to_string().bright_blue());
            println!("  Branch: {}", worktree.branch.bright_green());
            println!("  Port: {}", worktree.port.to_string().bright_yellow());
        }
        Err(e) => {
            // Worktree might already exist
            println!("  {} {}", "⚠".yellow(), format!("Worktree creation skipped: {}", e).yellow());
        }
    }

    // Build context for bug-investigator agent
    println!("\n{}", "Running bug investigation...".bright_cyan());

    let context = format!(
        r#"Investigate and fix the following bug:

Bug Description: {}

Your tasks:
1. Identify the root cause of the bug
2. Suggest a fix
3. Implement the fix if possible
4. Run relevant tests to verify the fix

Provide a detailed analysis of:
- Root cause
- Affected files
- Proposed solution
- Test results
"#,
        description
    );

    // Execute bug-investigator agent
    println!("  Spawning bug-investigator agent...");

    use autoflow_agents::execute_agent;

    match execute_agent("debug-blocker", &context, 15).await {
        Ok(result) => {
            if result.success {
                println!("  {} Investigation complete", "✓".green());

                // Display agent output
                println!("\n{}", "Investigation Results:".bright_green().bold());
                println!("{}", result.output);

                // Save analysis
                std::fs::create_dir_all(Paths::BUGS_DIR)?;

                let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
                let analysis_file = format!("{}/bug-{}.md", Paths::BUGS_DIR, timestamp);

                let analysis_content = format!(
                    r#"# Bug Analysis

**Date**: {}
**Description**: {}

## Investigation Results

{}

## Branch

Bugfix branch: `{}`
Worktree location: Check with `autoflow worktree list`
"#,
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    description,
                    result.output,
                    branch_name
                );

                std::fs::write(&analysis_file, analysis_content)?;
                println!("\n{} Analysis saved to {}", "✓".green(), analysis_file.bright_blue());

                if auto_fix {
                    println!("\n{}", "Next steps:".bright_cyan());
                    println!("  1. Review the fix in the bugfix worktree");
                    println!("  2. Run tests: {} in the worktree", "npm test".bright_blue());
                    println!("  3. Merge if tests pass: {}", format!("autoflow worktree merge {}", branch_name).bright_blue());
                    println!("  4. Delete worktree: {}", format!("autoflow worktree delete {}", branch_name).bright_blue());
                } else {
                    println!("\n{}", "Next steps:".bright_cyan());
                    println!("  1. Review the analysis in {}", analysis_file.bright_blue());
                    println!("  2. Switch to bugfix branch to implement fix");
                    println!("  3. Run {} with --auto-fix flag to apply automated fix", "autoflow fix".bright_blue());
                }
            } else {
                println!("  {} Investigation failed: {:?}", "✗".red(), result.error);
                bail!("Bug investigation failed");
            }
        }
        Err(e) => {
            println!("  {} Agent spawn failed: {}", "✗".red(), e);
            println!("\n{}", "Creating bug report manually...".bright_yellow());

            // Create manual bug report
            std::fs::create_dir_all(Paths::BUGS_DIR)?;

            let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
            let analysis_file = format!("{}/bug-{}.md", Paths::BUGS_DIR, timestamp);

            let manual_report = format!(
                r#"# Bug Report

**Date**: {}
**Description**: {}

## Status

Agent investigation failed. Manual investigation required.

## Branch

Bugfix branch: `{}`

## Next Steps

1. Manually investigate the bug
2. Implement fix in the bugfix branch
3. Run tests
4. Merge when ready: `autoflow worktree merge {}`
"#,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                description,
                branch_name,
                branch_name
            );

            std::fs::write(&analysis_file, manual_report)?;
            println!("  {} Bug report created: {}", "✓".green(), analysis_file.bright_blue());
        }
    }

    Ok(())
}
