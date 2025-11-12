// Git integration for automatic project commits
use std::path::Path;
use std::process::Command;
use autoflow_data::{Result, AutoFlowError, Sprint, SprintStatus};

/// Commit project changes with a formatted message
///
/// This function stages all changes and creates a commit in the project repository.
/// It will silently skip if:
/// - No .git directory exists
/// - There are no changes to commit
/// - Git commands fail
pub fn commit_project_changes(project_path: &Path, sprint: &Sprint, message: &str) -> Result<()> {
    // Check if .git directory exists
    let git_dir = project_path.join(".git");
    if !git_dir.exists() {
        tracing::debug!("No .git directory found at {:?}, skipping commit", project_path);
        return Ok(());
    }

    // Format commit message with sprint context
    let commit_message = format!(
        "{}\n\nSprint {}: {}\nWorkflow: {:?}\nStatus: {:?}\n\n🤖 Automated commit by AutoFlow",
        message,
        sprint.id,
        sprint.goal,
        sprint.workflow_type,
        sprint.status
    );

    // Check if there are changes to commit
    let status_output = Command::new("git")
        .current_dir(project_path)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| AutoFlowError::ValidationError(format!("Failed to check git status: {}", e)))?;

    if status_output.stdout.is_empty() {
        tracing::debug!("No changes to commit in {:?}", project_path);
        return Ok(());
    }

    // Stage all changes (excluding .autoflow/)
    let add_result = Command::new("git")
        .current_dir(project_path)
        .args(["add", "-A", ":(exclude).autoflow/"])
        .output()
        .map_err(|e| AutoFlowError::ValidationError(format!("Failed to stage changes: {}", e)))?;

    if !add_result.status.success() {
        let error = String::from_utf8_lossy(&add_result.stderr);
        tracing::warn!("Failed to stage changes: {}", error);
        return Ok(()); // Don't fail the sprint, just skip committing
    }

    // Check if there are staged changes to commit
    let diff_result = Command::new("git")
        .current_dir(project_path)
        .args(["diff", "--cached", "--quiet"])
        .status()
        .map_err(|e| AutoFlowError::ValidationError(format!("Failed to check staged changes: {}", e)))?;

    // git diff --cached --quiet exits with 0 if no changes, 1 if there are changes
    if diff_result.success() {
        tracing::debug!("No staged changes to commit after excluding .autoflow/ in {:?}", project_path);
        return Ok(());
    }

    // Create commit
    let commit_result = Command::new("git")
        .current_dir(project_path)
        .args(["commit", "-m", &commit_message])
        .output()
        .map_err(|e| AutoFlowError::ValidationError(format!("Failed to create commit: {}", e)))?;

    if commit_result.status.success() {
        let output = String::from_utf8_lossy(&commit_result.stdout);
        tracing::info!("✓ Created git commit: {} - {}", message, output.trim());
    } else {
        let error = String::from_utf8_lossy(&commit_result.stderr);
        tracing::warn!("Failed to create commit: {} - Error: {}", message, error.trim());
    }

    Ok(())
}

/// Determine if a commit should be created after this sprint status/phase
pub fn should_commit_after_phase(status: SprintStatus) -> bool {
    match status {
        // Commit after code writing phases
        SprintStatus::WriteCode => true,
        SprintStatus::WriteUnitTests => true,
        SprintStatus::WriteE2eTests => true,

        // Commit after successful reviews
        SprintStatus::CodeReview => true,

        // Commit after successful test runs
        SprintStatus::RunUnitTests => true,
        SprintStatus::RunE2eTests => true,

        // Commit when sprint is done
        SprintStatus::Done => true,
        SprintStatus::Complete => true,

        // Commit after fix phases to track changes made by each fixer
        SprintStatus::ReviewFix => true,
        SprintStatus::UnitFix => true,
        SprintStatus::E2eFix => true,

        // Don't commit on pending/blocked
        SprintStatus::Pending => false,
        SprintStatus::Blocked => false,
    }
}

/// Get a human-readable commit message for a phase
pub fn get_commit_message_for_phase(status: SprintStatus) -> &'static str {
    match status {
        SprintStatus::WriteCode => "Implement code for sprint",
        SprintStatus::WriteUnitTests => "Add unit tests",
        SprintStatus::WriteE2eTests => "Add E2E tests",
        SprintStatus::CodeReview => "Code review passed",
        SprintStatus::RunUnitTests => "Unit tests passed",
        SprintStatus::RunE2eTests => "E2E tests passed",
        SprintStatus::ReviewFix => "Fix code review issues",
        SprintStatus::UnitFix => "Fix unit test failures",
        SprintStatus::E2eFix => "Fix E2E test failures",
        SprintStatus::Done => "Sprint completed",
        SprintStatus::Complete => "Sprint completed",
        _ => "Update code",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_commit_after_write_phases() {
        assert!(should_commit_after_phase(SprintStatus::WriteCode));
        assert!(should_commit_after_phase(SprintStatus::WriteUnitTests));
        assert!(should_commit_after_phase(SprintStatus::WriteE2eTests));
    }

    #[test]
    fn test_should_not_commit_after_fix_phases() {
        assert!(!should_commit_after_phase(SprintStatus::ReviewFix));
        assert!(!should_commit_after_phase(SprintStatus::UnitFix));
        assert!(!should_commit_after_phase(SprintStatus::E2eFix));
    }

    #[test]
    fn test_should_commit_when_done() {
        assert!(should_commit_after_phase(SprintStatus::Done));
    }
}
