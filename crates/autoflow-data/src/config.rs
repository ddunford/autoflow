use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub autoflow: AutoFlowConfig,
    pub paths: PathsConfig,
    pub defaults: DefaultsConfig,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| crate::AutoFlowError::ValidationError(e.to_string()))?;
        Ok(config)
    }

    pub fn global() -> Result<Self> {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let config_path = PathBuf::from(home).join(".autoflow/config.toml");
        Self::load(config_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoFlowConfig {
    pub version: String,
    pub install_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub agents_dir: PathBuf,
    pub skills_dir: PathBuf,
    pub reference_dir: PathBuf,
    pub schemas_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub model: String,
    pub max_iterations: u32,
    pub parallel_sprints: bool,
    pub auto_commit: bool,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_iterations: 50,
            parallel_sprints: false,
            auto_commit: true,
        }
    }
}
