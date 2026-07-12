//! Detection of verification assertions lacking explicit evidence labels.

const CLAIMS: &[&str] = &[
    "validated",
    "tests passed",
    "test passed",
    "build passed",
    "checks passed",
    "no files were changed",
    "no changes were made",
    "no tests were run",
    "deployed",
    "proven",
    "completed successfully",
    "task complete",
    "work complete",
    "implementation complete",
];

pub(super) fn is_verification(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    CLAIMS.iter().any(|claim| lower.contains(claim))
}

fn has_level(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    super::level::labels()
        .iter()
        .any(|label| lower.contains(&label.to_ascii_lowercase()))
}

pub(super) fn is_valid(line: &str) -> bool {
    !is_verification(line) || (has_level(line) && super::evidence_source::is_named(line))
}

pub(super) fn violations(answer: &str) -> Vec<&str> {
    answer
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !is_valid(line))
        .collect()
}
