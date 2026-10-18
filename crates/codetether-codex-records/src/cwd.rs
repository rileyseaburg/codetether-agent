//! Working-directory recovery from legacy environment context.
use serde_json::Value;

/// Extract the `<cwd>…</cwd>` value from a legacy `<environment_context>`
/// user message.
///
/// Legacy Codex rollouts inject a user message whose text looks like:
///
/// ```text
/// <environment_context>
///   <cwd>/home/riley/project</cwd>
///   <approval_policy>on-request</approval_policy>
///   ...
/// </environment_context>
/// ```
///
/// This function returns the trimmed inner text of the first `<cwd>` tag found
/// in any `content[].text` field of a `message` record. Returns `None` for
/// non-`message` records or when no `<cwd>` tag is present.
///
/// # Examples
///
/// ```rust
/// # use serde_json::json;
/// # use codetether_codex_records::extract_cwd_from_env_context;
/// let v = json!({
///     "type": "message",
///     "role": "user",
///     "content": [
///         {"type": "input_text",
///          "text": "<environment_context>\n  <cwd>/home/riley/proj</cwd>\n</environment_context>"}
///     ]
/// });
/// assert_eq!(
///     extract_cwd_from_env_context(&v).as_deref(),
///     Some("/home/riley/proj")
/// );
/// ```
/// # Arguments
/// * `value` — Legacy response item to inspect.
/// # Returns
/// Trimmed contents of the first working-directory tag, if present.
pub fn extract_cwd_from_env_context(value: &Value) -> Option<String> {
    if value.get("type").and_then(Value::as_str) != Some("message") {
        return None;
    }
    let content = value.get("content")?.as_array()?;
    for part in content {
        let text = part.get("text").and_then(Value::as_str)?;
        if let Some(start) = text.find("<cwd>")
            && let Some(end) = text[start + 5..].find("</cwd>")
        {
            return Some(text[start + 5..start + 5 + end].trim().to_string());
        }
    }
    None
}
