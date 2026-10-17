//! PRD response recovery: extract json.

use crate::{candidates::gather_json_candidates, sanitize::sanitize_json};

/// Extract JSON object from text that may be wrapped in markdown code blocks.
pub(crate) fn extract_json(text: &str) -> Option<String> {
    // Try each extraction strategy, applying sanitization if raw parse fails
    let candidates = gather_json_candidates(text);
    for candidate in candidates {
        // Try raw first
        if serde_json::from_str::<serde_json::Value>(&candidate).is_ok() {
            return Some(candidate);
        }
        // Try after sanitizing common LLM quirks
        let sanitized = sanitize_json(&candidate);
        if serde_json::from_str::<serde_json::Value>(&sanitized).is_ok() {
            return Some(sanitized);
        }
    }
    None
}
