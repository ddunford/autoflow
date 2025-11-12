use anyhow::{Context, Result};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub updated_agents: Vec<String>,
    pub new_agents: Vec<String>,
    pub updated_skills: Vec<String>,
    pub new_skills: Vec<String>,
}

impl UpdateInfo {
    pub fn has_updates(&self) -> bool {
        !self.updated_agents.is_empty()
            || !self.new_agents.is_empty()
            || !self.updated_skills.is_empty()
            || !self.new_skills.is_empty()
    }

    pub fn total_count(&self) -> usize {
        self.updated_agents.len()
            + self.new_agents.len()
            + self.updated_skills.len()
            + self.new_skills.len()
    }
}

/// Check if updates are available by comparing template files with installed files
pub fn check_for_updates() -> Result<UpdateInfo> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let template_agents_dir = home.join(".autoflow/agents");
    let template_skills_dir = home.join(".autoflow/skills");
    let installed_agents_dir = home.join(".claude/agents");
    let installed_skills_dir = home.join(".claude/skills");

    let mut info = UpdateInfo {
        updated_agents: Vec::new(),
        new_agents: Vec::new(),
        updated_skills: Vec::new(),
        new_skills: Vec::new(),
    };

    // Check agents
    if template_agents_dir.exists() && installed_agents_dir.exists() {
        let template_agents = list_markdown_files(&template_agents_dir)?;
        let installed_agents = list_agent_files(&installed_agents_dir)?;

        for (name, template_path) in &template_agents {
            let installed_name = format!("{}.agent", name);
            if let Some(installed_path) = installed_agents.get(&installed_name) {
                // Check if template is newer
                if is_newer(&template_path, installed_path)? {
                    info.updated_agents.push(installed_name.clone());
                }
            } else {
                // New agent
                info.new_agents.push(name.clone());
            }
        }
    }

    // Check skills
    if template_skills_dir.exists() && installed_skills_dir.exists() {
        let template_skills = list_skill_dirs(&template_skills_dir)?;
        let installed_skills = list_skill_dirs(&installed_skills_dir)?;

        for (name, template_path) in &template_skills {
            if let Some(installed_path) = installed_skills.get(name) {
                // Check if template is newer
                if is_skill_newer(&template_path, installed_path)? {
                    info.updated_skills.push(name.clone());
                }
            } else {
                // New skill
                info.new_skills.push(name.clone());
            }
        }
    }

    Ok(info)
}

/// Check if we should run update check based on last check time and config
pub fn should_check_for_updates() -> Result<bool> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let check_file = home.join(".autoflow/.last_update_check");

    // If file doesn't exist, we should check
    if !check_file.exists() {
        return Ok(true);
    }

    // Check if it's been more than 24 hours
    let metadata = fs::metadata(&check_file)?;
    let modified = metadata.modified()?;
    let elapsed = SystemTime::now().duration_since(modified)?;

    // Check once per day
    Ok(elapsed.as_secs() > 86400)
}

/// Update the last check timestamp
pub fn update_check_timestamp() -> Result<()> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let check_file = home.join(".autoflow/.last_update_check");

    fs::create_dir_all(check_file.parent().unwrap())?;
    fs::write(&check_file, "")?;

    Ok(())
}

/// Prompt user for updates and execute if approved
pub fn prompt_and_update(info: &UpdateInfo) -> Result<bool> {
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Updates Available".bright_yellow().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    if !info.updated_agents.is_empty() {
        println!("{}", "📦 Updated Agents:".bright_cyan());
        for agent in &info.updated_agents {
            println!("   {} {}", "↻".yellow(), agent);
        }
        println!();
    }

    if !info.new_agents.is_empty() {
        println!("{}", "✨ New Agents:".bright_cyan());
        for agent in &info.new_agents {
            println!("   {} {}", "+".green(), agent);
        }
        println!();
    }

    if !info.updated_skills.is_empty() {
        println!("{}", "📦 Updated Skills:".bright_cyan());
        for skill in &info.updated_skills {
            println!("   {} {}", "↻".yellow(), skill);
        }
        println!();
    }

    if !info.new_skills.is_empty() {
        println!("{}", "✨ New Skills:".bright_cyan());
        for skill in &info.new_skills {
            println!("   {} {}", "+".green(), skill);
        }
        println!();
    }

    print!("Update now? [Y/n/skip] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "" | "y" | "yes" => {
            println!();
            println!("{}", "🔄 Running update...".bright_cyan());
            run_update_script()?;
            update_check_timestamp()?;
            println!();
            println!("{}", "✅ Update complete!".bright_green());
            println!();
            Ok(true)
        }
        "skip" => {
            // Update timestamp so we don't check again for a while
            update_check_timestamp()?;
            println!();
            println!("{}", "⏭ Skipped update check for 24 hours".bright_yellow());
            println!();
            Ok(false)
        }
        _ => {
            println!();
            println!("{}", "❌ Update cancelled".bright_yellow());
            println!();
            Ok(false)
        }
    }
}

/// Run the update script
fn run_update_script() -> Result<()> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let update_script = home.join(".autoflow/update.sh");

    // Check if script exists
    if !update_script.exists() {
        // Try to find it in the autoflow repo
        if let Ok(current_dir) = std::env::current_dir() {
            let repo_script = current_dir.join("scripts/update.sh");
            if repo_script.exists() {
                let output = Command::new("bash")
                    .arg(&repo_script)
                    .arg("--force")
                    .output()?;

                if !output.status.success() {
                    anyhow::bail!("Update script failed: {}", String::from_utf8_lossy(&output.stderr));
                }
                return Ok(());
            }
        }

        anyhow::bail!("Update script not found. Please run the installer.");
    }

    let output = Command::new("bash")
        .arg(&update_script)
        .arg("--force")
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Update script failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

/// List all .md files in a directory (for template agents)
fn list_markdown_files(dir: &Path) -> Result<HashMap<String, PathBuf>> {
    let mut files = HashMap::new();

    if !dir.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "md") {
            if let Some(stem) = path.file_stem() {
                if let Some(name) = stem.to_str() {
                    files.insert(name.to_string(), path);
                }
            }
        }
    }

    Ok(files)
}

/// List all .agent.md files in a directory (for installed agents)
fn list_agent_files(dir: &Path) -> Result<HashMap<String, PathBuf>> {
    let mut files = HashMap::new();

    if !dir.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.ends_with(".agent.md") {
                    let name = file_name.trim_end_matches(".md").to_string();
                    files.insert(name, path);
                }
            }
        }
    }

    Ok(files)
}

/// List all skill directories
fn list_skill_dirs(dir: &Path) -> Result<HashMap<String, PathBuf>> {
    let mut skills = HashMap::new();

    if !dir.exists() {
        return Ok(skills);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                skills.insert(name.to_string(), path);
            }
        }
    }

    Ok(skills)
}

/// Check if file A is newer than file B based on modification time
fn is_newer(a: &Path, b: &Path) -> Result<bool> {
    let a_meta = fs::metadata(a)?;
    let b_meta = fs::metadata(b)?;

    let a_modified = a_meta.modified()?;
    let b_modified = b_meta.modified()?;

    Ok(a_modified > b_modified)
}

/// Check if a skill directory is newer by comparing SKILL.md files
fn is_skill_newer(template_dir: &Path, installed_dir: &Path) -> Result<bool> {
    let template_skill = template_dir.join("SKILL.md");
    let installed_skill = installed_dir.join("SKILL.md");

    if template_skill.exists() && installed_skill.exists() {
        return is_newer(&template_skill, &installed_skill);
    }

    Ok(false)
}
