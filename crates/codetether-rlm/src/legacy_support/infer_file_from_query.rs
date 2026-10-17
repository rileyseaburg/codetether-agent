//! Legacy executor infer file from query.
use super::*;

/// Infer the file following the last case-insensitive in separator.
///
/// # Arguments
/// * `query` - Original analysis question.
/// # Returns
/// The first trailing token with surrounding punctuation removed, if present.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::legacy_support::infer_file_from_query("find x in src/main.rs").as_deref(), Some("src/main.rs"));
/// ```
pub fn infer_file_from_query(query: &str) -> Option<String> {
    let lower = query.to_lowercase();
    let idx = lower.rfind(" in ")?;
    let candidate = query[idx + 4..]
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|c: char| c == '"' || c == '\'' || c == '`' || c == '.' || c == ',');
    if candidate.is_empty() {
        None
    } else {
        Some(candidate.to_string())
    }
}
