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

    // For now, since we don't have claude-code, create a mock sprint
    // In production, this would call: execute_agent("make-sprints", &context, 10).await?;

    println!("\n{}", "⚠️  Note:".yellow());
    println!("  Agent execution requires claude-code to be installed");
    println!("  For now, creating a template sprint...");

    // Create a template sprint
    let next_id = sprints_data.sprints.len() as u32 + 1;
    let new_sprint = create_template_sprint(next_id, &description);

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
