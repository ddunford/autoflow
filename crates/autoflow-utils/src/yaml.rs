// YAML extraction and parsing utilities

/// Extract YAML content from agent output
/// Handles both ```yaml and ```yml code blocks
pub fn extract_yaml_from_output(output: &str) -> String {
    // Check if output contains yaml code block
    if output.contains("```yaml") {
        output
            .split("```yaml")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(output)
            .trim()
            .to_string()
    } else if output.contains("```yml") {
        output
            .split("```yml")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(output)
            .trim()
            .to_string()
    } else {
        // Assume entire output is YAML
        output.trim().to_string()
    }
}

/// Extract markdown content from agent output
pub fn extract_markdown_from_output(output: &str) -> String {
    if output.contains("```markdown") || output.contains("```md") {
        let marker = if output.contains("```markdown") {
            "```markdown"
        } else {
            "```md"
        };

        output
            .split(marker)
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(output)
            .trim()
            .to_string()
    } else {
        output.trim().to_string()
    }
}

/// Validate YAML syntax (basic check)
pub fn is_valid_yaml(content: &str) -> bool {
    serde_yaml::from_str::<serde_yaml::Value>(content).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_yaml_from_code_block() {
        let output = r#"Here's the YAML:
```yaml
project:
  name: test
```
Done!"#;

        let yaml = extract_yaml_from_output(output);
        assert_eq!(yaml, "project:\n  name: test");
    }

    #[test]
    fn test_extract_yaml_yml_variant() {
        let output = r#"```yml
key: value
```"#;

        let yaml = extract_yaml_from_output(output);
        assert_eq!(yaml, "key: value");
    }

    #[test]
    fn test_extract_yaml_no_code_block() {
        let yaml = "key: value";
        assert_eq!(extract_yaml_from_output(yaml), "key: value");
    }

    #[test]
    fn test_is_valid_yaml() {
        assert!(is_valid_yaml("key: value"));
        assert!(is_valid_yaml("project:\n  name: test"));
        assert!(!is_valid_yaml("{ invalid yaml"));
    }
}
