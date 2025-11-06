use autoflow_data::{SprintStatus, WorkflowType};
use std::collections::HashMap;

/// Defines a phase in a workflow
#[derive(Debug, Clone)]
pub struct WorkflowPhase {
    /// The sprint status for this phase
    pub status: SprintStatus,

    /// The agent to execute for this phase
    pub agent: &'static str,

    /// Maximum turns for the agent in this phase
    pub max_turns: u32,

    /// The status to transition to if this phase fails (for fix phases)
    pub fix_status: Option<SprintStatus>,

    /// Maximum number of retries before marking as blocked
    pub max_retries: u32,

    /// Whether this phase requires validation (e.g., tests must pass)
    pub requires_validation: bool,
}

/// Defines a complete workflow for a specific workflow type
#[derive(Debug, Clone)]
pub struct WorkflowDefinition {
    /// The workflow type this definition is for
    pub workflow_type: WorkflowType,

    /// The sequence of phases in this workflow
    pub phases: Vec<WorkflowPhase>,
}

impl WorkflowDefinition {
    /// Get the phase definition for a given status
    pub fn get_phase(&self, status: SprintStatus) -> Option<&WorkflowPhase> {
        self.phases.iter().find(|p| p.status == status)
    }

    /// Get the next phase after the current status
    pub fn next_phase(&self, current: SprintStatus) -> Option<&WorkflowPhase> {
        let current_idx = self.phases.iter().position(|p| p.status == current)?;
        self.phases.get(current_idx + 1)
    }

    /// Get the fix phase for a given status (if one exists)
    pub fn get_fix_phase(&self, status: SprintStatus) -> Option<&WorkflowPhase> {
        let phase = self.get_phase(status)?;
        let fix_status = phase.fix_status?;
        self.get_phase(fix_status)
    }
}

/// Get the workflow definition for a given workflow type
pub fn get_workflow_definition(workflow_type: WorkflowType) -> WorkflowDefinition {
    match workflow_type {
        WorkflowType::Implementation => implementation_workflow(),
        WorkflowType::Documentation => documentation_workflow(),
        WorkflowType::Test => test_workflow(),
        WorkflowType::Infrastructure => infrastructure_workflow(),
        WorkflowType::Refactor => refactor_workflow(),
    }
}

/// Full TDD implementation workflow
fn implementation_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        workflow_type: WorkflowType::Implementation,
        phases: vec![
            WorkflowPhase {
                status: SprintStatus::Pending,
                agent: "none", // Auto-advance
                max_turns: 0,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteUnitTests,
                agent: "test-writer",
                max_turns: 6,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteCode,
                agent: "code-implementer",
                max_turns: 10,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::CodeReview,
                agent: "reviewer",
                max_turns: 5,
                fix_status: Some(SprintStatus::ReviewFix),
                max_retries: 5,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::ReviewFix,
                agent: "review-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 5,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::RunUnitTests,
                agent: "unit-test-runner",
                max_turns: 5,
                fix_status: Some(SprintStatus::UnitFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::UnitFix,
                agent: "unit-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteE2eTests,
                agent: "e2e-writer",
                max_turns: 6,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::RunE2eTests,
                agent: "e2e-test-runner",
                max_turns: 5,
                fix_status: Some(SprintStatus::E2eFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::E2eFix,
                agent: "e2e-fixer",
                max_turns: 10,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Complete,
                agent: "health-check",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Done,
                agent: "none",
                max_turns: 0,
                fix_status: None,
                max_retries: 0,
                requires_validation: false,
            },
        ],
    }
}

/// Simplified documentation workflow
fn documentation_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        workflow_type: WorkflowType::Documentation,
        phases: vec![
            WorkflowPhase {
                status: SprintStatus::Pending,
                agent: "none", // Auto-advance
                max_turns: 0,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteCode, // Reuse WriteCode status for writing docs
                agent: "doc-writer",
                max_turns: 8,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::CodeReview, // Reuse CodeReview for doc review
                agent: "doc-reviewer",
                max_turns: 5,
                fix_status: Some(SprintStatus::ReviewFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::ReviewFix,
                agent: "doc-fixer",
                max_turns: 6,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Complete,
                agent: "health-check",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Done,
                agent: "none",
                max_turns: 0,
                fix_status: None,
                max_retries: 0,
                requires_validation: false,
            },
        ],
    }
}

