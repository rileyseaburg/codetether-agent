//! Convert tool-returned image metadata into vision content.

use codetether_provider_core::ContentPart;
use codetether_tool_core::ToolResult;

pub(crate) fn tool_images(result: &ToolResult) -> Vec<ContentPart> {
    codetether_tool_core::images::content(Some(&result.metadata))
}
