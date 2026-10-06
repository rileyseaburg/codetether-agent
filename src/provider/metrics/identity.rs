//! Fresh, model-visible routing identity, independent of persisted session prompts.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};

const START: &str = "<codetether-harness-identity>\n";
const END: &str = "\n</codetether-harness-identity>";

/// Adds current routing identity without changing caller-owned conversation content.
/// Repeated wrapping replaces only the dedicated harness system message.
pub(crate) fn inject(
    mut request: CompletionRequest,
    provider: &str,
    model: &str,
) -> CompletionRequest {
    request.messages.retain(|message| !is_identity(message));
    request.messages.insert(
        0,
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: prompt(provider, model),
            }],
        },
    );
    request
}

/// Formats authoritative routing metadata for transports without request messages.
pub(crate) fn prompt(provider: &str, model: &str) -> String {
    let identity = serde_json::json!({ "provider": provider, "model": model });
    format!(
        "{START}Authoritative routing identity supplied by the CodeTether harness \
         for this request: {identity}. When asked which model or provider you are, \
         report these exact values. They identify the routed model and provider, \
         not independently verified underlying weights. Do not infer identity \
         from training data, earlier turns, or a requested alias.{END}"
    )
}

fn is_identity(message: &Message) -> bool {
    matches!(message.role, Role::System)
        && matches!(message.content.as_slice(), [ContentPart::Text { text }]
            if text.starts_with(START) && text.ends_with(END))
}

#[cfg(test)]
#[path = "identity/tests.rs"]
mod tests;