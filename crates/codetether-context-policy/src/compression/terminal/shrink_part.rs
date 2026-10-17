//! Select the mutable text payload for each provider content kind.
use super::payload::shrink_string_payload;
use codetether_provider_core::ContentPart;

/// Shrink one content part to `cap_bytes` with a head/tail snippet.
/// Returns whether the part was changed.
pub(super) fn shrink_content_part(part: &mut ContentPart, cap_bytes: usize) -> bool {
    match part {
        ContentPart::Text { text } => shrink_string_payload(text, cap_bytes, "text"),
        ContentPart::ToolResult { content, .. } => {
            shrink_string_payload(content, cap_bytes, "tool_result")
        }
        ContentPart::ToolCall { arguments, .. } => {
            shrink_string_payload(arguments, cap_bytes, "tool_call_arguments")
        }
        ContentPart::Thinking { text, .. } => shrink_string_payload(text, cap_bytes, "thinking"),
        ContentPart::Image { .. } | ContentPart::File { .. } => false,
    }
}
