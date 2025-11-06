use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::tasks::Task;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintsYaml {
    pub project: ProjectMetadata,
    pub sprints: Vec<Sprint>,
}

impl SprintsYaml {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let sprints = serde_yaml::from_str(&content)?;
        Ok(sprints)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Validate and fix YAML content by adding missing required fields
    pub fn validate_and_fix(yaml_content: &str) -> Result<Self> {
        // First try to parse as-is
        match serde_yaml::from_str::<Self>(yaml_content) {
            Ok(sprints) => Ok(sprints),
            Err(_) => {
                // If parsing fails, try to add missing fields
                let mut value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
                let now = Utc::now().to_rfc3339();

                // Fix project metadata
                if let Some(project) = value.get_mut("project") {
                    if let Some(project_map) = project.as_mapping_mut() {
                        // Add last_updated if missing (required)
                        if !project_map.contains_key(&serde_yaml::Value::String("last_updated".to_string())) {
                            project_map.insert(
                                serde_yaml::Value::String("last_updated".to_string()),
                                serde_yaml::Value::String(now.clone()),
                            );
                        }
                        // Add current_sprint if missing (optional, but explicit)
                        if !project_map.contains_key(&serde_yaml::Value::String("current_sprint".to_string())) {
                            project_map.insert(
                                serde_yaml::Value::String("current_sprint".to_string()),
                                serde_yaml::Value::Null,
                            );
                        }
                        // Add version if missing (for backwards compatibility)
                        if !project_map.contains_key(&serde_yaml::Value::String("version".to_string())) {
                            project_map.insert(
                                serde_yaml::Value::String("version".to_string()),
                                serde_yaml::Value::String("0.1.0".to_string()),
                            );
                        }
                        // Add description if missing (for backwards compatibility)
                        if !project_map.contains_key(&serde_yaml::Value::String("description".to_string())) {
                            project_map.insert(
                                serde_yaml::Value::String("description".to_string()),
                                serde_yaml::Value::String("AutoFlow Project".to_string()),
                            );
                        }
                    }
                }

                // Fix sprints
                if let Some(sprints) = value.get_mut("sprints") {
                    if let Some(sprints_seq) = sprints.as_sequence_mut() {
                        for sprint in sprints_seq.iter_mut() {
                            if let Some(sprint_map) = sprint.as_mapping_mut() {
                                // Add last_updated if missing (required)
                                if !sprint_map.contains_key(&serde_yaml::Value::String("last_updated".to_string())) {
                                    sprint_map.insert(
                                        serde_yaml::Value::String("last_updated".to_string()),
                                        serde_yaml::Value::String(now.clone()),
                                    );
                                }
                                // Add started if missing (optional)
                                if !sprint_map.contains_key(&serde_yaml::Value::String("started".to_string())) {
                                    sprint_map.insert(
                                        serde_yaml::Value::String("started".to_string()),
                                        serde_yaml::Value::Null,
                                    );
                                }
                                // Add completed_at if missing (optional)
                                if !sprint_map.contains_key(&serde_yaml::Value::String("completed_at".to_string())) {
                                    sprint_map.insert(
                                        serde_yaml::Value::String("completed_at".to_string()),
                                        serde_yaml::Value::Null,
                                    );
                                }
                            }
                        }
                    }
                }

                // Try to parse again with fixed content
                serde_yaml::from_value(value)
                    .map_err(|e| crate::AutoFlowError::ValidationError(
                        format!("Failed to parse SPRINTS.yml even after fixing: {}", e)
                    ))
            }
        }
    }

