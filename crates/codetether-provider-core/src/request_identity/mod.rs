//! Fresh, model-visible routing identity, independent of persisted session prompts.
//!
//! [inject] updates a request projection; [system_prompt] handles transports
//! accepting one system string. Canonical block recognition protects caller text.
//!
//! # Examples
//! ```
//! let text = codetether_provider_core::request_identity::prompt("provider", "model");
//! assert!(text.contains("model"));
//! ```

mod block;
mod system;
pub use system::{caller_prompt, system_prompt};

use crate::{CompletionRequest, ContentPart, Message, Role};

/// Opening delimiter for canonical harness routing metadata.
pub const START: &str = "<codetether-harness-identity>\n";
/// Closing delimiter for canonical harness routing metadata.
pub const END: &str = "\n</codetether-harness-identity>";

/// Adds current routing identity without changing caller-owned conversation content.
/// Repeated wrapping replaces only the dedicated harness system message.
///
/// # Arguments
/// * `request` - Owned request projection to annotate.
/// * `provider` - Canonical provider identity for this request.
/// * `model` - Resolved routed model identity.
/// # Returns
/// The request with exactly one fresh leading harness identity message.
pub fn inject(mut request: CompletionRequest, provider: &str, model: &str) -> CompletionRequest {
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
///
/// # Arguments
/// * `provider` - Canonical provider name.
/// * `model` - Resolved model identifier.
/// # Returns
/// A marked block containing JSON-escaped routing metadata.
pub fn prompt(provider: &str, model: &str) -> String {
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
            if block::is_identity(text))
}
