//! Converts MCP results into text output and separate image metadata.

use codetether_mcp_core::types::{CallToolResult, ToolContent};
use codetether_tool_core::ToolResult;
use serde_json::json;

/// Preserve MCP error status, textual resources, and already-base64 image data.
///
/// # Arguments
/// * `result` - Remote response, including its error flag and ordered content.
/// # Returns
/// Joined text with image metadata suitable for local tool consumers.
/// # Examples
/// ```rust
/// use codetether_mcp_core::types::{CallToolResult,ToolContent};
/// let converted=codetether_mcp_tools::convert::result(CallToolResult{
///     content:vec![ToolContent::Text{text:"hello".into()}],is_error:false,
/// });
/// assert_eq!(converted.output,"hello");
/// assert!(converted.success);
/// ```
pub fn result(result: CallToolResult) -> ToolResult {
    let mut text = Vec::new();
    let mut images = Vec::new();
    for item in result.content {
        match item {
            ToolContent::Text { text: value } => text.push(value),
            ToolContent::Image { data, mime_type } => images.push(json!({
                "data_url": format!("data:{mime_type};base64,{data}"),
                "mime_type": mime_type,
            })),
            ToolContent::Resource { resource } => {
                text.push(serde_json::to_string(&resource).unwrap_or_default());
            }
        }
    }
    let output = text.join("\n");
    let result = if result.is_error {
        ToolResult::error(output)
    } else {
        ToolResult::success(output)
    };
    if images.is_empty() {
        result
    } else {
        result.with_metadata("image_data_url", json!(images))
    }
}

#[cfg(test)]
#[path = "mcp_tools_convert_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "mcp_tools_convert_text_tests.rs"]
mod text_tests;

#[cfg(test)]
#[path = "mcp_tools_convert_image_tests.rs"]
mod image_tests;
