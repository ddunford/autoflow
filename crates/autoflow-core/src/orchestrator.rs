use autoflow_data::{AutoFlowError, Result, Sprint, SprintStatus};
use chrono::Utc;

pub struct Orchestrator {
    max_iterations: u32,
    save_callback: Option<Box<dyn Fn(&Sprint) -> Result<()> + Send + Sync>>,
}

impl Orchestrator {
    pub fn new(max_iterations: u32) -> Self {
        Self {
            max_iterations,
            save_callback: None,
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
                return Err(AutoFlowError::SprintBlocked(
                    sprint.id,
                    format!("Sprint blocked after {} iterations", iteration),
                ));
            }

            // Execute the phase based on current status
            let phase_result = self.execute_phase(sprint).await;

            match phase_result {
                Ok(should_advance) => {
                    if should_advance {
                        // Phase succeeded, advance to next status
                        let current_status = sprint.status;

                        // Reset retry count for this status
                        retry_count.insert(current_status, 0);

                        sprint.advance()?;

                        tracing::info!(
                            "Sprint {} advanced from {:?} to {:?}",
                            sprint.id,
                            current_status,
                            sprint.status
                        );
                    } else {
                        // Phase needs retry (e.g., tests failed)
                        let current_status = sprint.status;
                        let count = retry_count.entry(current_status).or_insert(0);
                        *count += 1;

                        tracing::warn!(
                            "Sprint {} status {:?} needs retry ({}/{})",
                            sprint.id,
                            current_status,
                            count,
                            current_status.max_retries()
                        );

                        // Check if we've exceeded max retries
                        if *count >= current_status.max_retries() {
                            tracing::error!(
                                "Sprint {} exceeded max retries for {:?}, marking as BLOCKED",
                                sprint.id,
                                current_status
                            );
                            sprint.status = SprintStatus::Blocked;
                            sprint.blocked_count = Some(*count);
                            sprint.last_updated = Utc::now();
                        } else {
                            // Move to fix status if available
                            if let Some(fix_status) = self.get_fix_status(current_status) {
                                sprint.status = fix_status;
                                sprint.last_updated = Utc::now();
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

                    if *count >= current_status.max_retries() {
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
        use autoflow_agents::{build_agent_context, execute_agent, get_agent_for_status};

        let agent_name = get_agent_for_status(&sprint.status);
        let context = build_agent_context(sprint);
        let max_turns = self.get_max_turns_for_status(sprint.status);

        tracing::info!(
            "Executing agent '{}' for sprint {} status {:?}",
            agent_name,
            sprint.id,
            sprint.status
        );

        // Execute agent
        let result = execute_agent(agent_name, &context, max_turns)
            .await
            .map_err(|e| AutoFlowError::AgentExecutionFailed(agent_name.to_string(), e.to_string()))?;

        if result.success {
            tracing::info!("Agent '{}' completed successfully", agent_name);

            // Determine if we should advance or retry based on status
            let should_advance = match sprint.status {
                // Test phases - check if tests passed
                SprintStatus::RunUnitTests | SprintStatus::RunE2eTests => {
                    // TODO: Parse test results from agent output
                    // For now, assume success means tests passed
                    true
                }
                // Review phase - check if review passed
                SprintStatus::CodeReview => {
                    // TODO: Parse review results
                    // For now, assume success means review passed
                    true
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

    /// Get the fix status for a given status (if applicable)
    fn get_fix_status(&self, status: SprintStatus) -> Option<SprintStatus> {
        match status {
            SprintStatus::CodeReview => Some(SprintStatus::ReviewFix),
            SprintStatus::RunUnitTests => Some(SprintStatus::UnitFix),
            SprintStatus::RunE2eTests => Some(SprintStatus::E2eFix),
            _ => None,
        }
    }

    /// Get max turns for agent based on status
    fn get_max_turns_for_status(&self, status: SprintStatus) -> u32 {
        match status {
            SprintStatus::WriteCode => 10,
            SprintStatus::CodeReview => 5,
            SprintStatus::ReviewFix => 8,
            SprintStatus::UnitFix => 8,
            SprintStatus::E2eFix => 10,
            SprintStatus::WriteUnitTests | SprintStatus::WriteE2eTests => 6,
            _ => 5,
        }
    }

    /// Run multiple sprints in parallel
    pub async fn run_parallel(&self, sprints: &mut [Sprint]) -> Result<Vec<Result<()>>> {
        use futures::future::join_all;

        let futures = sprints.iter_mut().map(|sprint| self.run_sprint(sprint));

        let results = join_all(futures).await;

        Ok(results)
    }
}
