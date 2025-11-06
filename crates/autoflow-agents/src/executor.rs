use anyhow::{bail, Context, Result};
use serde_json::json;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

/// Agent execution result
#[derive(Debug)]
pub struct AgentResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub log_path: Option<PathBuf>,
    pub json_log_path: Option<PathBuf>,
}

/// Agent definition loaded from .agent.md file
#[derive(Debug)]
struct AgentDef {
    model: String,
    tools: Vec<String>,
    system_prompt: String,
}

/// Load agent definition from ~/.claude/agents/{name}.agent.md
async fn load_agent_def(agent_name: &str) -> Result<AgentDef> {
    let home = std::env::var("HOME").context("HOME env var not set")?;
    let agent_path = PathBuf::from(home)
        .join(".claude")
        .join("agents")
        .join(format!("{}.agent.md", agent_name));

    if !agent_path.exists() {
        bail!("Agent file not found: {:?}", agent_path);
    }

    let content = tokio::fs::read_to_string(&agent_path)
        .await
        .context(format!("Failed to read agent file: {:?}", agent_path))?;

    // Parse frontmatter
    let mut lines = content.lines();

    // First line should be ---
    if lines.next() != Some("---") {
        bail!("Agent file missing frontmatter: {:?}", agent_path);
    }

    let mut model = "claude-sonnet-4-5-20250929".to_string();
    let mut tools = Vec::new();
    let mut in_frontmatter = true;
    let mut system_prompt = String::new();

    for line in lines {
        if in_frontmatter {
            if line == "---" {
                in_frontmatter = false;
                continue;
            }

            // Parse frontmatter fields
            if let Some(rest) = line.strip_prefix("model:") {
                model = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("tools:") {
                tools = rest
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        } else {
            // Everything after frontmatter is the system prompt
            system_prompt.push_str(line);
            system_prompt.push('\n');
        }
    }

    Ok(AgentDef {
        model,
        tools,
        system_prompt: system_prompt.trim().to_string(),
    })
}

/// Execute a Claude Code agent
pub async fn execute_agent(
    agent_name: &str,
    context: &str,
    _max_turns: u32,
    sprint_id: Option<u32>,
) -> Result<AgentResult> {
    tracing::info!("Executing agent: {}", agent_name);

    // Load agent definition
    let agent_def = load_agent_def(agent_name).await?;

    tracing::debug!("Agent model: {}", agent_def.model);
    tracing::debug!("Agent tools: {:?}", agent_def.tools);

    // Setup logging paths
    let (_log_dir, log_file, json_log_file) = if let Some(id) = sprint_id {
        let sprint_dir = PathBuf::from(".autoflow")
            .join("sprints")
            .join(format!("sprint-{:03}", id));
        let log_dir = sprint_dir.join("logs");

        create_dir_all(&log_dir).await.context("Failed to create log directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let log_file = log_dir.join(format!("{}_{}.log", agent_name, timestamp));
        let json_log_file = log_dir.join(format!("{}_{}.json", agent_name, timestamp));

        (Some(log_dir), Some(log_file), Some(json_log_file))
    } else {
        (None, None, None)
    };

    // Open log files
    let mut text_log = if let Some(ref path) = log_file {
        Some(File::create(path).await.context("Failed to create text log file")?)
    } else {
        None
    };

    let mut json_log = if let Some(ref path) = json_log_file {
        Some(File::create(path).await.context("Failed to create JSON log file")?)
    } else {
        None
    };

    // Write log header
    if let Some(ref mut log) = text_log {
        let header = format!(
            "================================\n\
             Agent: {}\n\
             Sprint: {:?}\n\
             Started: {}\n\
             ================================\n",
            agent_name,
            sprint_id,
            chrono::Utc::now().to_rfc3339()
        );
        log.write_all(header.as_bytes()).await?;
    }

    // Build the full prompt combining system prompt and context
    let full_prompt = format!("{}\n\n# Context\n\n{}", agent_def.system_prompt, context);

    // Check if debug mode is enabled
    let debug_mode = std::env::var("AUTOFLOW_DEBUG").unwrap_or_default() == "1"
        || std::env::var("RUST_LOG").unwrap_or_default().contains("debug");

    // Execute using claude CLI in print mode
    let mut child = Command::new("claude")
        .arg("--print")
        .arg("--output-format")
        .arg("text")
        .arg("--model")
        .arg(&agent_def.model)
        .arg("--dangerously-skip-permissions") // For automated execution
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn claude CLI")?;

    // Write full prompt to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(full_prompt.as_bytes())
            .await
            .context("Failed to write prompt to claude CLI")?;
        stdin.shutdown().await?;
    }

    // Read stdout with real-time logging
    let stdout = child
        .stdout
        .take()
        .context("Failed to capture stdout")?;
    let mut stdout_reader = BufReader::new(stdout).lines();

    let mut output = String::new();
    let start_time = chrono::Utc::now();

    while let Some(line) = stdout_reader.next_line().await? {
        tracing::debug!("Agent output: {}", line);
        output.push_str(&line);
        output.push('\n');

        // Write to text log
        if let Some(ref mut log) = text_log {
            log.write_all(line.as_bytes()).await?;
            log.write_all(b"\n").await?;
        }

        // Write to JSON log
        if let Some(ref mut json_log) = json_log {
            let json_entry = json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "agent": agent_name,
                "type": "output",
                "content": line,
            });
            json_log.write_all(serde_json::to_string(&json_entry)?.as_bytes()).await?;
            json_log.write_all(b"\n").await?;
        }

        // Filter output for console display (unless debug mode)
        if debug_mode {
            // Show everything in debug mode
            println!("{}", line);
        } else {
            // Show only important lines (tool usage, file operations, etc.)
            if line.starts_with("Using")
                || line.starts_with("Tool")
                || line.starts_with("Reading")
                || line.starts_with("Writing")
                || line.starts_with("Editing")
                || line.starts_with("✓")
                || line.starts_with("✗")
                || line.starts_with("Error")
                || line.starts_with("Warning")
                || line.contains("SUCCESS")
                || line.contains("FAILED")
            {
                println!("  {}", line);
            }
        }
    }

    // Read stderr for errors
    let stderr = child
        .stderr
        .take()
        .context("Failed to capture stderr")?;
    let mut stderr_reader = BufReader::new(stderr).lines();

    let mut error_output = String::new();
    while let Some(line) = stderr_reader.next_line().await? {
        tracing::warn!("Agent stderr: {}", line);
        error_output.push_str(&line);
        error_output.push('\n');

        // Write errors to both logs
        if let Some(ref mut log) = text_log {
            log.write_all(format!("STDERR: {}\n", line).as_bytes()).await?;
        }

        if let Some(ref mut json_log) = json_log {
            let json_entry = json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "agent": agent_name,
                "type": "error",
                "content": line,
            });
            json_log.write_all(serde_json::to_string(&json_entry)?.as_bytes()).await?;
            json_log.write_all(b"\n").await?;
        }

        // Always show errors on console
        eprintln!("  ERROR: {}", line);
    }

    // Wait for completion
    let status = child.wait().await.context("Failed to wait for agent")?;
    let end_time = chrono::Utc::now();
    let duration = end_time.signed_duration_since(start_time);

    // Write completion info to logs
    if let Some(ref mut log) = text_log {
        let footer = format!(
            "\n================================\n\
             Completed: {}\n\
             Duration: {}s\n\
             Exit Code: {}\n\
             ================================\n",
            end_time.to_rfc3339(),
            duration.num_seconds(),
            status.code().unwrap_or(-1)
        );
        log.write_all(footer.as_bytes()).await?;
    }

    if let Some(ref mut json_log) = json_log {
        let summary = json!({
            "timestamp": end_time.to_rfc3339(),
            "agent": agent_name,
            "type": "summary",
            "duration_seconds": duration.num_seconds(),
            "exit_code": status.code().unwrap_or(-1),
            "success": status.success(),
        });
        json_log.write_all(serde_json::to_string(&summary)?.as_bytes()).await?;
        json_log.write_all(b"\n").await?;
    }

    Ok(AgentResult {
        success: status.success(),
        output,
        error: if status.success() {
            None
        } else {
            Some(format!(
                "Agent exited with status: {}\nStderr: {}",
                status, error_output
            ))
        },
        log_path: log_file,
        json_log_path: json_log_file,
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
