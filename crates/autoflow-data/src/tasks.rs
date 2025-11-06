use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub effort: String,
    pub priority: Priority,
    pub feature: String,

    #[serde(default)]
    pub docs: Vec<String>,

    #[serde(default)]
    pub business_rules: Vec<String>,

    #[serde(default)]
    pub integration_notes: Option<String>,

    pub testing: TestingRequirements,

    #[serde(default)]
    pub status: TaskStatus,

    #[serde(default)]
    pub committed_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub reviewed_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub tested_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub done_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub git_commit: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Committed,
    Reviewed,
    Tested,
    Done,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingRequirements {
    #[serde(default)]
    pub unit_tests: Option<TestRequirement>,

    #[serde(default)]
    pub integration_tests: Option<TestRequirement>,

    #[serde(default)]
    pub e2e_tests: Option<TestRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRequirement {
    pub required: bool,
    pub reason: String,
}
