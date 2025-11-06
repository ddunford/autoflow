use anyhow::{Context, Result, bail};
use autoflow_agents::execute_agent;
use autoflow_data::{SprintsYaml, SprintStatus};
use autoflow_utils::{extract_yaml_from_output, Paths};
use colored::*;
use std::fs;
use std::path::Path;

pub async fn run(instruction: String) -> Result<()> {
    println!("{}", "🔄 Pivoting project based on your feedback...".bright_cyan().bold());
    println!();

    // Check if project is initialized
    if !Path::new(".autoflow/docs").exists() {
        bail!(
            "{}\nRun {} first",
            "Project not initialized.".red(),
            "autoflow create".bright_blue()
        );
    }

    // Read all existing documentation
    println!("{}", "📖 Reading current documentation...".bright_cyan());
    let build_spec = fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
    let architecture = fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
    let api_spec = fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
    let ui_spec = fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();
    let data_model = fs::read_to_string(".autoflow/docs/DATA_MODEL.md").unwrap_or_default();
    let testing_strategy = fs::read_to_string(".autoflow/docs/TESTING_STRATEGY.md").unwrap_or_default();
    let error_handling = fs::read_to_string(".autoflow/docs/ERROR_HANDLING.md").unwrap_or_default();
    let state_management = fs::read_to_string(".autoflow/docs/STATE_MANAGEMENT.md").unwrap_or_default();
    let security = fs::read_to_string(".autoflow/docs/SECURITY.md").unwrap_or_default();
    let deployment = fs::read_to_string(".autoflow/docs/DEPLOYMENT.md").unwrap_or_default();

    let doc_count = [
        &build_spec, &architecture, &api_spec, &ui_spec, &data_model,
        &testing_strategy, &error_handling, &state_management, &security, &deployment
    ].iter().filter(|s| !s.is_empty()).count();

    println!("  {} Read {} documentation files", "✓".green(), doc_count.to_string().bright_blue());
    println!();

    // Check if sprints exist and save their current state
    let sprints_exist = Path::new(Paths::SPRINTS_YML).exists();
    let sprint_states = if sprints_exist {
        println!("{}", "📋 Saving current sprint states...".bright_cyan());
        match SprintsYaml::load(Paths::SPRINTS_YML) {
            Ok(data) => {
                let states: Vec<(u32, SprintStatus)> = data.sprints.iter()
                    .map(|s| (s.id, s.status.clone()))
                    .collect();
                println!("  {} Saved {} sprint states", "✓".green(), states.len().to_string().bright_blue());
                println!();
                Some(states)
            }
            Err(_) => None
        }
    } else {
        None
    };

    // Update documentation based on user feedback
    println!("{}", "🤖 Updating documentation based on your feedback...".bright_cyan());
    println!("  Spawning make-docs agent...");

    let docs_context = format!(r#"Update the project documentation based on this feedback:

USER FEEDBACK:
{}

CURRENT DOCUMENTATION:

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

INSTRUCTIONS:
1. Read the user's feedback carefully
2. Identify which documentation files need updates
3. Update the relevant sections to address the feedback
4. Maintain consistency across all documentation files
5. Preserve existing content that's still valid
6. Only create files that are relevant to the project

Update or create the following files in .autoflow/docs/:
- BUILD_SPEC.md (always)
- ARCHITECTURE.md (always)
- TESTING_STRATEGY.md (always)
- ERROR_HANDLING.md (always)
- DEPLOYMENT.md (always)
- API_SPEC.md (if backend/API)
- UI_SPEC.md (if frontend/UI)
- DATA_MODEL.md (if database)
- STATE_MANAGEMENT.md (if frontend framework)
- SECURITY.md (if backend/API)

IMPORTANT: All documentation files MUST be created in .autoflow/docs/ directory.
"#, instruction, build_spec, architecture, api_spec, ui_spec, data_model,
    testing_strategy, error_handling, state_management, security, deployment);

    match execute_agent("make-docs", &docs_context, 20, None).await {
        Ok(result) => {
            if result.success {
                println!("  {} Documentation updated", "✓".green());
            } else {
                println!("  {} Agent completed with warnings", "⚠".yellow());
            }
        }
        Err(e) => {
            println!("  {} Failed to update docs: {}", "✗".red(), e);
            bail!("Documentation update failed");
        }
    }
    println!();

    // Regenerate sprints if they exist
    if sprints_exist {
        println!("{}", "📋 Regenerating sprint plan with updated documentation...".bright_cyan());
        println!("  Spawning make-sprints agent...");

        // Re-read updated documentation
        let build_spec = fs::read_to_string(".autoflow/docs/BUILD_SPEC.md").unwrap_or_default();
        let architecture = fs::read_to_string(".autoflow/docs/ARCHITECTURE.md").unwrap_or_default();
        let api_spec = fs::read_to_string(".autoflow/docs/API_SPEC.md").unwrap_or_default();
        let ui_spec = fs::read_to_string(".autoflow/docs/UI_SPEC.md").unwrap_or_default();
        let data_model = fs::read_to_string(".autoflow/docs/DATA_MODEL.md").unwrap_or_default();
        let testing_strategy = fs::read_to_string(".autoflow/docs/TESTING_STRATEGY.md").unwrap_or_default();
        let error_handling = fs::read_to_string(".autoflow/docs/ERROR_HANDLING.md").unwrap_or_default();
        let state_management = fs::read_to_string(".autoflow/docs/STATE_MANAGEMENT.md").unwrap_or_default();
        let security = fs::read_to_string(".autoflow/docs/SECURITY.md").unwrap_or_default();
        let deployment = fs::read_to_string(".autoflow/docs/DEPLOYMENT.md").unwrap_or_default();
        let integration_guide = fs::read_to_string(".autoflow/INTEGRATION_GUIDE.md").unwrap_or_default();

        let sprints_context = format!(r#"Generate a complete sprint plan from the following UPDATED project documentation:

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

# INTEGRATION_GUIDE.md
{}

IMPORTANT:
1. Read the documentation above carefully
2. Break down the features into logical sprints
3. Each sprint task should LINK to the documentation section it implements
4. Follow TDD workflow: Tests → Implementation → Review
5. Reference specific sections from the docs (e.g., "See DATA_MODEL.md#UserSchema")
6. Output ONLY raw YAML - no markdown fences, no explanations

The agent definition already contains the full YAML format. Just output the actual YAML content.
"#, build_spec, architecture, api_spec, ui_spec, data_model, testing_strategy,
    error_handling, state_management, security, deployment, integration_guide);

        match execute_agent("make-sprints", &sprints_context, 20, None).await {
            Ok(result) => {
                if result.success {
                    println!("  {} Sprint plan regenerated", "✓".green());

                    // Save the output to SPRINTS.yml
                    let yaml_content = extract_yaml_from_output(&result.output);

                    // If we have saved sprint states, try to restore them
                    if let Some(ref states) = sprint_states {
                        match restore_sprint_states(&yaml_content, states) {
                            Ok(updated_yaml) => {
                                fs::write(Paths::SPRINTS_YML, updated_yaml)?;
                                println!("  {} Restored sprint states (kept {} active/completed sprints)",
                                    "✓".green(),
                                    states.iter().filter(|(_, s)| !matches!(s, SprintStatus::Pending | SprintStatus::Blocked)).count()
                                );
                            }
                            Err(_) => {
                                // If restoration fails, just save the new sprints
                                fs::write(Paths::SPRINTS_YML, yaml_content)?;
                                println!("  {} Could not restore sprint states - all sprints reset to PENDING", "⚠".yellow());
                            }
                        }
                    } else {
                        fs::write(Paths::SPRINTS_YML, yaml_content)?;
                    }

                    println!("  {} Saved to {}", "✓".green(), Paths::SPRINTS_YML.bright_blue());
                } else {
                    println!("  {} Failed to regenerate sprints", "✗".red());
                    bail!("Sprint regeneration failed");
                }
            }
            Err(e) => {
                println!("  {} Failed to regenerate sprints: {}", "✗".red(), e);
                bail!("Sprint regeneration failed");
            }
        }
        println!();
    }

    // Summary
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  ✅ Pivot Complete!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    println!("{}", "📝 What changed:".bright_cyan());
    println!("  • Documentation updated based on your feedback");
    if sprints_exist {
        println!("  • Sprint plan regenerated from updated docs");
        if sprint_states.is_some() {
            println!("  • Existing sprint states preserved where possible");
        }
    }
    println!();

    println!("{}", "🚀 Next Steps:".bright_cyan());
    println!("  1. Review the updated documentation:");
    println!("     {}", "cat .autoflow/docs/BUILD_SPEC.md".bright_blue());
    println!("     {}", "cat .autoflow/docs/ARCHITECTURE.md".bright_blue());
    if sprints_exist {
        println!();
        println!("  2. Review the updated sprint plan:");
        println!("     {}", "autoflow sprints list".bright_blue());
        println!("     {}", "cat .autoflow/SPRINTS.yml".bright_blue());
        println!();
        println!("  3. Continue development:");
        println!("     {}", "autoflow start --parallel".bright_blue());
    } else {
        println!();
        println!("  2. Generate sprints when ready:");
        println!("     {}", "autoflow create".bright_blue());
    }
    println!();

    Ok(())
}

/// Try to restore sprint states from old sprints into new sprint plan
fn restore_sprint_states(new_yaml: &str, old_states: &[(u32, SprintStatus)]) -> Result<String> {
    use autoflow_data::SprintsYaml;

    // Parse the new YAML
    let mut sprints_data: SprintsYaml = serde_yaml::from_str(new_yaml)
        .context("Failed to parse new sprints YAML")?;

    // Create a map of old states
    let state_map: std::collections::HashMap<u32, SprintStatus> = old_states.iter()
        .cloned()
        .collect();

    // Update sprint states where IDs match
    for sprint in &mut sprints_data.sprints {
        if let Some(old_status) = state_map.get(&sprint.id) {
            // Preserve all states except PENDING and BLOCKED
            // PENDING and BLOCKED get reset to PENDING
            match old_status {
                SprintStatus::Pending | SprintStatus::Blocked => {
                    sprint.status = SprintStatus::Pending;
                }
                _ => {
                    // Preserve active/completed states
                    sprint.status = old_status.clone();
                }
            }
        }
    }

    // Serialize back to YAML
    let updated_yaml = serde_yaml::to_string(&sprints_data)
        .context("Failed to serialize updated sprints")?;

    Ok(updated_yaml)
}
