//! Tool argument pretty-printing.

/// Pretty-print tool call arguments when reasonably sized.
#[allow(dead_code)]
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

/// Build a size-limited preview for tool arguments.
#[allow(dead_code)]
pub fn build_tool_arguments_preview(
    tool_name: &str,
    arguments: &str,
    max_lines: usize,
    max_bytes: usize,
) -> (String, bool) {
    let formatted = format_tool_call_arguments(tool_name, arguments);
    super::text_preview::build_text_preview(&formatted, max_lines, max_bytes)
}
