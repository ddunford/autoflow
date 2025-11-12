/// Embedded assets for distribution
/// This ensures agents, skills, schemas, and templates are available even when
/// installed via `cargo install` or binary releases without a git repository.
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

// Embed all agent files
macro_rules! embed_agents {
    ($($name:literal),* $(,)?) => {
        &[
            $(
                ($name, include_str!(concat!("../../../agents/", $name, ".md"))),
            )*
        ]
    };
}

const EMBEDDED_AGENTS: &[(&str, &str)] = embed_agents![
    "blocker-resolver",
    "code-implementer",
    "debug-blocker",
    "e2e-fixer",
    "e2e-test-runner",
    "e2e-writer",
    "environment-setup",
    "health-check",
    "infra-implementer",
    "integration-fixer",
    "integration-test-runner",
    "integration-test-writer",
    "make-docs",
    "make-docs-backend",
    "make-docs-foundation",
    "make-docs-frontend",
    "make-docs-ops",
    "make-docs-quality",
    "make-sprints",
    "review-fixer",
    "reviewer",
    "test-writer",
    "unit-fixer",
    "unit-test-runner",
];

// Embed skill directories
// Each skill is a directory with SKILL.md and potentially other files
macro_rules! embed_skills {
    ($($name:literal),* $(,)?) => {
        &[
            $(
                ($name, include_str!(concat!("../../../skills/", $name, "/SKILL.md"))),
            )*
        ]
    };
}

const EMBEDDED_SKILLS: &[(&str, &str)] = embed_skills![
    "backward-compatibility",
    "docker-optimization",
    "jest-to-vitest",
    "keycloak-setup",
    "laravel-cache-configuration",
    "laravel-session-management",
    "laravel-test-environment",
    "nextjs-app-router",
    "owasp-security-audit",
    "react-performance",
    "test-data-builder",
];

// Embed templates
const EMBEDDED_TEMPLATE_SPRINTS: &str = include_str!("../../../templates/SPRINTS.template.yml");
const EMBEDDED_TEMPLATE_CLAUDE: &str = include_str!("../../../templates/CLAUDE.template.md");

/// Extract all embedded assets to their target directories
/// This is called on first run or when assets are missing/outdated
pub fn extract_embedded_assets() -> Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());

    extract_agents(&home)?;
    extract_skills(&home)?;
    extract_templates(&home)?;

    Ok(())
}

/// Extract agents to ~/.claude/agents/
fn extract_agents(home: &str) -> Result<()> {
    let agents_dir = PathBuf::from(home).join(".claude/agents");
    fs::create_dir_all(&agents_dir)
        .context("Failed to create agents directory")?;

    for (name, content) in EMBEDDED_AGENTS {
        let dest = agents_dir.join(format!("{}.agent.md", name));

        // Only write if file doesn't exist or is older than build time
        let should_write = !dest.exists();

        if should_write {
            fs::write(&dest, content)
                .context(format!("Failed to write agent: {}", name))?;
        }
    }

    Ok(())
}

/// Extract skills to ~/.claude/skills/
fn extract_skills(home: &str) -> Result<()> {
    let skills_dir = PathBuf::from(home).join(".claude/skills");
    fs::create_dir_all(&skills_dir)
        .context("Failed to create skills directory")?;

    for (name, content) in EMBEDDED_SKILLS {
        let skill_dir = skills_dir.join(name);
        fs::create_dir_all(&skill_dir)
            .context(format!("Failed to create skill directory: {}", name))?;

        let dest = skill_dir.join("SKILL.md");

        // Only write if file doesn't exist
        if !dest.exists() {
            fs::write(&dest, content)
                .context(format!("Failed to write skill: {}", name))?;
        }
    }

    Ok(())
}

/// Extract templates to ~/.autoflow/templates/
fn extract_templates(home: &str) -> Result<()> {
    let templates_dir = PathBuf::from(home).join(".autoflow/templates");
    fs::create_dir_all(&templates_dir)
        .context("Failed to create templates directory")?;

    let sprints_template = templates_dir.join("SPRINTS.template.yml");
    if !sprints_template.exists() {
        fs::write(&sprints_template, EMBEDDED_TEMPLATE_SPRINTS)
            .context("Failed to write SPRINTS.template.yml")?;
    }

    let claude_template = templates_dir.join("CLAUDE.template.md");
    if !claude_template.exists() {
        fs::write(&claude_template, EMBEDDED_TEMPLATE_CLAUDE)
            .context("Failed to write CLAUDE.template.md")?;
    }

    Ok(())
}

/// Check if embedded assets need to be extracted
/// Returns true if any required asset is missing
pub fn needs_extraction() -> bool {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return true, // Can't check, so extract to be safe
    };

    // Check if at least one agent exists
    let agents_dir = PathBuf::from(&home).join(".claude/agents");
    if !agents_dir.exists() {
        return true;
    }

    // Check if we have any agents at all
    let has_agents = EMBEDDED_AGENTS.iter().any(|(name, _)| {
        agents_dir.join(format!("{}.agent.md", name)).exists()
    });

    if !has_agents {
        return true;
    }

    // Check if at least one skill exists
    let skills_dir = PathBuf::from(&home).join(".claude/skills");
    if !skills_dir.exists() {
        return true;
    }

    // Check templates
    let templates_dir = PathBuf::from(&home).join(".autoflow/templates");
    if !templates_dir.exists()
        || !templates_dir.join("SPRINTS.template.yml").exists()
        || !templates_dir.join("CLAUDE.template.md").exists() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_agents_count() {
        assert!(EMBEDDED_AGENTS.len() >= 13, "Should have at least 13 agents embedded");
    }

    #[test]
    fn test_embedded_skills_count() {
        assert!(EMBEDDED_SKILLS.len() >= 10, "Should have at least 10 skills embedded");
    }

    #[test]
    fn test_embedded_templates_not_empty() {
        assert!(!EMBEDDED_TEMPLATE_SPRINTS.is_empty(), "SPRINTS template should not be empty");
        assert!(!EMBEDDED_TEMPLATE_CLAUDE.is_empty(), "CLAUDE template should not be empty");
    }
}
