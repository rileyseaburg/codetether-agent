//! Recover ordered JSON calls and remove recognized markup blocks.
use crate::{remaining_keys_as_args, tool_call_markup_re};
use serde_json::Value;

/// # Arguments
/// Receives assistant text containing optional tool_call markup.
/// # Returns
/// Cleaned text and ordered parsed calls; malformed blocks are omitted.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_call_parser::extract_markup_tool_calls("hello"), ("hello".into(), vec![]));
/// ```
pub fn extract_markup_tool_calls(text: &str) -> (String, Vec<(String, String)>) {
    let mut calls = Vec::new();
    let re = tool_call_markup_re();

    for capture in re.captures_iter(text) {
        let Some(block) = capture.get(1).map(|m| m.as_str()) else {
            continue;
        };
        let Ok(payload) = serde_json::from_str::<Value>(block) else {
            continue;
        };
        let Some(name) = payload.get("name").and_then(Value::as_str) else {
            continue;
        };
        let arguments = payload
            .get("arguments")
            .or_else(|| payload.get("args"))
            .or_else(|| payload.get("input"))
            .cloned()
            .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| remaining_keys_as_args(&payload));
        calls.push((name.to_string(), arguments));
    }

    let cleaned = re.replace_all(text, "").into_owned();
    (cleaned, calls)
}
