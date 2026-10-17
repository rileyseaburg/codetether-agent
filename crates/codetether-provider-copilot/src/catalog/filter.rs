//! Legacy dated model detection.
use crate::*;
/// Check if a model ID is a dated variant (e.g. "gpt-4o-2024-05-13") that should
/// be filtered out in favor of the canonical alias (e.g. "gpt-4o").
pub(crate) fn is_dated_model_variant(id: &str) -> bool {
    // Match IDs ending in a YYYY-MM-DD date suffix
    let bytes = id.as_bytes();
    if bytes.len() < 11 {
        return false;
    }
    // Check for "-YYYY-MM-DD" at end
    let tail = &id[id.len() - 11..];
    tail.starts_with('-')
        && tail[1..5].bytes().all(|b| b.is_ascii_digit())
        && tail.as_bytes()[5] == b'-'
        && tail[6..8].bytes().all(|b| b.is_ascii_digit())
        && tail.as_bytes()[8] == b'-'
        && tail[9..11].bytes().all(|b| b.is_ascii_digit())
}
