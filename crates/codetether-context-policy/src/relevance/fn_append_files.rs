//! append files policy.
use super::*;

/// Extract path-like tokens from `text` and append unique ones to `out`.
///
/// Heuristic: tokens that look like filesystem paths (contain `/` but
/// not `://`, so URLs are excluded) or end with a common source-file
/// extension. Intentionally conservative — this feeds
/// [`Bucket`] projection, so false positives directly harm delegation
/// calibration (over-reporting `Chained` dependency).
pub(super) fn append_files(text: &str, out: &mut Vec<String>) {
    for raw in text.split(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | '(' | ')' | '`')) {
        let trimmed = raw.trim_matches(|c: char| matches!(c, '"' | '\'' | '.'));
        if trimmed.is_empty() || trimmed.len() < 3 {
            continue;
        }
        let looks_like_path =
            (trimmed.contains('/') && !trimmed.contains("://") && trimmed.len() > 3)
                || ends_with_source_ext(trimmed);
        if looks_like_path && !out.contains(&trimmed.to_string()) {
            out.push(trimmed.to_string());
        }
    }
}
