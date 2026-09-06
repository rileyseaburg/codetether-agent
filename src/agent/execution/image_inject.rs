//! Convert tool-returned image metadata into vision content.

use crate::provider::ContentPart;
use crate::tool::ToolResult;

pub(super) fn tool_images(result: &ToolResult) -> Vec<ContentPart> {
    crate::tool::result_images::content(Some(&result.metadata))
}
