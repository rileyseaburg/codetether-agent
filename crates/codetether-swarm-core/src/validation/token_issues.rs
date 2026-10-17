use super::{IssueCategory, IssueSeverity, ValidationIssue};

pub(super) fn push_token_issues(
    total_tokens: usize,
    context_window: usize,
    exceeds_limit: bool,
    issues: &mut Vec<ValidationIssue>,
) {
    if exceeds_limit {
        issues.push(exceeds_issue(total_tokens, context_window));
    } else if total_tokens > context_window * 8 / 10 {
        issues.push(nearly_full_issue(total_tokens, context_window));
    }
}

fn exceeds_issue(total_tokens: usize, context_window: usize) -> ValidationIssue {
    token_issue(
        format!(
            "Estimated token usage ({total_tokens} tokens) exceeds model context window ({context_window} tokens)"
        ),
        "Reduce subtask complexity or split into smaller subtasks",
    )
}

fn nearly_full_issue(total_tokens: usize, context_window: usize) -> ValidationIssue {
    token_issue(
        format!(
            "Estimated token usage ({total_tokens} tokens) is at {}% of context window",
            (total_tokens * 100) / context_window
        ),
        "Consider reducing subtask complexity to avoid token limits",
    )
}

fn token_issue(message: String, suggestion: &str) -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Warning,
        category: IssueCategory::TokenEstimate,
        message,
        suggestion: Some(suggestion.to_string()),
    }
}
