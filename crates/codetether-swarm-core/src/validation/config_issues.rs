use super::{IssueCategory, IssueSeverity, ValidationIssue};

pub(super) fn config_error(message: &str, suggestion: &str) -> ValidationIssue {
    config_issue(IssueSeverity::Error, message.to_string(), suggestion)
}

pub(super) fn config_warning(message: String, suggestion: &str) -> ValidationIssue {
    config_issue(IssueSeverity::Warning, message, suggestion)
}

fn config_issue(severity: IssueSeverity, message: String, suggestion: &str) -> ValidationIssue {
    ValidationIssue {
        severity,
        category: IssueCategory::Configuration,
        message,
        suggestion: Some(suggestion.to_string()),
    }
}
