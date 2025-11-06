use anyhow::{bail, Context};
use autoflow_data::{Sprint, SprintsYaml};
use colored::*;
use std::fs;
use std::path::Path;

pub async fn run(description: String, requirements: Option<String>) -> anyhow::Result<()> {
    println!("{}", "➕ Adding new feature...".bright_cyan().bold());
    println!("\nFeature: {}", description.bright_blue());

    if let Some(ref req) = requirements {
        println!("Requirements: {}", req.bright_blue());
    }

    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";
    if !Path::new(sprints_path).exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow init".bright_blue()
        );
    }

    // Load existing sprints
    println!("\n{}", "Loading existing sprints...".bright_cyan());
    let mut sprints_data = SprintsYaml::load(sprints_path)
        .context("Failed to load SPRINTS.yml")?;

    println!("Current sprints: {}", sprints_data.sprints.len().to_string().bright_blue());

    // Check if INTEGRATION_GUIDE.md exists
    let integration_guide_path = ".autoflow/INTEGRATION_GUIDE.md";
    let has_integration_guide = Path::new(integration_guide_path).exists();

    if !has_integration_guide {
        println!("\n{}", "💡 Tip:".bright_yellow());
        println!("  Run {} first to analyze your codebase", "autoflow analyze".bright_blue());
        println!("  This helps generate better integration-aware sprints");
    }

    // Build context for make-sprints agent
    println!("\n{}", "Generating sprints...".bright_cyan());

    let mut context = format!(
        r#"Generate sprints for the following feature:

Feature: {}
"#,
        description
    );

    if let Some(req) = requirements {
        context.push_str(&format!("\nRequirements:\n{}\n", req));
    }

    // Add integration guide if available
    if has_integration_guide {
        if let Ok(guide_content) = fs::read_to_string(integration_guide_path) {
            context.push_str("\n# Integration Guide\n\n");
            context.push_str(&guide_content);
            println!("  Using integration guide for context");
        }
    }

    // Add existing project info
    context.push_str(&format!(
        r#"

# Existing Project
- Total Sprints: {}
- Next Sprint ID: {}

Generate new sprints that integrate with the existing codebase.
Focus on incremental, testable changes.
Return ONLY valid YAML (no markdown code blocks).
"#,
        sprints_data.project.total_sprints,
        sprints_data.sprints.len() + 1
    ));

    // Execute make-sprints agent
    println!("  Spawning make-sprints agent...");

    use autoflow_agents::execute_agent;

    let next_id = sprints_data.sprints.len() as u32 + 1;

    // Try to execute the agent, fall back to template if it fails
    let new_sprint = match execute_agent("make-sprints", &context, 10, None).await {
        Ok(result) => {
            if result.success {
                // Parse the agent output to extract sprint
                match parse_sprint_from_output(&result.output, next_id, &description) {
                    Ok(sprint) => {
                        println!("  {} Agent generated sprint", "✓".green());
                        sprint
                    }
                    Err(e) => {
                        println!("  {} Failed to parse agent output: {}", "⚠".yellow(), e);
                        println!("  Creating template sprint instead...");
                        create_template_sprint(next_id, &description)
                    }
                }
            } else {
                println!("  {} Agent execution failed", "⚠".yellow());
                println!("  Creating template sprint instead...");
                create_template_sprint(next_id, &description)
            }
        }
        Err(e) => {
            println!("  {} Agent spawn failed: {}", "⚠".yellow(), e);
            println!("  Creating template sprint instead...");
            create_template_sprint(next_id, &description)
        }
    };

    // Add to sprints
    sprints_data.sprints.push(new_sprint.clone());
    sprints_data.project.total_sprints = sprints_data.sprints.len() as u32;
    sprints_data.project.last_updated = chrono::Utc::now();

    // Save updated sprints
    println!("\n{}", "Saving updated sprints...".bright_cyan());
    sprints_data.save(sprints_path)
        .context("Failed to save SPRINTS.yml")?;

    println!("{} {}", "✅".green(), "Sprint added successfully!".bright_green());

    // Display summary
    println!("\n{}", "New Sprint".bright_green().bold());
    println!("  ID: {}", new_sprint.id.to_string().bright_blue());
    println!("  Goal: {}", new_sprint.goal.bright_blue());
    println!("  Status: {}", format!("{:?}", new_sprint.status).bright_yellow());
    println!("  Effort: {}", new_sprint.total_effort.bright_blue());
    println!("  Tasks: {}", new_sprint.tasks.len().to_string().bright_blue());

    println!("\n{}", "Next steps:".bright_cyan());
    println!("  1. Review sprint in {}", sprints_path.bright_blue());
    println!("  2. Run {} to start development", "autoflow start".bright_blue());

    Ok(())
}

