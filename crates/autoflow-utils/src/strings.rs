// String manipulation utilities

/// Sanitize string for use in branch name
/// Converts to lowercase, replaces non-alphanumeric with dashes
pub fn sanitize_branch_name(description: &str, max_words: usize) -> String {
    description
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .take(max_words)
        .collect::<Vec<_>>()
        .join("-")
}

/// Truncate string to max length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Pluralize word based on count
pub fn pluralize(word: &str, count: usize) -> String {
    if count == 1 {
        word.to_string()
    } else {
        format!("{}s", word)
    }
}

/// Format duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_branch_name() {
        assert_eq!(
            sanitize_branch_name("Fix: Login button not working!", 5),
            "fix-login-button-not-working"
        );
        assert_eq!(
            sanitize_branch_name("Add payment @@ processing", 3),
            "add-payment-processing"
        );
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a long string", 10), "this is...");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("sprint", 1), "sprint");
        assert_eq!(pluralize("sprint", 0), "sprints");
        assert_eq!(pluralize("sprint", 5), "sprints");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3665), "1h 1m");
    }
}
