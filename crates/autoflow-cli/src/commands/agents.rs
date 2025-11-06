use colored::*;
use std::fs;
use std::path::PathBuf;

pub async fn run(detailed: bool) -> anyhow::Result<()> {
    println!("{}", "🤖 Available Agents".bright_cyan().bold());

    // Load agents from ~/.claude/agents/
    let home = std::env::var("HOME")?;
    let agents_dir = PathBuf::from(home).join(".claude/agents");

    if !agents_dir.exists() {
        println!("\n{}", "No agents directory found".yellow());
        println!("Expected location: {}", agents_dir.display());
        return Ok(());
    }

    // Read all .agent.md files
    let entries = fs::read_dir(&agents_dir)?;
    let mut agents: Vec<(String, String, String)> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md")
            && path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.ends_with(".agent"))
                .unwrap_or(false)
        {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .trim_end_matches(".agent")
                .to_string();

            let content = fs::read_to_string(&path)?;

            // Parse frontmatter
            let description = extract_frontmatter_field(&content, "description")
                .unwrap_or("No description".to_string());
            let model = extract_frontmatter_field(&content, "model")
                .unwrap_or("claude-sonnet-4-5-20250929".to_string());

            agents.push((name, description, model));
        }
    }

    agents.sort_by(|a, b| a.0.cmp(&b.0));

    println!("\nFound {} agents\n", agents.len().to_string().bright_blue());

    for (name, description, model) in &agents {
        if detailed {
            println!("{}", format!("━━━━ {} ━━━━", name).bright_cyan());
            println!("  {}: {}", "Description".bold(), description);
            println!("  {}: {}", "Model".bold(), model.bright_blue());
            println!();
        } else {
            println!("  {} - {}", name.bright_blue(), description);
        }
    }

    if !detailed {
        println!("\n{}", "Use --detailed for more information".bright_black());
    }

    Ok(())
}

fn extract_frontmatter_field(content: &str, field: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_frontmatter = false;

    for line in lines {
        if line.trim() == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                continue;
            } else {
                break;
            }
        }

        if in_frontmatter && line.starts_with(&format!("{}:", field)) {
            return Some(
                line.trim_start_matches(&format!("{}:", field))
                    .trim()
                    .to_string(),
            );
        }
    }

    None
}
