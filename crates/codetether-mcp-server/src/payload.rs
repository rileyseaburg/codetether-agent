//! Bounded tool-response text for MCP bus events.
/// Compact a tool response for live bus publication.
/// # Arguments
/// * `output` - Full tool output text.
/// # Returns
/// Output within the shared bus byte budget, with an MCP-specific truncation note.
/// # Examples
/// ```
/// assert_eq!(codetether_mcp_server::payload::tool_response("ok"), "ok");
/// ```
pub fn tool_response(output: &str) -> String {
    codetether_bus_core::payload::bounded(
        output,
        "\n\n[truncated in MCP bus payload: tool response]",
    )
}
