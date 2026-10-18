//! Validate and convert one user role message.
use crate::*;
use codetether_openai_wire::OpenAiRequestMessage;
pub(crate) fn convert(
    message: &OpenAiRequestMessage,
    index: usize,
    role: &str,
) -> OpenAiApiResult<codetether_provider_core::Message> {
    let content = parse_openai_content_parts(&message.content);
    if content.is_empty() {
        return Err(openai_bad_request(format!(
            "messages[{index}] must include text content"
        )));
    }

    Ok(codetether_provider_core::Message {
        role: if role == "system" {
            codetether_provider_core::Role::System
        } else {
            codetether_provider_core::Role::User
        },
        content,
    })
}
