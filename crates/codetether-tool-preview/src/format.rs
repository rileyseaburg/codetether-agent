//! Pretty-print small tool calls and extract question text.
//! Tool argument pretty-printing.

/// Pretty-print tool call arguments when reasonably sized.
#[allow(dead_code)]
///
/// # Arguments
/// * `name` - Tool name, with special handling for questions.
/// * `arguments` - JSON or literal arguments.
///
/// # Returns
/// Pretty JSON, question text, or the original string when parsing is skipped.
///
/// # Examples
/// ```
/// assert_eq!(codetether_tool_preview::format_tool_call_arguments("tool", "literal"), "literal");
/// ```
pub fn format_tool_call_arguments(name: &str, arguments: &str) -> String {
    const MAX_BYTES: usize = 4096;
    if arguments.len() > MAX_BYTES {
        return arguments.to_string();
    }
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return arguments.to_string();
    };
    if name == "question" {
        if let Some(q) = parsed.get("question").and_then(serde_json::Value::as_str) {
            return q.to_string();
        }
    }
    serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| arguments.to_string())
}
