//! Corrective guidance retains developer attribution.
use codetether_provider_core::{ContentPart, Message, Role};

/// Build corrective guidance attributed to the developer.
/// # Arguments
/// * `text` — Guidance to insert into the conversation.
/// # Returns
/// A developer message containing one text part.
/// # Examples
/// ```
/// let message = codetether_prompt_loop::response::corrective_message("continue");
/// assert_eq!(message.role, codetether_provider_core::Role::Developer);
/// ```
pub fn message(text: &str) -> Message {
    Message {
        role: Role::Developer,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}
#[cfg(test)]
#[path = "guidance_tests.rs"]
mod tests;
