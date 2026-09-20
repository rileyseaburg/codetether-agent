//! Tolerant extraction of a [`ReviewVerdict`] from free-form model output.
//!
//! Models wrap JSON in prose or fences. We take the last balanced `{ ... }`
//! object that parses as a verdict; anything else becomes `escalate` with
//! the raw tail as the reason so nothing is silently lost.

mod json_objects;

use super::verdict::ReviewVerdict;

/// Parse the reviewer's final message into a verdict.
pub fn parse(output: &str) -> ReviewVerdict {
    for candidate in json_objects::objects(output).rev() {
        if let Ok(verdict) = serde_json::from_str::<ReviewVerdict>(candidate) {
            return verdict;
        }
    }
    ReviewVerdict::escalate(format!(
        "reviewer returned no structured verdict: {}",
        tail(output, 300).trim()
    ))
}

/// The last `max` characters of `text`, on a char boundary.
fn tail(text: &str, max: usize) -> &str {
    let start = text
        .char_indices()
        .rev()
        .nth(max.saturating_sub(1))
        .map_or(0, |(index, _)| index);
    &text[start..]
}
