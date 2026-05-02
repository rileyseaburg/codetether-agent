use super::{IssueCategory, IssueSeverity, ValidationIssue};
use crate::swarm::SubTask;

pub(super) fn missing_dependency_issue(subtask: &SubTask, dependency: &str) -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Error,
        category: IssueCategory::Dependencies,
        message: format!(
            "Subtask '{}' depends on missing subtask '{}'",
            subtask.name, dependency
        ),
        suggestion: Some(format!(
            "Create subtask with id '{dependency}' or remove the dependency"
        )),
    }
}

pub(super) fn cycle_issue(cycle: &[&str]) -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Error,
        category: IssueCategory::Dependencies,
        message: format!("Circular dependency detected: {}", cycle.join(" -> ")),
        suggestion: Some("Break the cycle by removing one of the dependencies".into()),
    }
}
