//! Ordered message conversion with role-specific validation.
use crate::*;
use codetether_openai_wire::OpenAiRequestMessage;
/// Validate and convert messages in input order.
/// # Arguments
/// messages are the incoming wire conversation.
/// # Returns
/// Provider messages preserving roles and content ordering.
/// # Errors
/// Returns an invalid-request error for unsupported or incomplete input.
/// # Examples
/// ```
/// assert!(codetether_openai_adapter::convert_openai_messages(&[]).unwrap().is_empty());
/// ```
pub fn convert_openai_messages(
    messages: &[OpenAiRequestMessage],
) -> OpenAiApiResult<Vec<codetether_provider_core::Message>> {
    let mut converted = Vec::with_capacity(messages.len());
    for (index, message) in messages.iter().enumerate() {
        let role = message.role.to_ascii_lowercase();
        let message = match role.as_str() {
            "system" | "user" => crate::message_user::convert(message, index, &role)?,
            "assistant" => crate::message_assistant::convert(message, index)?,
            "tool" => crate::message_tool::convert(message, index)?,
            _ => {
                return Err(openai_bad_request(format!(
                    "messages[{index}].role `{}` is not supported",
                    message.role
                )));
            }
        };
        converted.push(message);
    }
    Ok(converted)
}
