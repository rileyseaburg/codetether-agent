//! Extraction and safety limits for Gemini Web tool-call markup.

mod markup;
mod parse;
mod scan;

use std::collections::HashSet;

pub(super) const MAX_PER_RESPONSE: usize = 8;

pub(super) fn extract(text: &str) -> (String, Vec<(String, String)>) {
    let normalized = markup::normalize(text);
    let blocks = scan::blocks(&normalized);
    let mut recognized = Vec::new();
    let mut unique = HashSet::new();
    let mut calls = Vec::new();
    let mut parsed_count = 0;
    for block in &blocks {
        let Some(call) = parse::call(block.json) else {
            continue;
        };
        recognized.push(block.range.clone());
        parsed_count += 1;
        if unique.insert(call.clone()) && calls.len() < MAX_PER_RESPONSE {
            calls.push(call);
        }
    }
    if calls.is_empty() {
        return (text.to_string(), calls);
    }
    if parsed_count > calls.len() {
        tracing::warn!(
            parsed_count,
            emitted_count = calls.len(),
            limit = MAX_PER_RESPONSE,
            "Gemini Web tool calls deduplicated or capped; waiting for real results"
        );
    }
    let without_calls = scan::remove(&normalized, &recognized);
    let cleaned = markup::result_regex()
        .replace_all(&without_calls, "")
        .trim()
        .to_string();
    (cleaned, calls)
}

pub(super) fn contains_unparsed_markup(text: &str) -> bool {
    markup::contains_call_opener(text)
}

pub(super) fn contains_result_markup(text: &str) -> bool {
    markup::contains_result_opener(text)
}

#[cfg(test)]
#[path = "tool_calls_tests.rs"]
mod tests;
