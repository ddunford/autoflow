/// Auto-sync agents and skills to ~/.claude/ on startup
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Sync agents and skills from source to ~/.claude/
/// This ensures they're always up-to-date without manual reinstall
pub async fn sync_agents_and_skills() -> Result<()> {
    // Find source directories (relative to binary or in ./agents and ./skills)
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().context("Failed to get binary directory")?;

    let possible_agent_sources = vec![
        PathBuf::from("./agents"),
        exe_dir.join("../agents"),
        exe_dir.join("../../agents"), // For development builds in target/release
    ];

    let possible_skill_sources = vec![
        PathBuf::from("./skills"),
        exe_dir.join("../skills"),
        exe_dir.join("../../skills"),
    ];

    // Find existing source directories
    let agent_source = possible_agent_sources.iter().find(|p| p.exists());
    let skill_source = possible_skill_sources.iter().find(|p| p.exists());

    // Claude directories
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let claude_agents_dir = PathBuf::from(&home).join(".claude/agents");
    let claude_skills_dir = PathBuf::from(&home).join(".claude/skills");

    // Create directories if they don't exist
    fs::create_dir_all(&claude_agents_dir)?;
    fs::create_dir_all(&claude_skills_dir)?;

    // Sync agents
    if let Some(source) = agent_source {
        sync_agents(source, &claude_agents_dir)?;
    }

    // Sync skills
    if let Some(source) = skill_source {
        sync_skills(source, &claude_skills_dir)?;
    }

    Ok(())
}

fn sync_agents(source: &PathBuf, dest: &PathBuf) -> Result<()> {
    let entries = fs::read_dir(source)
        .context(format!("Failed to read agents directory: {:?}", source))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let filename = path.file_stem()
                .and_then(|s| s.to_str())
                .context("Invalid agent filename")?;

            // Copy with .agent.md suffix to avoid conflicts with user's agents
            let dest_path = dest.join(format!("{}.agent.md", filename));

            // Only copy if source is newer or dest doesn't exist
            let should_copy = if dest_path.exists() {
                let source_modified = fs::metadata(&path)?.modified()?;
                let dest_modified = fs::metadata(&dest_path)?.modified()?;
                source_modified > dest_modified
            } else {
                true
            };

            if should_copy {
                fs::copy(&path, &dest_path)
                    .context(format!("Failed to copy agent: {:?} -> {:?}", path, dest_path))?;
            }
        }
    }

    Ok(())
}

fn sync_skills(source: &PathBuf, dest: &PathBuf) -> Result<()> {
    // Handle both directory-based skills (skills/*/SKILL.md) and flat skills (skills/*.md)

    // Directory-based skills
    let entries = fs::read_dir(source)
        .context(format!("Failed to read skills directory: {:?}", source))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let skill_md = path.join("SKILL.md");
            if skill_md.exists() {
                let skill_name = path.file_name()
                    .and_then(|s| s.to_str())
                    .context("Invalid skill directory name")?;

                let dest_skill_dir = dest.join(skill_name);

                // Only copy if source is newer or dest doesn't exist
                let should_copy = if dest_skill_dir.exists() {
                    let source_modified = fs::metadata(&skill_md)?.modified()?;
                    let dest_skill_md = dest_skill_dir.join("SKILL.md");
                    if dest_skill_md.exists() {
                        let dest_modified = fs::metadata(&dest_skill_md)?.modified()?;
                        source_modified > dest_modified
                    } else {
                        true
                    }
                } else {
                    true
                };

                if should_copy {
                    // Remove old directory if it exists
                    if dest_skill_dir.exists() {
                        fs::remove_dir_all(&dest_skill_dir)?;
                    }
                    // Copy entire skill directory
                    copy_dir_recursive(&path, &dest_skill_dir)?;
                }
            }
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Flat .md skills (backwards compatibility)
            let filename = path.file_name()
                .and_then(|s| s.to_str())
                .context("Invalid skill filename")?;

            let dest_path = dest.join(filename);

            let should_copy = if dest_path.exists() {
                let source_modified = fs::metadata(&path)?.modified()?;
                let dest_modified = fs::metadata(&dest_path)?.modified()?;
                source_modified > dest_modified
            } else {
                true
            };

            if should_copy {
                fs::copy(&path, &dest_path)?;
            }
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
