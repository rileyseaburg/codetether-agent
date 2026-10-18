//! Convert local tool results to MCP blocks without burying images in prose.

use crate::types::{CallToolResult, ToolContent};
use codetether_provider_core::ContentPart;
use codetether_tool_core::ToolResult;

/// Convert a local tool result into MCP text/image content blocks.
///
/// # Arguments
/// * `result` — Tool output with optional canonical image metadata.
/// # Returns
/// An MCP result preserving failure status and supported image attachments.
/// # Examples
/// ```
/// let result = codetether_mcp_core::tool_result::convert(codetether_tool_core::ToolResult::success("done"));
/// assert!(!result.is_error);
/// assert_eq!(result.content.len(), 1);
/// ```
pub fn convert(result: ToolResult) -> CallToolResult {
    let images = codetether_tool_core::images::content(Some(&result.metadata));
    let mut content = vec![ToolContent::Text {
        text: result.output,
    }];
    let mut is_error = !result.success;
    for part in images {
        let ContentPart::Image { url, .. } = part else {
            continue;
        };
        if let Some((mime, data)) = split(&url) {
            content.push(ToolContent::Image {
                data: data.into(),
                mime_type: mime.into(),
            });
        } else {
            is_error = true;
            content.push(ToolContent::Text {
                text: "Image attachment requires a base64 image data URL for MCP transport".into(),
            });
        }
    }
    CallToolResult { content, is_error }
}

fn split(url: &str) -> Option<(&str, &str)> {
    let (header, data) = url.strip_prefix("data:")?.split_once(',')?;
    let mime = header.strip_suffix(";base64")?;
    (mime.starts_with("image/") && !data.is_empty()).then_some((mime, data))
}

#[cfg(test)]
mod tests;
