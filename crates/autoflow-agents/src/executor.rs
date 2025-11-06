use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Agent execution result
#[derive(Debug)]
pub struct AgentResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Execute a Claude Code agent
pub async fn execute_agent(
    agent_name: &str,
    context: &str,
    max_turns: u32,
) -> Result<AgentResult> {
    tracing::info!("Executing agent: {}", agent_name);

    // TODO: Spawn Claude Code subprocess
    // For now, return mock success
    // In real implementation:
    // 1. Spawn: claude-code --agent {agent_name} --max-turns {max_turns}
    // 2. Pass context via stdin
    // 3. Parse JSON output stream
    // 4. Monitor completion

    let mut child = Command::new("claude-code")
        .arg("--agent")
        .arg(agent_name)
        .arg("--max-turns")
        .arg(max_turns.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn claude-code")?;

    // Write context to stdin
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin
            .write_all(context.as_bytes())
            .await
            .context("Failed to write context to agent")?;
        stdin.shutdown().await?;
    }

    // Read stdout
    let stdout = child
        .stdout
        .take()
        .context("Failed to capture stdout")?;
    let mut stdout_reader = BufReader::new(stdout).lines();

    let mut output = String::new();
    while let Some(line) = stdout_reader.next_line().await? {
        tracing::debug!("Agent output: {}", line);
        output.push_str(&line);
        output.push('\n');
    }

    // Wait for completion
    let status = child.wait().await.context("Failed to wait for agent")?;

    Ok(AgentResult {
        success: status.success(),
        output,
        error: if status.success() {
            None
        } else {
            Some(format!("Agent exited with status: {}", status))
        },
    })
}

/// Map sprint status to agent name
pub fn get_agent_for_status(status: &autoflow_data::SprintStatus) -> &'static str {
    use autoflow_data::SprintStatus;

    match status {
        SprintStatus::Pending => "make-sprints",
        SprintStatus::WriteUnitTests => "test-writer",
        SprintStatus::WriteCode => "code-implementer",
        SprintStatus::CodeReview => "reviewer",
        SprintStatus::ReviewFix => "review-fixer",
        SprintStatus::RunUnitTests => "unit-test-runner",
        SprintStatus::UnitFix => "unit-fixer",
        SprintStatus::WriteE2eTests => "e2e-writer",
        SprintStatus::RunE2eTests => "e2e-test-runner",
        SprintStatus::E2eFix => "e2e-fixer",
        SprintStatus::Complete => "health-check",
        SprintStatus::Done => "done",
        SprintStatus::Blocked => "debug-blocker",
    }
}

/// Get context for agent execution
pub fn build_agent_context(sprint: &autoflow_data::Sprint) -> String {
    format!(
        r#"Sprint #{}: {}

Status: {:?}
Total Effort: {}
Max Effort: {}

Deliverables:
{}

Tasks:
{}

Execute the appropriate actions for this sprint phase.
"#,
        sprint.id,
        sprint.goal,
        sprint.status,
        sprint.total_effort,
        sprint.max_effort,
        sprint
            .deliverables
            .iter()
            .map(|d| format!("- {}", d))
            .collect::<Vec<_>>()
            .join("\n"),
        sprint
            .tasks
            .iter()
            .map(|t| format!("- {} ({})", t.title, t.effort))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