    pub fn filter_by_status(&self, status: SprintStatus) -> Vec<&Sprint> {
        self.sprints
            .iter()
            .filter(|s| s.status == status)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "default_description")]
    pub description: String,
    pub total_sprints: u32,
    pub current_sprint: Option<u32>,
    pub last_updated: DateTime<Utc>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_description() -> String {
    "AutoFlow Project".to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowType {
    Implementation,
    Documentation,
    Test,
    Infrastructure,
    Refactor,
}

impl Default for WorkflowType {
    fn default() -> Self {
        WorkflowType::Implementation
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: u32,
    pub goal: String,
    pub status: SprintStatus,

    #[serde(default)]
    pub workflow_type: WorkflowType,

    pub duration: Option<String>,
    pub total_effort: String,
    pub max_effort: String,
    pub started: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deliverables: Vec<String>,
    pub tasks: Vec<Task>,

    #[serde(default)]
    pub dependencies: Vec<String>,

    #[serde(default)]
    pub integration_points: Option<IntegrationPoints>,

    #[serde(default)]
    pub blocked_count: Option<u32>,

    #[serde(default)]
    pub must_complete_first: bool,
}

impl Sprint {
    pub fn is_done(&self) -> bool {
        matches!(self.status, SprintStatus::Done)
    }

    pub fn advance(&mut self) -> Result<()> {
        if let Some(next_status) = self.status.next() {
            self.status = next_status;
            self.last_updated = Utc::now();
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SprintStatus {
    Pending,
    WriteUnitTests,
    WriteCode,
    CodeReview,
    ReviewFix,
    RunUnitTests,
    UnitFix,
    WriteE2eTests,
    RunE2eTests,
    E2eFix,
    Blocked,
    Complete,
    Done,
}

impl SprintStatus {
    /// Get next status in TDD pipeline
    pub fn next(&self) -> Option<SprintStatus> {
        match self {
            SprintStatus::Pending => Some(SprintStatus::WriteUnitTests),
            SprintStatus::WriteUnitTests => Some(SprintStatus::WriteCode),
            SprintStatus::WriteCode => Some(SprintStatus::CodeReview),
            SprintStatus::CodeReview => Some(SprintStatus::RunUnitTests),
            SprintStatus::ReviewFix => Some(SprintStatus::CodeReview),
            SprintStatus::RunUnitTests => Some(SprintStatus::WriteE2eTests),
            SprintStatus::UnitFix => Some(SprintStatus::RunUnitTests),
            SprintStatus::WriteE2eTests => Some(SprintStatus::RunE2eTests),
            SprintStatus::RunE2eTests => Some(SprintStatus::Complete),
            SprintStatus::E2eFix => Some(SprintStatus::RunE2eTests),
            SprintStatus::Complete => Some(SprintStatus::Done),
            SprintStatus::Done => None,
            SprintStatus::Blocked => None,
        }
    }

    /// Can this status be retried?
    pub fn is_retriable(&self) -> bool {
        matches!(
            self,
            SprintStatus::ReviewFix | SprintStatus::UnitFix | SprintStatus::E2eFix
        )
    }

    /// Maximum retry attempts before BLOCKED
    pub fn max_retries(&self) -> u32 {
        match self {
            SprintStatus::UnitFix | SprintStatus::E2eFix => 3,
            SprintStatus::ReviewFix => 5,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoints {
    /// Existing files that will be modified
    #[serde(default)]
    pub modifies: Vec<String>,

    /// New files that will be created
    #[serde(default)]
    pub creates: Vec<String>,

    /// Existing tests that need updates
    #[serde(default)]
    pub tests_existing: Vec<String>,

    /// Integration patterns to follow
    #[serde(default)]
    pub patterns: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprint_status_transitions() {
        assert_eq!(
            SprintStatus::Pending.next(),
            Some(SprintStatus::WriteUnitTests)
        );
        assert_eq!(
            SprintStatus::WriteUnitTests.next(),
            Some(SprintStatus::WriteCode)
        );
        assert_eq!(
            SprintStatus::WriteCode.next(),
            Some(SprintStatus::CodeReview)
        );
        assert_eq!(SprintStatus::Done.next(), None);
        assert_eq!(SprintStatus::Blocked.next(), None);
    }

    #[test]
    fn test_retriable_statuses() {
        assert!(SprintStatus::ReviewFix.is_retriable());
        assert!(SprintStatus::UnitFix.is_retriable());
        assert!(SprintStatus::E2eFix.is_retriable());
        assert!(!SprintStatus::Pending.is_retriable());
        assert!(!SprintStatus::Done.is_retriable());
    }

    #[test]
    fn test_max_retries() {
        assert_eq!(SprintStatus::UnitFix.max_retries(), 3);
        assert_eq!(SprintStatus::E2eFix.max_retries(), 3);
        assert_eq!(SprintStatus::ReviewFix.max_retries(), 5);
    }
}
