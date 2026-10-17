//! Text-block selection for conversation context and recall.
use codetether_provider_core::{ContentPart, Message, Role};

/// Join text blocks in their original order.
///
/// # Arguments
/// * `parts` - Content blocks to inspect.
///
/// # Returns
/// Text blocks separated by newlines; other content is omitted.
///
/// # Examples
/// ```
/// use codetether_context_policy::transcript::extract_text_content;
/// use codetether_provider_core::ContentPart;
/// let parts = [ContentPart::Text { text: "hello".into() }];
/// assert_eq!(extract_text_content(&parts), "hello");
/// ```
pub fn extract_text_content(parts: &[ContentPart]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Find the latest user message containing nonblank text.
///
/// # Arguments
/// * `messages` - Conversation in chronological order.
///
/// # Returns
/// Joined text from the latest matching user message, or None.
///
/// # Examples
/// ```
/// use codetether_context_policy::transcript::latest_user_text;
/// assert_eq!(latest_user_text(&[]), None);
/// ```
pub fn latest_user_text(messages: &[Message]) -> Option<String> {
    messages.iter().rev().find_map(|m| {
        if m.role != Role::User {
            return None;
        }
        let text = extract_text_content(&m.content);
        if text.trim().is_empty() {
            None
        } else {
            Some(text)
        }
    })
}
