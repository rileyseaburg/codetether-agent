use super::{IssueSeverity, ValidationIssue};

pub(super) fn append_issues(output: &mut String, issues: &[ValidationIssue]) {
    if issues.is_empty() {
        return;
    }
    output.push_str("\nIssues:\n");
    for issue in issues {
        append_issue(output, issue);
    }
}

fn append_issue(output: &mut String, issue: &ValidationIssue) {
    output.push_str(&format!(
        "  {} [{}] {}\n",
        severity_icon(issue.severity),
        format!("{:?}", issue.category).to_lowercase(),
        issue.message
    ));
    if let Some(suggestion) = &issue.suggestion {
        output.push_str(&format!("    → {suggestion}\n"));
    }
}

fn severity_icon(severity: IssueSeverity) -> &'static str {
    match severity {
        IssueSeverity::Error => "✗",
        IssueSeverity::Warning => "⚠",
        IssueSeverity::Info => "ℹ",
    }
}
