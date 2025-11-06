use anyhow::{bail, Context};
use autoflow_core::Orchestrator;
use autoflow_data::{SprintsYaml, SprintStatus};
use autoflow_utils::{check_for_updates, should_check_for_updates, prompt_and_update, update_check_timestamp};
use colored::*;
use std::path::Path;

pub async fn run(parallel: bool, sprint: Option<u32>) -> anyhow::Result<()> {
    println!("{}", "🚀 Starting AutoFlow...".bright_cyan().bold());

    // Check for updates (if enabled and interval has passed)
    if should_check_for_updates().unwrap_or(false) {
        match check_for_updates() {
            Ok(info) if info.has_updates() => {
                // Prompt user and update if they accept
                prompt_and_update(&info)?;
            }
            Ok(_) => {
                // No updates, just update timestamp
                update_check_timestamp()?;
            }
            Err(_) => {
                // Silently ignore update check failures
            }
        }
    }

    // Check if project is initialized
    let sprints_path = ".autoflow/SPRINTS.yml";

    // If SPRINTS.yml doesn't exist but IDEA.md does, initialize with empty sprints
    if !Path::new(sprints_path).exists() {
        if Path::new("IDEA.md").exists() {
            println!("\n{}", "Initializing project from IDEA.md...".bright_cyan());
            std::fs::create_dir_all(".autoflow/docs")?;

            let current_dir = std::env::current_dir()?;
            let project_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Project")
                .to_string();

            let empty_sprints = format!(
                "project:\n  name: \"{}\"\n  total_sprints: 0\n  current_sprint: null\n  last_updated: \"{}\"
\nsprints: []",
                project_name,
                chrono::Utc::now().to_rfc3339()
            );
            std::fs::write(sprints_path, empty_sprints)?;
        } else {
            bail!(
                "{}\nRun {} or {} first",
                "Project not initialized.".red(),
                "autoflow init".bright_blue(),
                "autoflow create".bright_blue()
            );
        }
    }

    // Load sprints with retry/fix logic
    println!("\n{}", "Checking project status...".bright_cyan());
    let mut sprints_data = match SprintsYaml::load(sprints_path) {
        Ok(data) => data,
        Err(e) => {
            // SPRINTS.yml exists but is invalid - try to fix it
            println!("  {} SPRINTS.yml validation failed: {}", "⚠".yellow(), e.to_string().yellow());
            println!("  {} Attempting to fix validation errors...", "→".yellow());

            let fix_context = format!(
                r#"VALIDATION ERROR FOUND:
{}

TASK: Fix ALL validation errors in `.autoflow/SPRINTS.yml`

The file exists but fails validation (likely due to schema updates).

IMPORTANT: Fix ALL instances of missing fields throughout the entire file:
- If 'last_updated' is missing from project → add it with current timestamp
- If 'last_updated' is missing from ANY sprint → add to ALL sprints
- If 'workflow_type' is missing from ANY sprint → add to ALL sprints (default: IMPLEMENTATION)
- If 'type' is missing from ANY task → add to ALL tasks (default: IMPLEMENTATION)

Steps:
1. Read the existing file using the Read tool
2. Identify ALL validation errors (check project, ALL sprints, ALL tasks)
3. Fix ALL occurrences at once:
   - Add missing 'type' field to EVERY task (IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR, BUGFIX)
   - Add missing 'workflow_type' field to EVERY sprint (IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR)
   - Add missing 'last_updated' field to project AND every sprint (use current timestamp: "2025-11-06T17:30:00Z")
   - Fix enum values to match SCREAMING_SNAKE_CASE
4. Use the Write tool to save the corrected SPRINTS.yml

Example fixes:
- Project missing last_updated → add: last_updated: "2025-11-06T17:30:00Z"
- Sprint missing workflow_type → add: workflow_type: IMPLEMENTATION
- Sprint missing last_updated → add: last_updated: "2025-11-06T17:30:00Z"
- Task missing type → add: type: IMPLEMENTATION

Only fix what's broken - preserve all existing content and sprint progress."#,
                e
            );

            match autoflow_agents::execute_agent("make-sprints", &fix_context, 20, None).await {
                Ok(result) if result.success => {
                    // Try loading again
                    match SprintsYaml::load(sprints_path) {
                        Ok(fixed_data) => {
                            println!("  {} SPRINTS.yml fixed and validated", "✓".green());
                            fixed_data
                        }
                        Err(e2) => {
                            bail!("Failed to fix SPRINTS.yml: {}\n\nPlease manually fix the file or delete it and run 'autoflow create'", e2);
                        }
                    }
                }
                _ => {
                    bail!("Failed to fix SPRINTS.yml automatically.\n\nOriginal error: {}\n\nPlease manually fix the file or delete it and run 'autoflow create'", e);
                }
            }
        }
    };

    // Check if sprints are empty and IDEA.md exists - offer to generate
    if sprints_data.sprints.is_empty() {
        println!("{}", "No sprints found in SPRINTS.yml".yellow());

        if Path::new("IDEA.md").exists() {
            println!("{}", "Found IDEA.md - generating project setup...".bright_cyan());
            println!();

            // Check if docs exist
            let docs_exist = Path::new(".autoflow/docs/BUILD_SPEC.md").exists();

            if !docs_exist {
                println!("{}", "📚 Generating project documentation...".bright_cyan());
                println!("  Spawning make-docs agent...");

                let idea_content = std::fs::read_to_string("IDEA.md")?;
                let docs_context = format!(r#"Generate comprehensive project documentation from this idea:

{}

Create the following files in .autoflow/docs/:

CORE DOCUMENTATION (always):
1. .autoflow/docs/BUILD_SPEC.md - Detailed technical specification
2. .autoflow/docs/ARCHITECTURE.md - System architecture and design
3. .autoflow/docs/TESTING_STRATEGY.md - Testing approach and requirements
4. .autoflow/docs/ERROR_HANDLING.md - Error management patterns
5. .autoflow/docs/DEPLOYMENT.md - Deployment and operations guide

CONDITIONAL DOCUMENTATION (based on project type):
6. .autoflow/docs/API_SPEC.md - API endpoints and data models (if backend/API)
7. .autoflow/docs/UI_SPEC.md - UI/UX specifications (if frontend/UI)
8. .autoflow/docs/DATA_MODEL.md - Database schema and relationships (if database)
9. .autoflow/docs/STATE_MANAGEMENT.md - Frontend state patterns (if frontend framework)
10. .autoflow/docs/SECURITY.md - Security implementation (if backend/API)

Detect project type from the IDEA and generate appropriate documentation.
Always include references and links between documents.

IMPORTANT: All documentation files MUST be created in the .autoflow/docs/ directory, NOT in the project root.
"#, idea_content);

                match autoflow_agents::execute_agent("make-docs", &docs_context, 15, None).await {
                    Ok(result) if result.success => {
                        println!("  {} Documentation generated", "✓".green());
                    }
                    _ => {
                        bail!("Failed to generate documentation. Please run 'autoflow create' instead.");
                    }
                }
                println!();
            }

            // Generate sprints from docs
            println!("{}", "📋 Generating sprint plan...".bright_cyan());
            println!("  Spawning make-sprints agent...");

            let build_spec = std::fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
            let architecture = std::fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
            let api_spec = std::fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
            let ui_spec = std::fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();
            let data_model = std::fs::read_to_string(".autoflow/docs/DATA_MODEL.md").unwrap_or_default();
            let testing_strategy = std::fs::read_to_string(".autoflow/docs/TESTING_STRATEGY.md").unwrap_or_default();
            let error_handling = std::fs::read_to_string(".autoflow/docs/ERROR_HANDLING.md").unwrap_or_default();
            let state_management = std::fs::read_to_string(".autoflow/docs/STATE_MANAGEMENT.md").unwrap_or_default();
            let security = std::fs::read_to_string(".autoflow/docs/SECURITY.md").unwrap_or_default();
            let deployment = std::fs::read_to_string(".autoflow/docs/DEPLOYMENT.md").unwrap_or_default();

            let sprints_context = format!(r#"Generate a complete sprint plan from the following project documentation:

# BUILD_SPEC.md
{}

# ARCHITECTURE.md
{}

# API_SPEC.md
{}

# UI_SPEC.md
{}

# DATA_MODEL.md
{}

# TESTING_STRATEGY.md
{}

# ERROR_HANDLING.md
{}

# STATE_MANAGEMENT.md
{}

# SECURITY.md
{}

# DEPLOYMENT.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Reference specific sections from the docs (e.g., "See DATA_MODEL.md#UserSchema")
6. Output ONLY raw YAML - no markdown fences, no explanations
"#, build_spec, architecture, api_spec, ui_spec, data_model, testing_strategy, error_handling, state_management, security, deployment);

            // Retry loop for sprint generation with validation
            let max_retries = 2;
            let mut retry_count = 0;
            let mut last_error = String::new();

            loop {
                let (agent_name, context) = if retry_count == 0 {
                    // First attempt: Full generation
                    ("make-sprints", format!("{}\n\nIMPORTANT: Use the Write tool to save the SPRINTS.yml file directly to `.autoflow/SPRINTS.yml`. This avoids truncation issues with large files.", sprints_context))
                } else if retry_count == 1 && std::path::Path::new(sprints_path).exists() {
                    // First retry: Try focused fix if file exists
                    println!("  {} Attempting focused fix...", "→".yellow());
                    ("make-sprints", format!(
                        r#"VALIDATION ERROR FOUND:
{}

TASK: Fix the SPRINTS.yml file at `.autoflow/SPRINTS.yml`

1. Read the existing file using the Read tool
2. Identify and fix the validation errors (missing fields, wrong types, invalid enum values, etc.)
3. Use the Write tool to save the corrected SPRINTS.yml

Common fixes:
- Add missing 'type' field to tasks (IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR, BUGFIX)
- Add missing 'workflow_type' field to sprints (IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR)
- Fix enum values to match schema (SCREAMING_SNAKE_CASE)
- Add missing required fields (last_updated, etc.)
- Fix YAML syntax errors (quotes, indentation)

Only fix what's broken - preserve all existing content."#,
                        last_error
                    ))
                } else {
                    // Final retry: Full regeneration from scratch
                    println!("  {} Full regeneration...", "↻".yellow());
                    ("make-sprints", format!("{}\n\nPREVIOUS ATTEMPT FAILED:\n{}\n\nGenerate a complete, valid SPRINTS.yml from scratch. Use the Write tool to save to `.autoflow/SPRINTS.yml` directly.", sprints_context, last_error))
                };

                match autoflow_agents::execute_agent(agent_name, &context, 20, None).await {
                    Ok(result) if result.success => {
                        // Check if file was written directly
                        if std::path::Path::new(sprints_path).exists() {
                            println!("  {} Sprint plan generated and saved", "✓".green());

                            // Validate the written file
                            match SprintsYaml::load(sprints_path) {
                                Ok(validated_sprints) => {
                                    println!("  {} Validated SPRINTS.yml", "✓".green());
                                    sprints_data = validated_sprints;
                                    break;
                                }
                                Err(e) => {
                                    last_error = format!("YAML validation failed: {}", e);
                                    println!("  {} {}", "⚠".yellow(), last_error.yellow());
                                    retry_count += 1;

                                    if retry_count >= max_retries {
                                        bail!("Failed to generate valid SPRINTS.yml after {} attempts. Last error: {}", max_retries, last_error);
                                    }
                                    println!("  {} Retrying... (attempt {}/{})", "↻".yellow(), retry_count + 1, max_retries);
                                    continue;
                                }
                            }
                        } else {
                            // Fallback: extract from output if file wasn't written
                            println!("  {} Sprint plan generated (from output)", "✓".green());

                            let yaml_content = autoflow_utils::extract_yaml_from_output(&result.output);
                            match SprintsYaml::validate_and_fix(&yaml_content) {
                                Ok(validated_sprints) => {
                                    validated_sprints.save(sprints_path)?;
                                    println!("  {} Validated and saved to {}", "✓".green(), sprints_path.bright_blue());
                                    sprints_data = validated_sprints;
                                    break;
                                }
                                Err(e) => {
                                    last_error = format!("YAML extraction/validation failed: {}. Output may be truncated.", e);
                                    println!("  {} {}", "⚠".yellow(), last_error.yellow());
                                    retry_count += 1;

                                    if retry_count >= max_retries {
                                        bail!("Failed to generate valid SPRINTS.yml after {} attempts. Last error: {}\n\nTip: The output may be too large. The agent should use the Write tool instead.", max_retries, last_error);
                                    }
                                    println!("  {} Retrying with explicit Write tool instruction... (attempt {}/{})", "↻".yellow(), retry_count + 1, max_retries);
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        retry_count += 1;
                        if retry_count >= max_retries {
                            bail!("Failed to generate sprints after {} attempts. Please run 'autoflow create' instead.", max_retries);
                        }
                        println!("  {} Agent execution failed, retrying... (attempt {}/{})", "⚠".yellow(), retry_count + 1, max_retries);
                        continue;
                    }
                }
            }
            println!();
        } else {
            bail!(
                "{}\n{}\n\nRun one of:\n  {} - If you have IDEA.md\n  {} - Manual setup",
                "No sprints found and no IDEA.md".red(),
                "Cannot continue without project requirements.".red(),
                "autoflow create".bright_blue(),
                "autoflow init".bright_blue()
            );
        }
    }

    println!("{}", "Loading sprints...".bright_cyan());

    // Filter sprints based on flags - get indices instead of refs
    let sprint_indices: Vec<usize> = if let Some(sprint_id) = sprint {
        // Run specific sprint
        println!("Running sprint: {}", sprint_id.to_string().bright_blue());

        let idx = sprints_data
            .sprints
            .iter()
            .position(|s| s.id == sprint_id)
            .context(format!("Sprint {} not found", sprint_id))?;

        vec![idx]
    } else {
        // Run all pending or in-progress sprints
        let runnable: Vec<usize> = sprints_data
            .sprints
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.status == SprintStatus::Pending
                    || (s.status != SprintStatus::Done && s.status != SprintStatus::Blocked)
            })
            .map(|(idx, _)| idx)
            .collect();

        if runnable.is_empty() {
            println!(
                "\n{}",
                "No runnable sprints found. All sprints are either complete or blocked."
                    .yellow()
            );
            return Ok(());
        }

        println!(
            "Running {} sprint(s)",
            runnable.len().to_string().bright_green()
        );
        runnable
    };

    // Create orchestrator
    let max_iterations = 50;
    let orchestrator = Orchestrator::new(max_iterations).with_save_callback({
        move |_sprint| {
            // TODO: Save just the updated sprint, not whole file
            // For now, we'll save after all iterations complete
            Ok(())
        }
    });

    // Execute sprints
    if parallel && sprint_indices.len() > 1 {
        println!("\n{}", "Mode: Parallel execution".bright_green());

        // Extract sprints to run
        let mut sprints_to_run: Vec<_> = sprint_indices
            .iter()
            .map(|&idx| sprints_data.sprints[idx].clone())
            .collect();

        // Run in parallel
        let results = orchestrator.run_parallel(&mut sprints_to_run).await?;

        // Update original sprints with results
        for (i, &idx) in sprint_indices.iter().enumerate() {
            sprints_data.sprints[idx] = sprints_to_run[i].clone();
        }

        // Check results
        for (i, result) in results.iter().enumerate() {
            match result {
                Ok(_) => {
                    println!(
                        "{} Sprint {} completed",
                        "✅".green(),
                        sprints_to_run[i].id
                    );
                }
                Err(e) => {
                    println!(
                        "{} Sprint {} failed: {}",
                        "❌".red(),
                        sprints_to_run[i].id,
                        e
                    );
                }
            }
        }
    } else {
        // Run sequentially
        println!("\n{}", "Mode: Sequential execution".bright_green());

        for &idx in &sprint_indices {
            let sprint = &mut sprints_data.sprints[idx];

            println!(
                "\n{} {} - {}",
                "Running Sprint".bright_cyan(),
                sprint.id.to_string().bright_blue(),
                sprint.goal.bright_white()
            );

            let sprint_id = sprint.id;
            match orchestrator.run_sprint(sprint).await {
                Ok(_) => {
                    println!(
                        "{} Sprint {} completed successfully",
                        "✅".green(),
                        sprint_id
                    );
                }
                Err(e) => {
                    println!(
                        "{} Sprint {} failed: {}",
                        "❌".red(),
                        sprint_id,
                        e
                    );

                    // Continue with next sprint unless it's a critical error
                    if sprint.status == SprintStatus::Blocked {
                        println!(
                            "{} Sprint {} is blocked, skipping remaining sprints",
                            "⚠️".yellow(),
                            sprint_id
                        );
                    }
                }
            }
        }
    }

    // Save updated sprints
    println!("\n{}", "Saving progress...".bright_cyan());
    sprints_data.save(sprints_path)
        .context("Failed to save SPRINTS.yml")?;

    // Display summary
    println!("\n{}", "Summary".bright_cyan().bold());
    println!("────────────────────────────────────────");

    let done = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status == SprintStatus::Done)
        .count();
    let blocked = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status == SprintStatus::Blocked)
        .count();
    let in_progress = sprints_data
        .sprints
        .iter()
        .filter(|s| s.status != SprintStatus::Done
                 && s.status != SprintStatus::Blocked
                 && s.status != SprintStatus::Pending)
        .count();

    println!("{}: {}/{}", "Completed".green(), done, sprints_data.sprints.len());
    if in_progress > 0 {
        println!("{}: {}", "In Progress".blue(), in_progress);
    }
    if blocked > 0 {
        println!("{}: {}", "Blocked".red(), blocked);
    }

    println!("\n{}", "✨ AutoFlow session complete!".bright_green().bold());

    Ok(())
}