/// Parse sprint from agent output
fn parse_sprint_from_output(output: &str, next_id: u32, _description: &str) -> anyhow::Result<Sprint> {
    // Extract YAML from markdown if present
    let yaml_content = if output.contains("```yaml") {
        output
            .split("```yaml")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(output)
            .trim()
    } else {
        output.trim()
    };

    // Write to temp file and parse with SprintsYaml
    let temp_path = format!("/tmp/sprint-{}.yml", next_id);
    fs::write(&temp_path, yaml_content)
        .context("Failed to write temp sprint file")?;

    // Try to load as full SprintsYaml document
    match SprintsYaml::load(&temp_path) {
        Ok(sprints_yaml) => {
            // Extract first sprint
            if let Some(mut sprint) = sprints_yaml.sprints.into_iter().next() {
                sprint.id = next_id;
                let _ = fs::remove_file(&temp_path); // Clean up
                Ok(sprint)
            } else {
                let _ = fs::remove_file(&temp_path);
                anyhow::bail!("No sprints found in agent output")
            }
        }
        Err(e) => {
            let _ = fs::remove_file(&temp_path);
            anyhow::bail!("Failed to parse sprint YAML: {}", e)
        }
    }
}

/// Create a template sprint (used when agent is not available)
fn create_template_sprint(id: u32, description: &str) -> Sprint {
    use autoflow_data::{SprintStatus, Task, Priority, TestingRequirements, TestRequirement};
    use chrono::Utc;

    Sprint {
        id,
        goal: description.to_string(),
        status: SprintStatus::Pending,
        duration: Some("Week 1".to_string()),
        total_effort: "8h".to_string(),
        max_effort: "15h".to_string(),
        started: None,
        last_updated: Utc::now(),
        completed_at: None,
        deliverables: vec![
            format!("{} implementation", description),
            "Unit tests".to_string(),
            "E2E tests".to_string(),
        ],
        tasks: vec![
            Task {
                id: format!("task-{:03}", id),
                title: format!("Implement {}", description),
                description: Some(format!("Implement {} feature", description)),
                r#type: Some("IMPLEMENTATION".to_string()),
                doc_reference: None,
                acceptance_criteria: vec!["Feature works as specified".to_string()],
                test_specification: Some("Unit tests pass".to_string()),
                effort: "6h".to_string(),
                priority: Priority::High,
                feature: description.to_string(),
                docs: vec![],
                business_rules: vec![
                    "Follow existing code patterns".to_string(),
                    "Maintain backward compatibility".to_string(),
                ],
                integration_notes: None,
                testing: TestingRequirements {
                    unit_tests: Some(TestRequirement {
                        required: true,
                        reason: "Core functionality validation".to_string(),
                    }),
                    integration_tests: None,
                    e2e_tests: None,
                },
                status: autoflow_data::TaskStatus::Pending,
                committed_at: None,
                reviewed_at: None,
                tested_at: None,
                done_at: None,
                git_commit: None,
            },
        ],
        dependencies: vec![],
        integration_points: None,
        blocked_count: None,
        must_complete_first: false,
    }
}
