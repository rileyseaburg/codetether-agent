//! Construct the initial deterministic PRD generation request.

use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};
pub(crate) fn initial(prompt: &str, model: &str) -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_owned(),
            }],
        }],
        tools: vec![],
        model: model.to_string(),
        temperature: Some(0.3),
        top_p: None,
        max_tokens: Some(4096),
        stop: vec![],
    }
}
