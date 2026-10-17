//! Shared forage tokenize rules.
use std::collections::HashSet;
pub(super) fn tokenize_for_alignment(value: &str) -> HashSet<String> {
    value
        .split(|c: char| !c.is_ascii_alphanumeric())
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| s.len() >= 4)
        .collect()
}
