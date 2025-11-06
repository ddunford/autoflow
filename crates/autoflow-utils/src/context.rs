// Agent context building utilities

use autoflow_data::Sprint;

/// Builder for agent execution context
pub struct AgentContextBuilder {
    sections: Vec<(String, String)>,
}

impl AgentContextBuilder {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// Add a section with title and content
    pub fn section(mut self, title: &str, content: &str) -> Self {
        self.sections.push((title.to_string(), content.to_string()));
        self
    }

    /// Add sprint information
    pub fn sprint(mut self, sprint: &Sprint) -> Self {
        let content = format!(
            r#"Sprint #{}: {}

Status: {:?}
Total Effort: {}
Max Effort: {}

Deliverables:
{}

Tasks:
{}"#,
            sprint.id,
            sprint.goal,
            sprint.status,
            sprint.total_effort,
            sprint.max_effort,
            sprint
                .deliverables
                .iter()
                .map(|d| format!("- {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            sprint
                .tasks
                .iter()
                .map(|t| format!("- {} ({})", t.title, t.effort))
                .collect::<Vec<_>>()
                .join("\n"),
        );

        self.sections.push(("Sprint Details".to_string(), content));
        self
    }

    /// Add file content
    pub fn file_content(self, file_path: &str, content: &str) -> Self {
        self.section(&format!("File: {}", file_path), content)
    }

    /// Add instruction
    pub fn instruction(self, instruction: &str) -> Self {
        self.section("Instructions", instruction)
    }

    /// Build the final context string
    pub fn build(self) -> String {
        self.sections
            .into_iter()
            .map(|(title, content)| format!("# {}\n\n{}", title, content))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl Default for AgentContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick helper to build sprint context
pub fn build_sprint_context(sprint: &Sprint, additional_instructions: Option<&str>) -> String {
    let mut builder = AgentContextBuilder::new().sprint(sprint);

    if let Some(instructions) = additional_instructions {
        builder = builder.instruction(instructions);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use autoflow_data::{Sprint, SprintStatus};
    use chrono::Utc;

    #[test]
    fn test_context_builder() {
        let context = AgentContextBuilder::new()
            .section("Overview", "This is a test")
            .section("Details", "More information here")
            .build();

        assert!(context.contains("# Overview"));
        assert!(context.contains("This is a test"));
        assert!(context.contains("# Details"));
    }

    #[test]
    fn test_sprint_context() {
        let sprint = Sprint {
            id: 1,
            goal: "Test sprint".to_string(),
            status: SprintStatus::Pending,
            duration: None,
            total_effort: "5 hours".to_string(),
            max_effort: "8 hours".to_string(),
            started: None,
            last_updated: Utc::now(),
            completed_at: None,
            deliverables: vec!["Deliverable 1".to_string()],
            tasks: vec![],
            dependencies: vec![],
            integration_points: None,
            blocked_count: None,
            must_complete_first: false,
        };

        let context = build_sprint_context(&sprint, Some("Please implement this"));

        assert!(context.contains("Sprint #1"));
        assert!(context.contains("Test sprint"));
        assert!(context.contains("Deliverable 1"));
        assert!(context.contains("Please implement this"));
    }
}
