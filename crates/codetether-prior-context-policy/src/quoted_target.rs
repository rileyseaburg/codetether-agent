//! Recognize quoted target names without treating quoted commands as directives.

pub(super) fn target_only(input: &str) -> bool {
    let normalized = input
        .to_lowercase()
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    super::targets::PHRASES.contains(&normalized.as_str())
        || ["memory", "scope", "sessions"].contains(&normalized.as_str())
}
