//! Extract summary text from provider responses.
//! Only human-readable text/thinking blocks are included in the final summary.

use crate::provider::{CompletionResponse, ContentPart};

/// Convert a completion response into displayable summary text.
pub fn summarize_response(response: CompletionResponse) -> String {
    response
        .message
        .content
        .iter()
        .filter_map(text_part)
        .collect::<Vec<_>>()
        .join("\n")
}

fn text_part(part: &ContentPart) -> Option<&str> {
    match part {
        ContentPart::Text { text } | ContentPart::Thinking { text } => Some(text.as_str()),
        ContentPart::ToolCall { .. }
        | ContentPart::ToolResult { .. }
        | ContentPart::Image { .. }
        | ContentPart::File { .. } => None,
    }
}