/// Test-specific workflow
fn test_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        workflow_type: WorkflowType::Test,
        phases: vec![
            WorkflowPhase {
                status: SprintStatus::Pending,
                agent: "none", // Auto-advance
                max_turns: 0,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteCode, // Reuse WriteCode for writing tests
                agent: "test-implementer",
                max_turns: 8,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::CodeReview,
                agent: "reviewer",
                max_turns: 5,
                fix_status: Some(SprintStatus::ReviewFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::ReviewFix,
                agent: "review-fixer",
                max_turns: 6,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::RunUnitTests, // Run the tests we just wrote
                agent: "unit-test-runner",
                max_turns: 5,
                fix_status: Some(SprintStatus::UnitFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::UnitFix,
                agent: "unit-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Complete,
                agent: "health-check",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Done,
                agent: "none",
                max_turns: 0,
                fix_status: None,
                max_retries: 0,
                requires_validation: false,
            },
        ],
    }
}

/// Infrastructure workflow with integration tests
fn infrastructure_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        workflow_type: WorkflowType::Infrastructure,
        phases: vec![
            WorkflowPhase {
                status: SprintStatus::Pending,
                agent: "none", // Auto-advance
                max_turns: 0,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteCode,
                agent: "infra-implementer",
                max_turns: 10,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::CodeReview,
                agent: "reviewer",
                max_turns: 5,
                fix_status: Some(SprintStatus::ReviewFix),
                max_retries: 5,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::ReviewFix,
                agent: "review-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 5,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteE2eTests, // Use E2E for integration tests
                agent: "integration-test-writer",
                max_turns: 6,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::RunE2eTests, // Run integration tests
                agent: "integration-test-runner",
                max_turns: 5,
                fix_status: Some(SprintStatus::E2eFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::E2eFix,
                agent: "integration-fixer",
                max_turns: 10,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Complete,
                agent: "health-check",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Done,
                agent: "none",
                max_turns: 0,
                fix_status: None,
                max_retries: 0,
                requires_validation: false,
            },
        ],
    }
}

/// Refactoring workflow - verify tests exist, refactor, re-run tests
fn refactor_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        workflow_type: WorkflowType::Refactor,
        phases: vec![
            WorkflowPhase {
                status: SprintStatus::Pending,
                agent: "none", // Auto-advance
                max_turns: 0,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteUnitTests, // Verify tests exist first
                agent: "test-verifier",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::WriteCode, // Perform refactoring
                agent: "refactor-implementer",
                max_turns: 10,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::CodeReview,
                agent: "reviewer",
                max_turns: 5,
                fix_status: Some(SprintStatus::ReviewFix),
                max_retries: 5,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::ReviewFix,
                agent: "review-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 5,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::RunUnitTests, // Verify tests still pass
                agent: "unit-test-runner",
                max_turns: 5,
                fix_status: Some(SprintStatus::UnitFix),
                max_retries: 3,
                requires_validation: true,
            },
            WorkflowPhase {
                status: SprintStatus::UnitFix,
                agent: "unit-fixer",
                max_turns: 8,
                fix_status: None,
                max_retries: 3,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Complete,
                agent: "health-check",
                max_turns: 5,
                fix_status: None,
                max_retries: 1,
                requires_validation: false,
            },
            WorkflowPhase {
                status: SprintStatus::Done,
                agent: "none",
                max_turns: 0,
                fix_status: None,
                max_retries: 0,
                requires_validation: false,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implementation_workflow() {
        let workflow = get_workflow_definition(WorkflowType::Implementation);
        assert_eq!(workflow.workflow_type, WorkflowType::Implementation);
        assert!(workflow.phases.len() > 5);

        // Test phase lookup
        let write_code = workflow.get_phase(SprintStatus::WriteCode);
        assert!(write_code.is_some());
        assert_eq!(write_code.unwrap().agent, "code-implementer");
    }

    #[test]
    fn test_documentation_workflow() {
        let workflow = get_workflow_definition(WorkflowType::Documentation);
        assert_eq!(workflow.workflow_type, WorkflowType::Documentation);

        // Documentation should skip unit tests
        let write_code = workflow.get_phase(SprintStatus::WriteCode);
        assert!(write_code.is_some());
        assert_eq!(write_code.unwrap().agent, "doc-writer");

        // Should not have unit test phases
        let unit_tests = workflow.get_phase(SprintStatus::RunUnitTests);
        assert!(unit_tests.is_none());
    }

    #[test]
    fn test_next_phase() {
        let workflow = get_workflow_definition(WorkflowType::Implementation);

        let next = workflow.next_phase(SprintStatus::WriteCode);
        assert!(next.is_some());
        assert_eq!(next.unwrap().status, SprintStatus::CodeReview);
    }

    #[test]
    fn test_fix_phase() {
        let workflow = get_workflow_definition(WorkflowType::Implementation);

        let fix = workflow.get_fix_phase(SprintStatus::CodeReview);
        assert!(fix.is_some());
        assert_eq!(fix.unwrap().status, SprintStatus::ReviewFix);
    }
}
