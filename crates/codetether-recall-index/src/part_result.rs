//! Tool-result excerpts retain a visible truncation marker.
use super::truncate::truncate_bytes;
const MAX_FIELD_BYTES: usize = 2048;
pub(super) fn render(tool_call_id: &str, content: &str) -> String {
    let preview = truncate_bytes(content, MAX_FIELD_BYTES);
    let tail = if content.len() > MAX_FIELD_BYTES {
        " [...truncated]"
    } else {
        ""
    };
    format!("[ToolResult id={tool_call_id}]\n{preview}{tail}\n")
}
