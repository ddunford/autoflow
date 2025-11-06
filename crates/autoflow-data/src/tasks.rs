use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(default = "generate_task_id")]
    pub id: String,
    pub title: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub r#type: Option<String>,

    #[serde(default)]
    pub doc_reference: Option<String>,

    #[serde(default)]
    pub acceptance_criteria: Vec<String>,

    #[serde(default)]
    pub test_specification: Option<String>,

    #[serde(default = "default_effort")]
    pub effort: String,

    #[serde(default = "default_priority")]
    pub priority: Priority,

    #[serde(default = "default_feature")]
    pub feature: String,

    #[serde(default)]
    pub docs: Vec<String>,

    #[serde(default)]
    pub business_rules: Vec<String>,

    #[serde(default)]
    pub integration_notes: Option<String>,

    #[serde(default = "default_testing")]
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

fn generate_task_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    format!("task-{}", COUNTER.fetch_add(1, Ordering::SeqCst))
}

fn default_effort() -> String {
    "4h".to_string()
}

fn default_priority() -> Priority {
    Priority::Medium
}

fn default_feature() -> String {
    "core".to_string()
}

fn default_testing() -> TestingRequirements {
    TestingRequirements {
        unit_tests: None,
        integration_tests: None,
        e2e_tests: None,
    }
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
