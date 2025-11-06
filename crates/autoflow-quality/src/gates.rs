use super::pipeline::{GateContext, GateResult, QualityGate};
use super::schema_validator::{SchemaFixer, SchemaValidator};
use autoflow_data::Result;
use std::fs;

/// Schema validation gate
pub struct SchemaValidationGate;

impl QualityGate for SchemaValidationGate {
    fn name(&self) -> &str {
        "Schema Validation"
    }

    fn run(&self, context: &GateContext) -> Result<GateResult> {
        // Try auto-fix BEFORE validation if enabled
        if context.auto_fix {
            if SchemaFixer::auto_fix(&context.sprints_path)? {
                tracing::info!("Applied auto-fixes to SPRINTS.yml");
            }
        }

        let validation = SchemaValidator::validate_sprints(&context.sprints_path)?;

        if validation.is_valid() {
            let msg = if context.auto_fix {
                "SPRINTS.yml conforms to schema (after auto-fix)".to_string()
            } else {
                "SPRINTS.yml conforms to schema".to_string()
            };
            Ok(GateResult::pass(self.name().to_string()).with_message(msg))
        } else {
            let errors: Vec<String> = validation
                .errors
                .iter()
                .map(|e| format!("{}: {}", e.path, e.message))
                .collect();

            Ok(GateResult::fail(self.name().to_string(), errors))
        }
    }

    fn is_critical(&self) -> bool {
        true
    }
}

/// Output format validation gate - detects markdown in YAML
pub struct OutputFormatGate;

impl QualityGate for OutputFormatGate {
    fn name(&self) -> &str {
        "Output Format Validation"
    }

    fn run(&self, context: &GateContext) -> Result<GateResult> {
        let content = fs::read_to_string(&context.sprints_path)?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check for markdown code blocks
        if content.contains("```yaml") || content.contains("```yml") {
            errors.push("Detected markdown code blocks in YAML file".to_string());
        }

        // Check for common agent mistakes
        if content.contains("status: Done") && !content.contains("status: DONE") {
            warnings.push("Status should be SCREAMING_SNAKE_CASE (DONE not Done)".to_string());
        }

        // Check for incorrect field names
        if content.contains("sprint_id:") {
            errors.push("Should use 'id:' not 'sprint_id:'".to_string());
        }

        if !errors.is_empty() {
            // Try auto-fix if enabled
            if context.auto_fix {
                if SchemaFixer::auto_fix(&context.sprints_path)? {
                    return Ok(GateResult::pass(self.name().to_string())
                        .with_message("Output format issues auto-fixed".to_string())
                        .with_fixed());
                }
            }

            Ok(GateResult::fail(self.name().to_string(), errors))
        } else if !warnings.is_empty() {
            Ok(GateResult::pass(self.name().to_string())
                .with_warning(warnings.join("; ")))
        } else {
            Ok(GateResult::pass(self.name().to_string())
                .with_message("Output format is clean".to_string()))
        }
    }

    fn is_critical(&self) -> bool {
        true
    }
}

/// Blocker detection gate - finds missing dependencies, APIs, etc.
pub struct BlockerDetectionGate;

impl QualityGate for BlockerDetectionGate {
    fn name(&self) -> &str {
        "Blocker Detection"
    }

    fn run(&self, context: &GateContext) -> Result<GateResult> {
        use autoflow_data::SprintsYaml;

        let sprints = SprintsYaml::load(&context.sprints_path)?;

        let mut warnings = Vec::new();

        // Check for missing dependencies
        for sprint in &sprints.sprints {
            if sprint.status == autoflow_data::SprintStatus::Blocked {
                warnings.push(format!(
                    "Sprint {} is BLOCKED - requires investigation",
                    sprint.id
                ));
            }

            // Check for unresolved dependencies
            for dep in &sprint.dependencies {
                if !sprints.sprints.iter().any(|s| s.id.to_string() == *dep) {
                    warnings.push(format!(
                        "Sprint {} has unknown dependency: {}",
                        sprint.id, dep
                    ));
                }
            }
        }

        if warnings.is_empty() {
            Ok(GateResult::pass(self.name().to_string())
                .with_message("No blockers detected".to_string()))
        } else {
            Ok(GateResult::pass(self.name().to_string())
                .with_warning(warnings.join("\n")))
        }
    }

    fn is_critical(&self) -> bool {
        false
    }
}

/// Code quality validation gate - basic checks
pub struct CodeQualityGate;

impl QualityGate for CodeQualityGate {
    fn name(&self) -> &str {
        "Code Quality Checks"
    }

    fn run(&self, context: &GateContext) -> Result<GateResult> {
        use autoflow_data::SprintsYaml;

        let sprints = SprintsYaml::load(&context.sprints_path)?;

        let mut warnings = Vec::new();

        // Check for sprints with no tasks
        for sprint in &sprints.sprints {
            if sprint.tasks.is_empty() {
                warnings.push(format!("Sprint {} has no tasks defined", sprint.id));
            }

            // Check for tasks with no business rules
            for task in &sprint.tasks {
                if task.business_rules.is_empty() {
                    warnings.push(format!(
                        "Task {} has no business rules defined",
                        task.id
                    ));
                }
            }
        }

        if warnings.is_empty() {
            Ok(GateResult::pass(self.name().to_string())
                .with_message("Code quality checks passed".to_string()))
        } else {
            Ok(GateResult::pass(self.name().to_string())
                .with_warning(warnings.join("\n")))
        }
    }

    fn is_critical(&self) -> bool {
        false
    }
}

/// Create default quality pipeline
pub fn create_default_pipeline() -> super::pipeline::QualityPipeline {
    super::pipeline::QualityPipeline::new()
        .add_gate(SchemaValidationGate)
        .add_gate(OutputFormatGate)
        .add_gate(BlockerDetectionGate)
        .add_gate(CodeQualityGate)
        .stop_on_failure(true)
}
