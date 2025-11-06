use colored::*;
use std::fs;
use std::path::PathBuf;

pub async fn run() -> anyhow::Result<()> {
    println!("{}", "🛠️  Available Skills".bright_cyan().bold());

    // Load skills from ~/.claude/skills/
    let home = std::env::var("HOME")?;
    let skills_dir = PathBuf::from(home).join(".claude/skills");

    if !skills_dir.exists() {
        println!("\n{}", "No skills directory found".yellow());
        println!("Expected location: {}", skills_dir.display());
        return Ok(());
    }

    // Read all .md files
    let entries = fs::read_dir(&skills_dir)?;
    let mut skills: Vec<(String, String)> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Read first non-empty line as description
            let content = fs::read_to_string(&path)?;
            let description = content
                .lines()
                .skip_while(|l| l.trim().is_empty() || l.starts_with('#'))
                .next()
                .unwrap_or("No description")
                .trim()
                .to_string();

            skills.push((name, description));
        }
    }

    skills.sort_by(|a, b| a.0.cmp(&b.0));

    println!("\nFound {} skills\n", skills.len().to_string().bright_blue());

    for (name, description) in &skills {
        println!("  {} - {}", name.bright_blue(), description);
    }

    Ok(())
}
