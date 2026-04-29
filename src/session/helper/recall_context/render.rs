//! Part rendering for recall context (budget-aware).

use crate::provider::ContentPart;

/// Maximum **bytes** retained from a single `ToolResult` or `ToolCall` args.
/// Uses byte offsets for O(1) truncation via `str::floor_char_boundary`.
const MAX_FIELD_BYTES: usize = 2048;

/// Render a single content part for recall, skipping thinking and
/// truncating large tool results.
pub fn render_part(part: &ContentPart) -> String {
    match part {
        ContentPart::Text { text } => format!("{text}\n"),
        ContentPart::Thinking { .. } => String::new(),
        ContentPart::ToolCall {
            id, name, arguments, ..
        } => {
            let args = truncate_bytes(arguments, MAX_FIELD_BYTES);
            format!("[ToolCall id={id} name={name}]\nargs: {args}\n")
        }
        ContentPart::ToolResult {
            tool_call_id,
            content,
        } => {
            let preview = truncate_bytes(content, MAX_FIELD_BYTES);
            let tail = if content.len() > MAX_FIELD_BYTES { " [...truncated]" } else { "" };
            format!("[ToolResult id={tool_call_id}]\n{preview}{tail}\n")
        }
        ContentPart::Image { mime_type, url } => {
            let mime = mime_type.clone().unwrap_or_else(|| "unknown".into());
            format!("[Image mime_type={mime} url_len={}]\n", url.len())
        }
        ContentPart::File { path, mime_type } => {
            let mime = mime_type.clone().unwrap_or_else(|| "unknown".into());
            format!("[File path={path} mime_type={mime}]\n")
        }
    }
}

/// Truncate to `max_len` bytes, respecting char boundaries.
pub(crate) fn truncate_bytes(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len { return s; }
    let mut end = max_len;
    while !s.is_char_boundary(end) && end > 0 {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod render_tests;
