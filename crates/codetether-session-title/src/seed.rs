//! Extract the first user message text for model-backed title generation.
use codetether_provider_core::{ContentPart, Message, Role};

/// Extract the first user message's plain text, if any.
/// # Arguments
/// `messages` is the ordered transcript.
/// # Returns
/// Nonblank text from the first user message, or `None`.
/// # Examples
/// ```rust
/// use codetether_session_title::first_user_text;
/// let value = first_user_text(&[]);
/// # let _ = value;
/// ```
pub fn first_user_text(messages: &[Message]) -> Option<String> {
    let msg = messages.iter().find(|m| m.role == Role::User)?;
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");
    (!text.trim().is_empty()).then_some(text)
}
