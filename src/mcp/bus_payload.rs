//! Compact MCP tool results before live bus broadcast.

pub fn tool_response(output: &str) -> String {
    crate::bus::payload::bounded(output, "\n\n[truncated in MCP bus payload: tool response]")
}
