use autoflow_data::{AutoFlowError, Result};
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Validation result with detailed errors
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, path: String, message: String) {
        self.valid = false;
        self.errors.push(ValidationError { path, message });
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Schema validator for YAML files
pub struct SchemaValidator {
    schema: JSONSchema,
}

impl SchemaValidator {
    /// Load schema from JSON file
    pub fn from_file<P: AsRef<Path>>(schema_path: P) -> Result<Self> {
        let schema_content = fs::read_to_string(schema_path)
            .map_err(|e| AutoFlowError::ValidationError(format!("Failed to read schema: {}", e)))?;

        let schema_json: Value = serde_json::from_str(&schema_content)
            .map_err(|e| AutoFlowError::ValidationError(format!("Invalid schema JSON: {}", e)))?;

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema_json)
            .map_err(|e| AutoFlowError::ValidationError(format!("Failed to compile schema: {}", e)))?;

        Ok(Self { schema: compiled })
    }

    /// Load default SPRINTS.yml schema
    pub fn sprints_schema() -> Result<Self> {
        // Try to load from ~/.autoflow/schemas/ first, then from crate
        let home_schema = dirs::home_dir()
            .map(|h| h.join(".autoflow/schemas/sprints.schema.json"));

        if let Some(path) = home_schema {
            if path.exists() {
                return Self::from_file(path);
            }
        }

        // Fallback to embedded schema
        let schema_json: Value = serde_json::from_str(include_str!("../../../schemas/sprints.schema.json"))
            .map_err(|e| AutoFlowError::ValidationError(format!("Invalid embedded schema: {}", e)))?;

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema_json)
            .map_err(|e| AutoFlowError::ValidationError(format!("Failed to compile schema: {}", e)))?;

        Ok(Self { schema: compiled })
    }

    /// Validate a YAML file against the schema
    pub fn validate_yaml<P: AsRef<Path>>(&self, yaml_path: P) -> Result<ValidationResult> {
        let yaml_content = fs::read_to_string(&yaml_path)
            .map_err(|e| AutoFlowError::ValidationError(format!("Failed to read YAML: {}", e)))?;

        // Parse YAML to JSON value for validation
        let yaml_value: Value = serde_yaml::from_str(&yaml_content)
            .map_err(|e| AutoFlowError::ValidationError(format!("Invalid YAML: {}", e)))?;

        self.validate_value(&yaml_value)
    }

    /// Validate a JSON value against the schema
    pub fn validate_value(&self, value: &Value) -> Result<ValidationResult> {
        let mut result = ValidationResult::new();

        if let Err(errors) = self.schema.validate(value) {
            for error in errors {
                let path = error.instance_path.to_string();
                let message = error.to_string();
                result.add_error(path, message);
            }
        }

        Ok(result)
    }

    /// Validate SPRINTS.yml and return detailed results
    pub fn validate_sprints(yaml_path: &str) -> Result<ValidationResult> {
        let validator = Self::sprints_schema()?;
        validator.validate_yaml(yaml_path)
    }
}

/// Auto-fix common issues in SPRINTS.yml
pub struct SchemaFixer;

impl SchemaFixer {
    /// Attempt to fix common schema violations
    pub fn auto_fix(yaml_path: &str) -> Result<bool> {
        let content = fs::read_to_string(yaml_path)?;

        let mut fixed_content = content.clone();
        let mut changed = false;

        // Fix common issues:
        // 1. Remove markdown code blocks that agents sometimes add
        if fixed_content.contains("```yaml") || fixed_content.contains("```") {
            fixed_content = Self::remove_markdown_blocks(&fixed_content);
            changed = true;
        }

        // 2. Fix incorrect status values (e.g., "Done" -> "DONE")
        fixed_content = Self::normalize_status_values(&fixed_content);

        // 3. Ensure effort values have 'h' suffix
        // This is harder to do with regex, skip for now

        if changed {
            fs::write(yaml_path, fixed_content)?;
            tracing::info!("Auto-fixed schema violations in {}", yaml_path);
        }

        Ok(changed)
    }

    fn remove_markdown_blocks(content: &str) -> String {
        let mut result = String::new();
        let mut in_code_block = false;
        let mut skip_next_newline = false;

        for line in content.lines() {
            if line.trim().starts_with("```") {
                if !in_code_block {
                    // Entering code block
                    in_code_block = true;
                    skip_next_newline = true;
                } else {
                    // Exiting code block
                    in_code_block = false;
                }
                continue;
            }

            if in_code_block {
                // We're inside a code block, keep the content
                if skip_next_newline && line.is_empty() {
                    skip_next_newline = false;
                    continue;
                }
                result.push_str(line);
                result.push('\n');
            } else {
                // Outside code block, keep only if not empty
                if !line.is_empty() {
                    result.push_str(line);
                    result.push('\n');
                }
            }
        }

        result
    }

    fn normalize_status_values(content: &str) -> String {
        content
            .replace("status: Done", "status: DONE")
            .replace("status: Pending", "status: PENDING")
            .replace("status: Blocked", "status: BLOCKED")
            .replace("status: Complete", "status: COMPLETE")
            .replace("status: WriteCode", "status: WRITE_CODE")
            .replace("status: WriteUnitTests", "status: WRITE_UNIT_TESTS")
            .replace("status: WriteE2eTests", "status: WRITE_E2E_TESTS")
            .replace("status: CodeReview", "status: CODE_REVIEW")
            .replace("status: ReviewFix", "status: REVIEW_FIX")
            .replace("status: RunUnitTests", "status: RUN_UNIT_TESTS")
            .replace("status: UnitFix", "status: UNIT_FIX")
            .replace("status: RunE2eTests", "status: RUN_E2E_TESTS")
            .replace("status: E2eFix", "status: E2E_FIX")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_markdown_blocks() {
        let input = r#"```yaml
project:
  name: Test
```
Some other content
"#;
        let expected = "project:\n  name: Test\nSome other content\n";
        assert_eq!(SchemaFixer::remove_markdown_blocks(input), expected);
    }

    #[test]
    fn test_normalize_status_values() {
        let input = "status: Done\nstatus: Pending\nstatus: WriteCode";
        let expected = "status: DONE\nstatus: PENDING\nstatus: WRITE_CODE";
        assert_eq!(SchemaFixer::normalize_status_values(input), expected);
    }
}
