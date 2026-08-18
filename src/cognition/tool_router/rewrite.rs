//! Response rewriting: replace descriptive text with structured tool calls.

use uuid::Uuid;

use super::parsed_call::ParsedToolCall;
use crate::provider::{CompletionResponse, ContentPart, FinishReason};

/// Rewrite `response` so tool calls are structured rather than described.
///
/// `<tool_call>` blocks are stripped from text parts, but surrounding reasoning
/// text is kept so the model retains its chain of thought on later turns. Parts
/// left empty are dropped.
pub(super) fn rewrite_response(
    mut response: CompletionResponse,
    calls: Vec<ParsedToolCall>,
) -> CompletionResponse {
    strip_tool_call_blocks(&mut response);
    for call in calls {
        response.message.content.push(ContentPart::ToolCall {
            id: format!("fc_{}", Uuid::new_v4()),
            name: call.name,
            arguments: call.arguments,
            thought_signature: None,
        });
    }
    // Signal the session loop that tool calls are present.
    response.finish_reason = FinishReason::ToolCalls;
    response
}

/// Remove `<tool_call>` blocks and any text parts they emptied.
fn strip_tool_call_blocks(response: &mut CompletionResponse) {
    let re = regex::Regex::new(r"(?s)<tool_call>.*?</tool_call>")
        .expect("tool_call strip pattern is a valid regex");
    for part in &mut response.message.content {
        if let ContentPart::Text { text } = part {
            *text = re.replace_all(text, "").trim().to_string();
        }
    }
    response
        .message
        .content
        .retain(|p| !matches!(p, ContentPart::Text { text } if text.is_empty()));
}
