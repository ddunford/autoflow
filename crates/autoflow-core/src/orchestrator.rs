use autoflow_data::{AutoFlowError, Result, Sprint, SprintStatus};
use crate::workflow::get_workflow_definition;
use crate::git::{commit_project_changes, should_commit_after_phase, get_commit_message_for_phase};
use chrono::Utc;
use std::path::PathBuf;

pub struct Orchestrator {
    max_iterations: u32,
    save_callback: Option<Box<dyn Fn(&Sprint) -> Result<()> + Send + Sync>>,
    project_path: Option<PathBuf>,
    enable_auto_commit: bool,
}

impl Orchestrator {
    pub fn new(max_iterations: u32) -> Self {
        Self {
            max_iterations,
            save_callback: None,
            project_path: None,
            enable_auto_commit: false,
        }
    }

    /// Set a callback to save sprint progress after each iteration
    pub fn with_save_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Sprint) -> Result<()> + Send + Sync + 'static,
    {
        self.save_callback = Some(Box::new(callback));
        self
    }

    /// Set the project path for automatic git commits
    pub fn with_project_path(mut self, path: PathBuf) -> Self {
        self.project_path = Some(path);
        self
    }

    /// Enable automatic git commits after successful phases
    pub fn with_auto_commit(mut self, enabled: bool) -> Self {
        self.enable_auto_commit = enabled;
        self
    }

    /// Run a sprint through its TDD pipeline phases
    pub async fn run_sprint(&self, sprint: &mut Sprint) -> Result<()> {
        let mut iteration = 0;
        let mut retry_count: std::collections::HashMap<SprintStatus, u32> = Default::default();

        // Set started timestamp if not already set
        if sprint.started.is_none() {
            sprint.started = Some(Utc::now());
        }

        while !sprint.is_done() && iteration < self.max_iterations {
            iteration += 1;

            tracing::info!(
                "Sprint {} - Iteration {} - Status: {:?}",
                sprint.id,
                iteration,
                sprint.status
            );

            // Check if we're in a BLOCKED state
            if sprint.status == SprintStatus::Blocked {
                tracing::error!("Sprint {} is BLOCKED", sprint.id);

                // Invoke blocker-resolver agent to analyze and diagnose
                tracing::info!("Invoking blocker-resolver agent to diagnose Sprint {}", sprint.id);

                match self.run_blocker_resolver(sprint).await {
                    Ok(analysis) => {
                        tracing::info!("Blocker analysis complete: {}", analysis);

                        // Mark that this sprint now uses blocker-resolver
                        // This means future failures will return to BLOCKED instead of unit-fixer
                        sprint.uses_blocker_resolver = true;

                        // Create git commit for blocker-resolver fixes
                        if self.enable_auto_commit {
                            if let Some(ref project_path) = self.project_path {
                                tracing::debug!("Committing blocker-resolver fixes for sprint {}", sprint.id);
                                let commit_msg = format!("Sprint {}: Fix blocked issues (blocker-resolver)", sprint.id);
                                if let Err(e) = commit_project_changes(project_path, sprint, &commit_msg) {
                                    tracing::warn!("Failed to create git commit for blocker-resolver: {}", e);
                                }
                            }
                        }

                        // Blocker-resolver may have fixed the issue, retry from test phase
                        tracing::info!("Blocker-resolver completed, resetting sprint {} to RUN_UNIT_TESTS to verify fix", sprint.id);
                        sprint.status = SprintStatus::RunUnitTests;
                        sprint.blocked_count = Some(0); // Reset blocked count
                        sprint.last_updated = Utc::now();

                        // Save progress and continue loop to retry
                        if let Some(ref save_fn) = self.save_callback {
                            save_fn(sprint)?;
                        }
                        continue; // Continue to next iteration
                    }
                    Err(e) => {
                        tracing::warn!("Blocker-resolver failed: {}", e);
                        // Blocker-resolver couldn't help, sprint stays blocked
                        return Err(AutoFlowError::SprintBlocked(
                            sprint.id,
                            format!("Blocker-resolver failed: {}", e),
                        ));
                    }
                }
            }

            // Execute the phase based on current status
            let phase_result = self.execute_phase(sprint).await;

            match phase_result {
                Ok(should_advance) => {
                    if should_advance {
                        // Phase succeeded, advance to next status using workflow
                        let current_status = sprint.status;

                        // Reset retry count for this status
                        retry_count.insert(current_status, 0);

                        let workflow = get_workflow_definition(sprint.workflow_type);

                        // If this is a fix phase, loop back to the validation phase
                        // Otherwise, advance to the next phase (skipping fix phases)
                        let next_status = if workflow.is_fix_phase(current_status) {
                            workflow.get_validation_phase_for_fix(current_status)
                                .map(|p| {
                                    tracing::info!(
                                        "Sprint {} completed fix phase {:?}, looping back to validation {:?}",
                                        sprint.id,
                                        current_status,
                                        p.status
                                    );
                                    p.status
                                })
                        } else {
                            workflow.next_phase_skip_fix(current_status)
                                .map(|p| {
                                    tracing::info!(
                                        "Sprint {} advanced from {:?} to {:?} (workflow: {:?})",
                                        sprint.id,
                                        current_status,
                                        p.status,
                                        sprint.workflow_type
                                    );
                                    p.status
                                })
                        };

                        if let Some(status) = next_status {
                            let previous_status = sprint.status;
                            sprint.status = status;
                            sprint.last_updated = Utc::now();

                            // Create git commit after successful phase completion
                            if self.enable_auto_commit && should_commit_after_phase(previous_status) {
                                if let Some(ref project_path) = self.project_path {
                                    tracing::debug!("Attempting to commit after phase: {:?}", previous_status);
                                    let commit_msg = get_commit_message_for_phase(previous_status);
                                    if let Err(e) = commit_project_changes(project_path, sprint, commit_msg) {
                                        tracing::warn!("Failed to create git commit for phase {:?}: {}", previous_status, e);
                                    }
                                }
                            }
                        } else {
                            tracing::warn!(
                                "Sprint {} at {:?} has no next phase in workflow",
                                sprint.id,
                                current_status
                            );
                        }
                    } else {
                        // Phase needs retry (e.g., tests failed)
                        let current_status = sprint.status;
                        let count = retry_count.entry(current_status).or_insert(0);
                        *count += 1;

                        // Get workflow to check max retries
                        let workflow = get_workflow_definition(sprint.workflow_type);
                        let max_retries = workflow
                            .get_phase(current_status)
                            .map(|p| p.max_retries)
                            .unwrap_or(1);

                        tracing::warn!(
                            "Sprint {} status {:?} needs retry ({}/{})",
                            sprint.id,
                            current_status,
                            count,
                            max_retries
                        );

                        // Check if we've exceeded max retries OR if this sprint uses blocker-resolver
                        if *count >= max_retries {
                            tracing::error!(
                                "Sprint {} exceeded max retries for {:?}, marking as BLOCKED",
                                sprint.id,
                                current_status
                            );
                            sprint.status = SprintStatus::Blocked;
                            sprint.blocked_count = Some(*count);
                            sprint.last_updated = Utc::now();
                        } else if sprint.uses_blocker_resolver {
                            // Sprint has been through blocker-resolver before
                            // Send it back to BLOCKED instead of using unit-fixer/e2e-fixer
                            // This prevents tactical fixes from breaking strategic changes
                            tracing::warn!(
                                "Sprint {} uses blocker-resolver - returning to BLOCKED instead of fix phase (retry {}/{})",
                                sprint.id,
                                count,
                                max_retries
                            );
                            sprint.status = SprintStatus::Blocked;
                            sprint.blocked_count = Some(*count);
                            sprint.last_updated = Utc::now();
                        } else {
                            // Move to fix status if available (from workflow)
                            if let Some(fix_phase) = workflow.get_fix_phase(current_status) {
                                sprint.status = fix_phase.status;
                                sprint.last_updated = Utc::now();
                                tracing::info!(
                                    "Sprint {} moving to fix phase {:?}",
                                    sprint.id,
                                    fix_phase.status
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Sprint {} phase failed: {}", sprint.id, e);

                    // Track retry
                    let current_status = sprint.status;
                    let count = retry_count.entry(current_status).or_insert(0);
                    *count += 1;

                    // Get workflow to check max retries
                    let workflow = get_workflow_definition(sprint.workflow_type);
                    let max_retries = workflow
                        .get_phase(current_status)
                        .map(|p| p.max_retries)
                        .unwrap_or(1);

                    if *count >= max_retries {
                        sprint.status = SprintStatus::Blocked;
                        sprint.blocked_count = Some(*count);
                        sprint.last_updated = Utc::now();
                    }
                }
            }

            // Save progress after each iteration
            if let Some(ref save_fn) = self.save_callback {
                save_fn(sprint)?;
            }
        }

        // Check if we hit max iterations
        if iteration >= self.max_iterations && !sprint.is_done() {
            return Err(AutoFlowError::MaxIterationsExceeded(self.max_iterations));
        }

        // Set completion timestamp if done
        if sprint.is_done() && sprint.completed_at.is_none() {
            sprint.completed_at = Some(Utc::now());
        }

        Ok(())
    }

    /// Execute a phase based on sprint status
    /// Returns Ok(true) if should advance, Ok(false) if should retry, Err if failed
    async fn execute_phase(&self, sprint: &Sprint) -> Result<bool> {
        use autoflow_agents::{build_agent_context, build_fixer_context, build_test_runner_context, execute_agent};

        // Get workflow definition for this sprint
        let workflow = get_workflow_definition(sprint.workflow_type);

        // Get the current phase from workflow
        let phase = workflow.get_phase(sprint.status).ok_or_else(|| {
            AutoFlowError::ValidationError(format!(
                "No phase definition for status {:?} in workflow {:?}",
                sprint.status, sprint.workflow_type
            ))
        })?;

        // Auto-advance Pending sprints without executing any agent
        // Pending means the sprint is planned but not started yet
        if sprint.status == SprintStatus::Pending {
            tracing::info!(
                "Sprint {} is Pending - auto-advancing to next phase (workflow: {:?})",
                sprint.id,
                sprint.workflow_type
            );
            return Ok(true);
        }

        // Skip execution if agent is "none"
        if phase.agent == "none" {
            tracing::info!(
                "Sprint {} status {:?} has no agent - auto-advancing",
                sprint.id,
                sprint.status
            );
            return Ok(true);
        }

        let agent_name = phase.agent;

        // Use lightweight context for different agent types to reduce token usage
        let context = match sprint.status {
            // Test runner agents - only need test specifications
            SprintStatus::RunUnitTests
            | SprintStatus::RunE2eTests
            | SprintStatus::WriteE2eTests => {
                tracing::debug!("Using lightweight test runner context for {:?}", sprint.status);
                build_test_runner_context(sprint)
            }
            // Fixer agents - only need failure reports
            SprintStatus::ReviewFix
            | SprintStatus::UnitFix
            | SprintStatus::E2eFix => {
                tracing::debug!("Using lightweight fixer context for {:?}", sprint.status);
                build_fixer_context(sprint)
            }
            // All other agents - need full context
            _ => {
                build_agent_context(sprint)
            }
        };

        let max_turns = phase.max_turns;

        tracing::info!(
            "Executing agent '{}' for sprint {} status {:?} (workflow: {:?})",
            agent_name,
            sprint.id,
            sprint.status,
            sprint.workflow_type
        );

        // Archive any existing failure reports before running agents that write to .failures
        if let Some(ref project_path) = self.project_path {
            archive_failure_reports_before_agent(project_path, sprint.id, agent_name);
        }

        // Execute agent
        let result = execute_agent(agent_name, &context, max_turns, Some(sprint.id))
            .await
            .map_err(|e| AutoFlowError::AgentExecutionFailed(agent_name.to_string(), e.to_string()))?;

        if result.success {
            tracing::info!("Agent '{}' completed successfully", agent_name);

            // Determine if we should advance or retry based on status
            let should_advance = match sprint.status {
                // Test phases - check if tests actually passed
                SprintStatus::RunUnitTests | SprintStatus::RunE2eTests => {
                    let passed = parse_test_results(&result.output);
                    if passed {
                        tracing::info!("Tests passed - advancing to next phase");
                    } else {
                        tracing::warn!("Tests failed - moving to fix phase");
                    }
                    passed
                }
                // Review phase - check if review actually passed
                SprintStatus::CodeReview => {
                    let passed = parse_review_results(&result.output);
                    if passed {
                        tracing::info!("Code review passed - advancing to next phase");
                    } else {
                        tracing::warn!("Code review failed - moving to fix phase");
                    }
                    passed
                }
                // All other phases advance on success
                _ => true,
            };

            Ok(should_advance)
        } else {
            tracing::warn!(
                "Agent '{}' failed: {:?}",
                agent_name,
                result.error
            );
            Ok(false) // Retry
        }
    }

    /// Run multiple sprints in parallel
    pub async fn run_parallel(&self, sprints: &mut [Sprint]) -> Result<Vec<Result<()>>> {
        use futures::future::join_all;

        let futures = sprints.iter_mut().map(|sprint| self.run_sprint(sprint));

        let results = join_all(futures).await;

        Ok(results)
    }

    /// Run blocker-resolver agent to diagnose blocked sprint
    async fn run_blocker_resolver(&self, sprint: &Sprint) -> Result<String> {
        use autoflow_agents::{build_fixer_context, execute_agent};

        // Use lightweight context - blocker-resolver only needs failure reports, not full task details
        let context = build_fixer_context(sprint);
        let max_turns = 10; // Give resolver plenty of turns to investigate

        tracing::info!("Executing blocker-resolver agent for sprint {}", sprint.id);

        let result = execute_agent("blocker-resolver", &context, max_turns, Some(sprint.id))
            .await
            .map_err(|e| AutoFlowError::AgentExecutionFailed("blocker-resolver".to_string(), e.to_string()))?;

        if result.success {
            tracing::info!("Blocker-resolver analysis complete");
            Ok(result.output)
        } else {
            Err(AutoFlowError::AgentExecutionFailed(
                "blocker-resolver".to_string(),
                result.error.unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }
}

/// Parse test results from agent output
/// Returns true if tests passed, false if they failed
///
/// Looks for standardized output marker: "TEST_RESULT: PASSED" or "TEST_RESULT: FAILED"
/// This format is enforced in all test-runner agent prompts for reliable parsing.
fn parse_test_results(output: &str) -> bool {
    // Look for the standardized marker first
    if output.contains("TEST_RESULT: PASSED") {
        return true;
    }

    if output.contains("TEST_RESULT: FAILED") {
        return false;
    }

    // Fallback: if no standardized marker found, log warning and assume passed
    // This allows backward compatibility but we should fix agents to use the marker
    tracing::warn!(
        "Test output missing standardized marker 'TEST_RESULT: PASSED/FAILED'. \
         Defaulting to PASSED. Please ensure test-runner agents output the required marker."
    );
    true
}

/// Parse code review results from agent output
/// Returns true if review passed, false if it failed
///
/// Looks for standardized output marker: "REVIEW_STATUS: PASSED" or "REVIEW_STATUS: FAILED"
/// This format is enforced in the reviewer agent prompt for reliable parsing.
fn parse_review_results(output: &str) -> bool {
    // Look for the standardized marker first
    if output.contains("REVIEW_STATUS: PASSED") {
        return true;
    }

    if output.contains("REVIEW_STATUS: FAILED") {
        return false;
    }

    // Fallback: if no standardized marker found, log warning and assume passed
    // This allows backward compatibility but we should fix the reviewer agent to use the marker
    tracing::warn!(
        "Review output missing standardized marker 'REVIEW_STATUS: PASSED/FAILED'. \
         Defaulting to PASSED. Please ensure reviewer agent outputs the required marker."
    );
    true
}

/// Archive existing failure reports before running agents that write to .failures
/// This preserves iteration history for debugging infinite loops
fn archive_failure_reports_before_agent(project_path: &PathBuf, sprint_id: u32, agent_name: &str) {
    use chrono::Local;
    use std::fs;

    // Map agent names to their failure file patterns
    let failure_file = match agent_name {
        "reviewer" => format!("sprint-{}-review.md", sprint_id),
        "unit-test-runner" => format!("sprint-{}-unit-tests.md", sprint_id),
        "e2e-test-runner" => format!("sprint-{}-integration-tests.md", sprint_id),
        "blocker-resolver" => format!("blocker-analysis-sprint-{}.md", sprint_id),
        _ => return, // Agent doesn't write to .failures
    };

    let failures_dir = project_path.join(".autoflow").join(".failures");
    let archive_dir = failures_dir.join("archive");
    let failure_path = failures_dir.join(&failure_file);

    // Only archive if the file exists
    if !failure_path.exists() {
        return;
    }

    // Ensure archive directory exists
    if let Err(e) = fs::create_dir_all(&archive_dir) {
        tracing::warn!("Failed to create archive directory: {}", e);
        return;
    }

    // Create timestamped archive filename
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let archive_filename = failure_file.replace(".md", &format!("-{}.md", timestamp));
    let archive_path = archive_dir.join(&archive_filename);

    // Copy to archive
    if let Err(e) = fs::copy(&failure_path, &archive_path) {
        tracing::warn!("Failed to archive {}: {}", failure_file, e);
    } else {
        tracing::info!("Archived {} to archive/{}", failure_file, archive_filename);
    }
}
