//! Conservative extraction of explicit execution prohibitions.

use super::constraint_entry::ConstraintEntry;

const PROHIBITIONS: &[&str] = &[
    "do not",
    "don't",
    "must not",
    "never",
    "prohibited",
    "forbid",
];
const EXECUTION: &[&str] = &[
    "test", "build", "compile", "compiler", "lint", "watcher", "command", "shell", "network",
];

pub(super) fn from_text(source: &'static str, text: &str) -> Vec<ConstraintEntry> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && is_execution_prohibition(line))
        .map(|line| ConstraintEntry::new(source, line))
        .collect()
}

fn is_execution_prohibition(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    PROHIBITIONS.iter().any(|term| lower.contains(term))
        && EXECUTION.iter().any(|term| lower.contains(term))
}
