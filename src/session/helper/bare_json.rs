//! Salvage bare-JSON tool-call narrations.
//!
//! Some flaky providers narrate a tool call as a raw JSON object —
//! `{"name":"bash","input":{...}}` — without the `<tool_call>` wrapper that
//! [`super::markup`] expects. Treated as prose, the agent loop exits with
//! zero tool calls (the swarm "0 usable changes" failure). This module
//! salvages those, but conservatively: it only fires when the trimmed text
//! is a single JSON object whose `name` matches a known tool, so ordinary
//! prose containing braces is never misread as a call.

use serde_json::Value;
use std::collections::HashSet;

#[cfg(test)]
#[path = "bare_json_tests.rs"]
mod bare_json_tests;

/// Try to read a bare-JSON tool call from `text`.
///
/// Returns `Some((name, arguments_json))` only when `text` (trimmed, with an
/// optional ```` ```json ```` fence removed) is exactly one JSON object whose
/// `name` is in `allowed`. Arguments are read from `arguments`/`args`/`input`,
/// otherwise from all non-`name` keys.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::bare_json::extract_bare_json_tool_call;
/// use std::collections::HashSet;
/// let allowed: HashSet<&str> = ["bash"].into_iter().collect();
/// let got = extract_bare_json_tool_call(
///     "{\"name\":\"bash\",\"input\":{\"command\":\"ls\"}}",
///     &allowed,
/// );
/// assert_eq!(got.unwrap().0, "bash");
/// assert!(extract_bare_json_tool_call("just prose {x}", &allowed).is_none());
/// ```
pub fn extract_bare_json_tool_call(
    text: &str,
    allowed: &HashSet<&str>,
) -> Option<(String, String)> {
    let trimmed = strip_json_fence(text.trim());
    if !(trimmed.starts_with('{') && trimmed.ends_with('}')) {
        return None;
    }
    let payload: Value = serde_json::from_str(trimmed).ok()?;
    let name = payload.get("name")?.as_str()?.to_string();
    if !allowed.contains(name.as_str()) {
        return None;
    }
    let arguments = payload
        .get("arguments")
        .or_else(|| payload.get("args"))
        .or_else(|| payload.get("input"))
        .cloned()
        .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|| remaining_keys_as_args(&payload));
    Some((name, arguments))
}

fn strip_json_fence(text: &str) -> &str {
    text.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

/// Build an arguments JSON object from a payload's keys excluding `name`.
///
/// Used both here and by [`super::markup`] when a tool call omits an explicit
/// `arguments`/`args`/`input` key.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::bare_json::remaining_keys_as_args;
/// use serde_json::json;
/// let args = remaining_keys_as_args(&json!({"name":"bash","command":"ls"}));
/// assert_eq!(args, "{\"command\":\"ls\"}");
/// ```
pub fn remaining_keys_as_args(payload: &Value) -> String {
    let Some(obj) = payload.as_object() else {
        return "{}".to_string();
    };
    let params: serde_json::Map<String, Value> = obj
        .iter()
        .filter(|(k, _)| *k != "name")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    serde_json::to_string(&Value::Object(params)).unwrap_or_else(|_| "{}".to_string())
}
