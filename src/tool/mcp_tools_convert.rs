//! Converts MCP results into text output and separate image metadata.

use crate::mcp::{CallToolResult, ToolContent};
use crate::tool::ToolResult;
use serde_json::json;

/// Preserve MCP error status, textual resources, and already-base64 image data.
pub(in crate::tool) fn result(result: CallToolResult) -> ToolResult {
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
