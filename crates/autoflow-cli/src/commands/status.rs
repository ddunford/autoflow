use autoflow_data::{SprintsYaml, SprintStatus};
use colored::*;
use std::path::Path;

pub async fn run(json: bool) -> anyhow::Result<()> {
    // Check if initialized
    if !Path::new(".autoflow/SPRINTS.yml").exists() {
        println!("{}", "📊 AutoFlow Status".bright_cyan().bold());
        println!("\n{}", "No sprints found. Run 'autoflow init' first.".yellow());
        return Ok(());
    }

    // Load sprints
    let sprints = SprintsYaml::load(".autoflow/SPRINTS.yml")?;

    if json {
        // JSON output
        let json_str = serde_json::to_string_pretty(&sprints)?;
        println!("{}", json_str);
        return Ok(());
    }

    // Human-readable output
    println!("{}", "📊 AutoFlow Status".bright_cyan().bold());
    println!();
    println!("{}: {}", "Project".bright_white().bold(), sprints.project.name.bright_blue());
    println!("{}: {}", "Total Sprints".bright_white().bold(), sprints.project.total_sprints);

    if let Some(current) = sprints.project.current_sprint {
        println!("{}: {}", "Current Sprint".bright_white().bold(), current.to_string().bright_green());
    } else {
        println!("{}: {}", "Current Sprint".bright_white().bold(), "None".yellow());
    }

    println!("{}: {}", "Last Updated".bright_white().bold(), sprints.project.last_updated.format("%Y-%m-%d %H:%M:%S"));

    if sprints.sprints.is_empty() {
        println!("\n{}", "No sprints defined yet.".yellow());
        println!("Create sprints by:");
        println!("  1. Writing {} with requirements", "BUILD_SPEC.md".bright_blue());
        println!("  2. Running {} to generate sprints", "autoflow start".bright_blue());
        return Ok(());
    }

    // Display sprints
    println!("\n{}", "Sprints:".bright_white().bold());
    println!("{}", "─".repeat(80).bright_black());

    for sprint in &sprints.sprints {
        let status_str = format!("{:?}", sprint.status);
        let status_colored = match sprint.status {
            SprintStatus::Done => status_str.green(),
            SprintStatus::Blocked => status_str.red(),
            SprintStatus::Pending => status_str.yellow(),
            _ => status_str.bright_blue(),
        };

        println!(
            "{} {} {} {}",
            format!("Sprint {}", sprint.id).bright_white().bold(),
            status_colored,
            "-".bright_black(),
            sprint.goal
        );

        println!(
            "  {} {} {} {} {} {}",
            "Effort:".bright_black(),
            sprint.total_effort,
            "│".bright_black(),
            "Tasks:".bright_black(),
            sprint.tasks.len(),
            "│".bright_black(),
        );

        if !sprint.deliverables.is_empty() {
            println!("  {} {}", "Deliverables:".bright_black(), sprint.deliverables.join(", "));
        }

        if let Some(started) = sprint.started {
            println!("  {} {}", "Started:".bright_black(), started.format("%Y-%m-%d %H:%M"));
        }

        if let Some(completed) = sprint.completed_at {
            println!("  {} {}", "Completed:".bright_black(), completed.format("%Y-%m-%d %H:%M"));
        }

        println!();
    }

    // Summary
    let total = sprints.sprints.len();
    let done = sprints.sprints.iter().filter(|s| s.status == SprintStatus::Done).count();
    let in_progress = sprints.sprints.iter().filter(|s| {
        !matches!(s.status, SprintStatus::Done | SprintStatus::Pending | SprintStatus::Blocked)
    }).count();
    let blocked = sprints.sprints.iter().filter(|s| s.status == SprintStatus::Blocked).count();

    println!("{}", "Summary:".bright_white().bold());
    println!("  {} {}/{}", "Completed:".green(), done, total);
    if in_progress > 0 {
        println!("  {} {}", "In Progress:".bright_blue(), in_progress);
    }
    if blocked > 0 {
        println!("  {} {}", "Blocked:".red(), blocked);
    }

    Ok(())
}
