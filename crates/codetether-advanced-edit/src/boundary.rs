//! boundary matching strategy.

/// Trimmed boundary matching
pub(super) fn trimmed_boundary_replacer(content: &str, find: &str) -> Vec<String> {
    let trimmed = find.trim();
    if trimmed == find {
        return vec![];
    }
    if content.contains(trimmed) {
        return vec![trimmed.to_string()];
    }
    vec![]
}
