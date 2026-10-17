//! Construct repair messages while preserving conversation roles and request limits.

use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};
pub(crate) fn build(
    prompt: &str,
    model: &str,
    last_text: &str,
    last_error: &str,
    attempt: usize,
) -> CompletionRequest {
    tracing::warn!(target:"codetether_agent::cli::go_ralph",
        attempt,
        error = %last_error,
        "PRD JSON extraction failed, retrying with repair prompt"
    );
    let repair = crate::repair_text::build(last_text, last_error);
    CompletionRequest {
        messages: vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: prompt.to_owned(),
                }],
            },
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: last_text.to_owned(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: repair }],
            },
        ],
        tools: vec![],
        model: model.to_string(),
        temperature: Some(0.1),
        top_p: None,
        max_tokens: Some(4096),
        stop: vec![],
    }
}
