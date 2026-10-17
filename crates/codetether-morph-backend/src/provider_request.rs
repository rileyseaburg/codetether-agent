//! Completion request with the original editing prompt and defaults.
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};
pub(super) fn build(prompt: &str, model: String) -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        }],
        tools: vec![],
        model,
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    }
}
