//! Validate and convert one assistant role message.
use crate::*;
use codetether_openai_wire::OpenAiRequestMessage;
pub(crate) fn convert(
    message: &OpenAiRequestMessage,
    index: usize,
) -> OpenAiApiResult<codetether_provider_core::Message> {
    let mut content = parse_openai_content_parts(&message.content);
    for tool_call in &message.tool_calls {
        if tool_call.kind != "function" {
            return Err(openai_bad_request(format!(
                "messages[{index}].tool_calls only support `function`"
            )));
        }
        if tool_call.function.name.trim().is_empty() {
            return Err(openai_bad_request(format!(
                "messages[{index}].tool_calls[].function.name is required"
            )));
        }

        content.push(codetether_provider_core::ContentPart::ToolCall {
            id: tool_call.id.clone(),
            name: tool_call.function.name.clone(),
            arguments: tool_call.function.arguments.clone(),
            thought_signature: None,
        });
    }

    if content.is_empty() {
        return Err(openai_bad_request(format!(
            "messages[{index}] must include `content` or `tool_calls` for assistant role"
        )));
    }

    Ok(codetether_provider_core::Message {
        role: codetether_provider_core::Role::Assistant,
        content,
    })
}
