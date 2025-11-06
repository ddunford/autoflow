// Centralized path constants and utilities

use std::path::{Path, PathBuf};

/// AutoFlow directory structure constants
pub struct Paths;

impl Paths {
    /// Main .autoflow directory
    pub const AUTOFLOW_DIR: &'static str = ".autoflow";

    /// SPRINTS.yml path
    pub const SPRINTS_YML: &'static str = ".autoflow/SPRINTS.yml";

    /// CLAUDE.md path
    pub const CLAUDE_MD: &'static str = ".autoflow/CLAUDE.md";

    /// Settings path
    pub const SETTINGS_JSON: &'static str = ".autoflow/settings.json";

    /// Bugs directory
    pub const BUGS_DIR: &'static str = ".autoflow/bugs";

    /// Tasks directory
    pub const TASKS_DIR: &'static str = ".autoflow/tasks";

    /// Build spec path
    pub const BUILD_SPEC: &'static str = "BUILD_SPEC.md";

    /// Architecture spec path
    pub const ARCHITECTURE_MD: &'static str = "ARCHITECTURE.md";

    /// API spec path
    pub const API_SPEC: &'static str = "API_SPEC.md";

    /// UI spec path
    pub const UI_SPEC: &'static str = "UI_SPEC.md";

    /// Integration guide path
    pub const INTEGRATION_GUIDE: &'static str = "INTEGRATION_GUIDE.md";

    /// Claude CLI config paths
    pub const CLAUDE_SETTINGS_LOCAL: &'static str = ".claude/settings.local.json";
    pub const MCP_JSON: &'static str = ".mcp.json";

    /// Agent paths
    pub const CLAUDE_AGENTS_DIR: &'static str = ".claude/agents";
    pub const CLAUDE_SKILLS_DIR: &'static str = ".claude/skills";
}

/// Check if project is initialized
pub fn is_initialized() -> bool {
    Path::new(Paths::AUTOFLOW_DIR).exists()
}

/// Get home directory path
pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Get Claude agents directory (global)
pub fn claude_agents_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join(Paths::CLAUDE_AGENTS_DIR))
}

/// Get Claude skills directory (global)
pub fn claude_skills_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join(Paths::CLAUDE_SKILLS_DIR))
}

/// Get Claude config path (global user scope)
pub fn claude_settings_path() -> Option<PathBuf> {
    home_dir().map(|h| h.join(Paths::CLAUDE_SETTINGS_LOCAL))
}

/// Get project MCP config path (project scope)
pub fn project_mcp_config() -> PathBuf {
    PathBuf::from(Paths::MCP_JSON)
}

/// Ensure directory exists
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths_constants() {
        assert_eq!(Paths::AUTOFLOW_DIR, ".autoflow");
        assert_eq!(Paths::SPRINTS_YML, ".autoflow/SPRINTS.yml");
        assert_eq!(Paths::CLAUDE_MD, ".autoflow/CLAUDE.md");
    }
}
