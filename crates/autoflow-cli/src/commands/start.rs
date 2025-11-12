use anyhow::{bail, Context};
use autoflow_core::Orchestrator;
use autoflow_data::{SprintsYaml, SprintStatus};
use autoflow_utils::{
    check_for_updates, should_check_for_updates, prompt_and_update, update_check_timestamp,
    check_binary_update, prompt_and_install_binary_update,
};
use colored::*;
use std::path::{Path, PathBuf};

pub async fn run(parallel: bool, sprint: Option<u32>, live: bool) -> anyhow::Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("{} {}", "🚀 Starting AutoFlow".bright_cyan().bold(), format!("v{}", version).dimmed());

    // Live logging is now enabled by default
    let live_enabled = live;
    if live_enabled {
        println!("{}", "📡 Live logging enabled - streaming to .autoflow/.debug/live/".bright_green());
        std::env::set_var("AUTOFLOW_LIVE_LOGGING", "1");
    }

    // Check for updates (if enabled and interval has passed)
    if should_check_for_updates().unwrap_or(false) {
        // First check for binary updates
        match check_binary_update().await {
            Ok(Some(binary_update)) => {
                // Prompt user and install if they accept
                let updated = prompt_and_install_binary_update(&binary_update).await?;
                if updated {
                    // Binary was updated, restart is needed
                    println!("{}", "⚠️  Binary updated! Please restart the command.".bright_yellow().bold());
                    return Ok(());
                }
            }
            Ok(None) => {
                // No binary update available
            }
            Err(e) => {
                // Silently ignore binary update check failures
                tracing::debug!("Binary update check failed: {}", e);
            }
        }

        // Then check for agent/skill updates
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

    // If SPRINTS.yml doesn't exist, check for IDEA.md
    if !Path::new(sprints_path).exists() {
        if !Path::new("IDEA.md").exists() {
            bail!(
                "{}\nRun {} or {} first",
                "Project not initialized.".red(),
                "autoflow init".bright_blue(),
                "autoflow create".bright_blue()
            );
        }
        // If IDEA.md exists, we'll generate SPRINTS.yml below (skip to generation logic)
        println!("\n{}", "Initializing project from IDEA.md...".bright_cyan());
        std::fs::create_dir_all(".autoflow/docs")?;
    }

    // Load sprints with comprehensive validation to collect ALL errors
    println!("\n{}", "Checking project status...".bright_cyan());

    // Skip validation if file doesn't exist - will be generated below
    let mut sprints_data = if !Path::new(sprints_path).exists() {
        // File doesn't exist, will be generated from IDEA.md below
        // Create minimal structure to reach generation logic
        let current_dir = std::env::current_dir()?;
        let project_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Project")
            .to_string();

        SprintsYaml {
            project: autoflow_data::ProjectMetadata {
                name: project_name,
                version: "0.1.0".to_string(),
                description: "Generated from IDEA.md".to_string(),
                total_sprints: 0,
                current_sprint: None,
                last_updated: chrono::Utc::now(),
            },
            sprints: vec![],
        }
    } else {
        // File exists, validate and fix if needed
        let validation_errors = SprintsYaml::validate_all_errors(sprints_path);

        if let Err(all_errors) = validation_errors {
        // SPRINTS.yml exists but has validation errors
        println!("  {} SPRINTS.yml validation failed", "⚠".yellow());

        // Check if there are any in-progress sprints - if so, skip regeneration and resume
        if let Ok(existing_data) = SprintsYaml::load_without_validation(sprints_path) {
            let has_in_progress = existing_data.sprints.iter().any(|s| matches!(
                s.status,
                SprintStatus::WriteUnitTests
                    | SprintStatus::WriteCode
                    | SprintStatus::CodeReview
                    | SprintStatus::ReviewFix
                    | SprintStatus::RunUnitTests
                    | SprintStatus::UnitFix
                    | SprintStatus::WriteE2eTests
                    | SprintStatus::RunE2eTests
                    | SprintStatus::E2eFix
            ));

            if has_in_progress {
                println!("  {} Found in-progress sprints - resuming execution...", "→".bright_cyan());
                println!("  {} Skipping regeneration to preserve sprint progress", "ℹ".bright_cyan());
                existing_data
            } else {
                // No in-progress work, safe to attempt fix
                println!("  {} Attempting to fix validation errors...", "→".yellow());

        let fix_context = format!(
            r#"VALIDATION ERRORS FOUND:
{}

TASK: Fix ALL validation errors in `.autoflow/SPRINTS.yml`

The file exists but fails validation (likely due to schema updates).

IMPORTANT: Fix ALL instances of missing/incorrect fields throughout the entire file:
- If 'last_updated' is missing from project → add it with current timestamp
- If 'last_updated' is missing from ANY sprint → add to ALL sprints
- If 'workflow_type' is missing from ANY sprint → add to ALL sprints (default: IMPLEMENTATION)
- If 'type' is missing from ANY task → add to ALL tasks (default: IMPLEMENTATION)
- Fix any enum values to match SCREAMING_SNAKE_CASE

Steps:
1. Read the existing file using the Read tool
2. Review ALL the validation errors listed above
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
            all_errors
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
                bail!("Failed to fix SPRINTS.yml automatically.\n\nOriginal error: {}\n\nPlease manually fix the file or delete it and run 'autoflow create'", all_errors);
            }
        }
            }
        } else {
            // Failed to load even without validation
            bail!("SPRINTS.yml exists but cannot be loaded:\n{}\n\nPlease manually fix the file or delete it and run 'autoflow create'", all_errors);
        }
        } else {
            // Validation passed, load the file
            match SprintsYaml::load(sprints_path) {
                Ok(data) => data,
                Err(e) => bail!("Failed to load SPRINTS.yml: {}", e),
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

                let idea_content = std::fs::read_to_string("IDEA.md")?;
                let base_context = format!(r#"Generate comprehensive project documentation from this idea:

{}

IMPORTANT: All documentation files MUST be created in .autoflow/docs/ directory, NOT in the project root.
"#, idea_content);

                // Generate foundation docs (BUILD_SPEC, ARCHITECTURE with error handling)
                println!("  Spawning make-docs-foundation agent...");
                match autoflow_agents::execute_agent("make-docs-foundation", &base_context, 15, None).await {
                    Ok(result) => {
                        if result.success {
                            println!("  {} Foundation docs generated (BUILD_SPEC, ARCHITECTURE)", "✓".green());
                        } else {
                            println!("  {} Foundation agent completed with warnings", "⚠".yellow());
                        }
                    }
                    Err(e) => {
                        println!("  {} Failed to generate foundation docs: {}", "⚠".yellow(), e);
                        // Create minimal BUILD_SPEC as fallback
                        let idea_content = std::fs::read_to_string("IDEA.md")?;
                        let minimal_spec = format!(r#"# Build Specification

## Original Idea

{}

## Tech Stack
To be determined during sprint planning.

## Architecture
To be determined during implementation.
"#, idea_content);
                        std::fs::write(".autoflow/docs/BUILD_SPEC.md", minimal_spec)?;
                    }
                }

                // Generate API docs (API_SPEC with data model and security)
                println!("  Spawning make-docs-api agent...");
                match autoflow_agents::execute_agent("make-docs-api", &base_context, 15, None).await {
                    Ok(result) => {
                        if result.success {
                            println!("  {} API docs generated (API_SPEC with data model and security)", "✓".green());
                        } else {
                            println!("  {} API agent completed with warnings", "⚠".yellow());
                        }
                    }
                    Err(e) => {
                        println!("  {} API docs generation failed (may not be applicable): {}", "⚠".yellow(), e);
                    }
                }

                // Generate UI docs (UI_SPEC with state management, TESTING_STRATEGY)
                println!("  Spawning make-docs-ui agent...");
                match autoflow_agents::execute_agent("make-docs-ui", &base_context, 15, None).await {
                    Ok(result) => {
                        if result.success {
                            println!("  {} UI docs generated (UI_SPEC, TESTING_STRATEGY)", "✓".green());
                        } else {
                            println!("  {} UI agent failed - check .autoflow/.debug/ logs for details", "⚠".yellow());
                            // Create minimal fallback if UI_SPEC doesn't exist
                            if !std::path::Path::new(".autoflow/docs/UI_SPEC.md").exists() {
                                let idea_content = std::fs::read_to_string("IDEA.md").unwrap_or_default();
                                let minimal_ui = format!(r#"# UI Specification

## Original Idea

{}

## UI Pages
To be determined during implementation.

## Design System
To be determined during implementation.
"#, idea_content);
                                std::fs::write(".autoflow/docs/UI_SPEC.md", minimal_ui)?;
                            }
                            if !std::path::Path::new(".autoflow/docs/TESTING_STRATEGY.md").exists() {
                                let minimal_testing = r#"# Testing Strategy

## Framework Choices
- Unit: Vitest/Jest with React Testing Library
- E2E: Playwright/Cypress
- Backend: PHPUnit/Pest

## Coverage Requirements
- Overall: 80% minimum
- Critical paths: 100%

## What to Test
- Unit: Business logic, utilities, components, hooks
- Integration: API endpoints, database ops, auth flows
- E2E: Critical user flows
"#;
                                std::fs::write(".autoflow/docs/TESTING_STRATEGY.md", minimal_testing)?;
                            }
                        }
                    }
                    Err(e) => {
                        println!("  {} UI docs generation failed: {}", "⚠".yellow(), e);
                        println!("  {} Creating minimal fallback docs...", "→".yellow());
                        // Create minimal fallback docs
                        let idea_content = std::fs::read_to_string("IDEA.md").unwrap_or_default();
                        let minimal_ui = format!(r#"# UI Specification

## Original Idea

{}

## UI Pages
To be determined during implementation.

## Design System
To be determined during implementation.
"#, idea_content);
                        std::fs::write(".autoflow/docs/UI_SPEC.md", minimal_ui)?;

                        let minimal_testing = r#"# Testing Strategy

## Framework Choices
- Unit: Vitest/Jest with React Testing Library
- E2E: Playwright/Cypress
- Backend: PHPUnit/Pest

## Coverage Requirements
- Overall: 80% minimum
- Critical paths: 100%

## What to Test
- Unit: Business logic, utilities, components, hooks
- Integration: API endpoints, database ops, auth flows
- E2E: Critical user flows
"#;
                        std::fs::write(".autoflow/docs/TESTING_STRATEGY.md", minimal_testing)?;
                    }
                }

                println!();
            }

            // Generate sprints from docs
            println!("{}", "📋 Generating sprint plan...".bright_cyan());
            println!("  Spawning make-sprints agent...");

            // Read the consolidated documentation files
            let build_spec = std::fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
            let architecture = std::fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
            let api_spec = std::fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
            let ui_spec = std::fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();
            let testing_strategy = std::fs::read_to_string(".autoflow/docs/TESTING_STRATEGY.md").unwrap_or_default();

            // Load JSON schema - try global location first, then embedded
            let json_schema = {
                let home = std::env::var("HOME").expect("HOME environment variable not set");
                let schema_path = PathBuf::from(home).join(".autoflow/schemas/sprints.schema.json");

                if let Ok(content) = std::fs::read_to_string(&schema_path) {
                    content
                } else {
                    // Fall back to embedded schema (compiled into binary)
                    include_str!("../../../../schemas/sprints.schema.json").to_string()
                }
            };

            let sprints_context = format!(r#"Generate a complete sprint plan from the following project documentation:

# JSON SCHEMA (CRITICAL - MUST FOLLOW EXACTLY)

Your output MUST validate against this JSON schema:

```json
{}
```

IMPORTANT SCHEMA REQUIREMENTS:
- All required fields MUST be present
- All enum values must match EXACTLY (case-sensitive, use SCREAMING_SNAKE_CASE)
- Valid task types: IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR, BUGFIX
- Valid workflow types: IMPLEMENTATION, DOCUMENTATION, TEST, INFRASTRUCTURE, REFACTOR
- Valid sprint statuses: PENDING, WRITE_UNIT_TESTS, WRITE_CODE, CODE_REVIEW, REVIEW_FIX, RUN_UNIT_TESTS, UNIT_FIX, WRITE_E2E_TESTS, RUN_E2E_TESTS, E2E_FIX, COMPLETE, DONE, BLOCKED
- All sprints must start with status: PENDING
- Include last_updated timestamp in ISO 8601 format
- CRITICAL: dependencies MUST be an array of strings (sprint IDs), NOT maps/objects
  CORRECT:   dependencies: ["1", "2"]
  WRONG:     dependencies: [{{Sprint 1: Infrastructure}}]
  WRONG:     dependencies: [Sprint 1: Infrastructure]

# PROJECT DOCUMENTATION

Generate a complete sprint plan from the following project documentation:

# BUILD_SPEC.md
{}

# ARCHITECTURE.md
{}

# API_SPEC.md
{}

# UI_SPEC.md
{}

# TESTING_STRATEGY.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Reference specific sections from the docs (e.g., "See API_SPEC.md#UserEndpoints" or "See ARCHITECTURE.md#ErrorHandling")
6. Note: Data model is in API_SPEC.md, error handling is in ARCHITECTURE.md, state management is in UI_SPEC.md, security is in API_SPEC.md
7. Output ONLY raw YAML - no markdown fences, no explanations

The agent definition already contains the full YAML format. Just output the actual YAML content.
"#, json_schema, build_spec, architecture, api_spec, ui_spec, testing_strategy);

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
                        r#"JSON SCHEMA (YOUR OUTPUT MUST VALIDATE AGAINST THIS):

```json
{}
```

VALIDATION ERROR FOUND:
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
                        json_schema,
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

    // Final schema validation before starting sprint execution
    println!("{}", "Validating sprint configuration...".bright_cyan());
    if let Err(validation_errors) = SprintsYaml::validate_all_errors(sprints_path) {
        bail!(
            "{}\n\n{}\n\n{}",
            "SPRINTS.yml failed final schema validation".red().bold(),
            validation_errors,
            "Please fix the validation errors manually or delete SPRINTS.yml and run 'autoflow create'".yellow()
        );
    }
    println!("  {} Schema validation passed", "✓".green());
    println!();

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
        // Run all pending or in-progress sprints, respecting dependencies and must_complete_first

        // First, check if any sprint with must_complete_first is BLOCKED
        // If so, prioritize running it (orchestrator will invoke blocker-resolver)
        let blocked_critical_sprint = sprints_data
            .sprints
            .iter()
            .enumerate()
            .find(|(_, s)| s.must_complete_first && s.status == SprintStatus::Blocked);

        if let Some((idx, sprint)) = blocked_critical_sprint {
            println!(
                "\n{}",
                format!("Critical sprint {} is BLOCKED - attempting to resolve...", sprint.id)
                    .yellow().bold()
            );
            println!("{}", "Invoking blocker-resolver to diagnose and fix the issue.".cyan());

            // Run only the blocked sprint - orchestrator will handle blocker-resolver
            vec![idx]
        } else {

        // Check dependencies and must_complete_first
        let runnable: Vec<usize> = sprints_data
            .sprints
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                // Must be runnable status (BLOCKED is runnable - blocker-resolver handles it)
                let is_runnable_status = s.status != SprintStatus::Done;

                if !is_runnable_status {
                    return false;
                }

                // Check if all dependencies are satisfied
                let dependencies_satisfied = s.dependencies.iter().all(|dep_id| {
                    sprints_data
                        .sprints
                        .iter()
                        .find(|other| other.id.to_string() == *dep_id)
                        .map(|dep| dep.status == SprintStatus::Done)
                        .unwrap_or(true) // If dependency not found, allow (to not break things)
                });

                dependencies_satisfied
            })
            .map(|(idx, _)| idx)
            .collect();

        // Return runnable sprints (even if empty - continuous mode loop will handle it)
        println!(
            "Running {} sprint(s)",
            runnable.len().to_string().bright_green()
        );
        runnable
        }
    };

    // Create orchestrator
    let max_iterations = 50;

    // Get current directory for git commits
    let project_path = std::env::current_dir()?;

    // Only use save callback in sequential mode to avoid race conditions
    // In parallel mode, we save once after all sprints complete
    let orchestrator = if parallel && sprint_indices.len() > 1 {
        // Parallel mode: no save callback (prevents race conditions)
        Orchestrator::new(max_iterations)
            .with_project_path(project_path)
            .with_auto_commit(true)
    } else {
        // Sequential mode: save after each iteration
        let sprints_path_for_callback = sprints_path.to_string();
        Orchestrator::new(max_iterations)
            .with_project_path(project_path)
            .with_auto_commit(true)
            .with_save_callback({
                move |updated_sprint| {
                    // Load current file, update the specific sprint, save back
                    match SprintsYaml::load(&sprints_path_for_callback) {
                        Ok(mut data) => {
                            // Find and update the sprint
                            if let Some(sprint) = data.sprints.iter_mut().find(|s| s.id == updated_sprint.id) {
                                *sprint = updated_sprint.clone();
                                data.project.last_updated = chrono::Utc::now();
                                data.save(&sprints_path_for_callback)?;
                            }
                            Ok(())
                        }
                        Err(e) => {
                            // Non-fatal: log but don't fail the sprint
                            tracing::warn!("Failed to save sprint progress: {}", e);
                            Ok(())
                        }
                    }
                }
            })
    };

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

        // Save progress after parallel execution
        sprints_data.save(sprints_path)
            .context("Failed to save sprint progress")?;
    } else {
        // Run sequentially - keep running until no more runnable sprints
        println!("\n{}", "Mode: Sequential execution (continuous)".bright_green());

        // If specific sprint(s) requested, run only those
        let mut indices_to_run: Vec<usize> = if !sprint_indices.is_empty() {
            sprint_indices.clone()
        } else {
            // Continuous mode - re-evaluate after each sprint
            vec![]
        };

        // Sort indices by priority (in-progress, must_complete_first, then lowest ID)
        if !indices_to_run.is_empty() {
            indices_to_run.sort_by_key(|&idx| {
                let sprint = &sprints_data.sprints[idx];
                let is_in_progress = sprint.status != SprintStatus::Pending && sprint.status != SprintStatus::Done;
                let is_critical = sprint.must_complete_first;
                (!is_in_progress, !is_critical, sprint.id)
            });
        }

        if !indices_to_run.is_empty() {
            // Run specific sprint(s) first
            for idx in indices_to_run {
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

                        // If sprint is blocked, note it but continue
                        if sprint.status == SprintStatus::Blocked {
                            println!(
                                "{} Sprint {} is blocked",
                                "⚠️".yellow(),
                                sprint_id
                            );
                        }
                    }
                }

                // Save progress after each sprint
                sprints_data.save(sprints_path)
                    .context("Failed to save sprint progress")?;
            }

            // After running specific sprints, continue in continuous mode if no --sprint flag was provided
            // (This handles the case where we ran a blocked critical sprint and should continue)
            if sprint.is_none() {
                println!("\n{}", "Transitioning to continuous mode...".bright_green());
            } else {
                // Specific sprint(s) requested via --sprint flag, exit after completing them
                return Ok(());
            }
        }

        // Continuous mode loop
        loop {
                // Re-evaluate runnable sprints after each completion
                let runnable: Vec<usize> = sprints_data
                .sprints
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    // Must be runnable status (BLOCKED is runnable now - blocker-resolver will handle it)
                    let is_runnable_status = s.status != SprintStatus::Done;

                    if !is_runnable_status {
                        return false;
                    }

                    // Check if any must_complete_first sprint is not done
                    // If so, only allow that sprint to run (blocks all others)
                    let has_incomplete_critical = sprints_data
                        .sprints
                        .iter()
                        .any(|other| other.must_complete_first && other.status != SprintStatus::Done);

                    if has_incomplete_critical && !s.must_complete_first {
                        return false; // Block non-critical sprints when critical sprint is incomplete
                    }

                    // Check if all dependencies are satisfied
                    let dependencies_satisfied = s.dependencies.iter().all(|dep_id| {
                        sprints_data
                            .sprints
                            .iter()
                            .find(|other| other.id.to_string() == *dep_id)
                            .map(|dep| dep.status == SprintStatus::Done)
                            .unwrap_or(true)
                    });

                    dependencies_satisfied
                })
                .map(|(idx, _)| idx)
                .collect();

            if runnable.is_empty() {
                println!("\n{}", "No more runnable sprints.".yellow());
                break;
            }

            // Select sprint with priority:
            // 1. In-progress sprints (finish what we started)
            // 2. must_complete_first sprints (critical foundation work)
            // 3. Lowest ID (maintain sequential order)
            let idx = *runnable.iter().min_by_key(|&&i| {
                let sprint = &sprints_data.sprints[i];
                let is_in_progress = sprint.status != SprintStatus::Pending && sprint.status != SprintStatus::Done;
                let is_critical = sprint.must_complete_first;

                // Priority tuple: (not in_progress, not critical, id)
                // Lower values are selected first, so we negate booleans to prioritize true values
                // false=0 true=1, so !true=0 (selected first), !false=1 (selected last)
                (!is_in_progress, !is_critical, sprint.id)
            }).unwrap();
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

                    // If sprint is blocked, stop execution
                    if sprint.status == SprintStatus::Blocked {
                        println!(
                            "{} Sprint {} is blocked, stopping execution",
                            "⚠️".yellow(),
                            sprint_id
                        );
                        break;
                    }
                }
            }

            // Save progress after each sprint
            sprints_data.save(sprints_path)
                .context("Failed to save sprint progress")?;
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
