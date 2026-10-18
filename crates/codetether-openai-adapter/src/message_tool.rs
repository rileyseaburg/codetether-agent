//! Validate and convert one tool role message.
use crate::*;
use codetether_openai_wire::OpenAiRequestMessage;
pub(crate) fn convert(
    message: &OpenAiRequestMessage,
    index: usize,
) -> OpenAiApiResult<codetether_provider_core::Message> {
    let tool_call_id = message
        .tool_call_id
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            openai_bad_request(format!(
                "messages[{index}].tool_call_id is required for tool role"
            ))
        })?
        .to_string();
    Ok(codetether_provider_core::Message {
        role: codetether_provider_core::Role::Tool,
        content: vec![codetether_provider_core::ContentPart::ToolResult {
            tool_call_id,
            content: parse_openai_tool_content(&message.content),
        }],
    })
}
