use thiserror::Error;

pub type Result<T> = std::result::Result<T, AutoFlowError>;

#[derive(Error, Debug)]
pub enum AutoFlowError {
    #[error("Failed to parse SPRINTS.yml: {0}")]
    SprintsParseError(#[from] serde_yaml::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Schema validation failed: {0}")]
    ValidationError(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("Agent execution failed: {0}")]
    AgentError(String),

    #[error("Agent '{0}' execution failed: {1}")]
    AgentExecutionFailed(String, String),

    #[error("Sprint {0} is blocked: {1}")]
    SprintBlocked(u32, String),

    #[error("Maximum iterations ({0}) reached")]
    MaxIterationsExceeded(u32),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Project not initialized. Run 'autoflow init' first.")]
    NotInitialized,

    #[error("Invalid dependency: {0}")]
    InvalidDependency(String),

    #[error("Missing dependency: sprint {sprint} depends on incomplete sprint {missing}")]
    MissingDependency { sprint: u32, missing: u32 },

    #[error("Bug not fixed: {bug_id} - {reason}")]
    BugNotFixed { bug_id: String, reason: String },

    #[error("Regression detected in {bug_id}: {failing_tests:?}")]
    RegressionDetected {
        bug_id: String,
        failing_tests: Vec<String>,
    },

    #[error("Infrastructure not ready: {issues:?}")]
    InfrastructureNotReady { issues: Vec<String> },

    #[error("Docker build failed")]
    DockerBuildFailed,

    #[error("Docker start failed")]
    DockerStartFailed,

    #[error("Service start timeout")]
    ServiceStartTimeout,

    #[error("Database connection failed")]
    DatabaseConnectionFailed,

    #[error("Redis connection failed")]
    RedisConnectionFailed,

    #[error("App health check failed")]
    AppHealthCheckFailed,

    #[error("Migration failed")]
    MigrationFailed,

    #[error("Merge conflict in branch: {branch}")]
    MergeConflict { branch: String },

    #[error("No backups found")]
    NoBackupsFound,

    #[error("No Bash installation found")]
    NoBashInstallation,

    #[error("Invalid component: {0}")]
    InvalidComponent(String),
}
