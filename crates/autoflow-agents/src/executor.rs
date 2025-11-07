use anyhow::{bail, Context, Result};
use autoflow_utils::get_debug_logger;
use std::path::PathBuf;
use std::process::Stdio;
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

    // Check for model override via environment variable
    let final_model = if let Ok(env_model) = std::env::var("AUTOFLOW_MODEL") {
        tracing::info!("Using model override from AUTOFLOW_MODEL env: {}", env_model);
        env_model
    } else {
        model
    };

    Ok(AgentDef {
        model: final_model,
        tools,
        system_prompt: system_prompt.trim().to_string(),
    })
}

/// Execute a Claude Code agent
pub async fn execute_agent(
    agent_name: &str,
    context: &str,
    _max_turns: u32,
    _sprint_id: Option<u32>,
) -> Result<AgentResult> {
    tracing::info!("Executing agent: {}", agent_name);

    // Initialize debug logger
    let debug_logger = get_debug_logger();
    if let Some(ref logger) = debug_logger {
        let _ = logger.log_agent_start(agent_name, context);
    }

    // Load agent definition
    let agent_def = load_agent_def(agent_name).await?;

    tracing::debug!("Agent model: {}", agent_def.model);
    tracing::debug!("Agent tools: {:?}", agent_def.tools);

    if let Some(ref logger) = debug_logger {
        let _ = logger.log_agent_step(
            agent_name,
            "Loaded agent definition",
            &format!("Model: {}\nTools: {:?}", agent_def.model, agent_def.tools)
        );
    }

    // Log the system prompt
    if let Some(ref logger) = debug_logger {
        let _ = logger.log_agent_step(
            agent_name,
            "Agent system prompt",
            &format!("---\n{}\n---", agent_def.system_prompt)
        );
    }

    // Note: All logging is now handled by the debug_logger in .autoflow/.debug/
    // Sprint-specific logs are no longer created to avoid duplication

    // Build the full prompt combining system prompt and context
    let full_prompt = format!("{}\n\n# Context\n\n{}", agent_def.system_prompt, context);

    // Log the FULL combined prompt that will be sent to Claude
    if let Some(ref logger) = debug_logger {
        let _ = logger.log_agent_step(
            agent_name,
            "Full prompt being sent to Claude",
            &format!("===\n{}\n===", full_prompt)
        );
    }

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

    while let Some(line) = stdout_reader.next_line().await? {
        tracing::debug!("Agent output: {}", line);
        output.push_str(&line);
        output.push('\n');

        // Write to debug log
        if let Some(ref logger) = debug_logger {
            let _ = logger.log_agent_output(agent_name, &format!("{}\n", line));
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

        // Always show errors on console
        eprintln!("  ERROR: {}", line);
    }

    // Wait for completion
    let status = child.wait().await.context("Failed to wait for agent")?;

    // Log completion to debug logger
    if let Some(ref logger) = debug_logger {
        if !status.success() {
            let error_msg = format!("Agent exited with status: {}\nStderr: {}", status, error_output);
            let _ = logger.log_agent_end(agent_name, status.success(), Some(&error_msg));
        } else {
            let _ = logger.log_agent_end(agent_name, status.success(), None::<&str>);
        }
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
        log_path: None,
        json_log_path: None,
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

/// Extract a section from markdown content by heading
fn extract_markdown_section(content: &str, section_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_content = String::new();
    let mut section_level = 0;

    for line in lines {
        // Check if this is a heading
        if line.starts_with('#') {
            let heading_level = line.chars().take_while(|c| *c == '#').count();
            let heading_text = line.trim_start_matches('#').trim();

            // Check if this is our target section
            if heading_text.eq_ignore_ascii_case(section_name) {
                in_section = true;
                section_level = heading_level;
                section_content.push_str(line);
                section_content.push('\n');
                continue;
            }

            // If we're in a section and hit a same-or-higher level heading, we're done
            if in_section && heading_level <= section_level {
                break;
            }
        }

        // Add line if we're in the target section
        if in_section {
            section_content.push_str(line);
            section_content.push('\n');
        }
    }

    if section_content.is_empty() {
        None
    } else {
        Some(section_content)
    }
}

/// Get context for agent execution with full task details and referenced documentation
pub fn build_agent_context(sprint: &autoflow_data::Sprint) -> String {
    // Build detailed task information
    let tasks_detail = sprint
        .tasks
        .iter()
        .map(|task| {
            let mut task_str = format!("\n## Task: {}\n", task.title);
            task_str.push_str(&format!("- ID: {}\n", task.id));
            task_str.push_str(&format!("- Effort: {}\n", task.effort));
            task_str.push_str(&format!("- Priority: {:?}\n", task.priority));
            task_str.push_str(&format!("- Type: {:?}\n", task.r#type));
            task_str.push_str(&format!("- Feature: {}\n", task.feature));

            if let Some(ref desc) = task.description {
                task_str.push_str(&format!("\n**Description:**\n{}\n", desc));
            }

            if !task.business_rules.is_empty() {
                task_str.push_str("\n**Business Rules:**\n");
                for rule in &task.business_rules {
                    task_str.push_str(&format!("- {}\n", rule));
                }
            }

            if !task.acceptance_criteria.is_empty() {
                task_str.push_str("\n**Acceptance Criteria:**\n");
                for criterion in &task.acceptance_criteria {
                    task_str.push_str(&format!("- {}\n", criterion));
                }
            }

            if let Some(ref test_spec) = task.test_specification {
                task_str.push_str(&format!("\n**Test Specification:**\n{}\n", test_spec));
            }

            if !task.docs.is_empty() {
                task_str.push_str("\n**Documentation References:**\n");
                for doc in &task.docs {
                    task_str.push_str(&format!("- {}\n", doc));
                }
            }

            task_str
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Load and include referenced documentation sections
    let mut doc_sections = String::new();
    let unique_docs: std::collections::HashSet<String> = sprint
        .tasks
        .iter()
        .flat_map(|t| t.docs.iter().cloned())
        .collect();

    if !unique_docs.is_empty() {
        doc_sections.push_str("\n\n# Referenced Documentation\n");

        for doc_ref in unique_docs {
            // Parse reference: "BUILD_SPEC.md#TechStack" -> ("BUILD_SPEC.md", Some("TechStack"))
            let parts: Vec<&str> = doc_ref.split('#').collect();
            let filename = parts[0];
            let section = parts.get(1);

            let doc_path = format!(".autoflow/docs/{}", filename);

            if let Ok(content) = std::fs::read_to_string(&doc_path) {
                if let Some(section_name) = section {
                    // Extract specific section
                    if let Some(section_content) = extract_markdown_section(&content, section_name) {
                        doc_sections.push_str(&format!("\n## {} (from {})\n\n{}\n", section_name, filename, section_content));
                    } else {
                        doc_sections.push_str(&format!("\n## {} (section not found in {})\n\n", section_name, filename));
                    }
                } else {
                    // Include entire file if no section specified
                    doc_sections.push_str(&format!("\n## {}\n\n{}\n", filename, content));
                }
            } else {
                doc_sections.push_str(&format!("\n## {} (file not found at {})\n\n", doc_ref, doc_path));
            }
        }
    }

    format!(
        r#"Sprint #{}: {}

**Status:** {:?}
**Workflow:** {:?}
**Total Effort:** {}
**Max Effort:** {}

# Deliverables

{}

# Tasks
{}
{}

---

Execute the appropriate actions for this sprint phase based on the status and workflow type.
Use the task details, business rules, acceptance criteria, and referenced documentation above.
"#,
        sprint.id,
        sprint.goal,
        sprint.status,
        sprint.workflow_type,
        sprint.total_effort,
        sprint.max_effort,
        sprint
            .deliverables
            .iter()
            .map(|d| format!("- {}", d))
            .collect::<Vec<_>>()
            .join("\n"),
        tasks_detail,
        doc_sections,
    )
}

/// Build lightweight context for test runner agents
/// Test runners only need the sprint goal and test specifications, not full task details
pub fn build_test_runner_context(sprint: &autoflow_data::Sprint) -> String {
    // Collect all test specifications from tasks
    let test_specs: Vec<String> = sprint
        .tasks
        .iter()
        .filter_map(|task| {
            task.test_specification.as_ref().map(|spec| {
                format!("**{}:**\n{}", task.title, spec)
            })
        })
        .collect();

    let test_specs_str = if test_specs.is_empty() {
        "No specific test specifications provided. Run all available tests for this sprint.".to_string()
    } else {
        test_specs.join("\n\n")
    };

    format!(
        r#"Sprint #{}: {}

# Test Runner Context

Run tests for the code implemented in this sprint.

## Test Specifications

{}

## Instructions

1. Identify and run the appropriate tests for this sprint
2. Report all test results clearly
3. **REQUIRED**: End your response with one of these markers:
   - `TEST_RESULT: PASSED` if all tests pass
   - `TEST_RESULT: FAILED` if any tests fail

The orchestrator uses this marker to determine workflow progression.
"#,
        sprint.id,
        sprint.goal,
        test_specs_str
    )
}
