//! Project chronological provider messages while matching tool names across rows.
use crate::{PartProjection, hidden::is_hidden_context_marker};
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Message;
use std::collections::HashMap;

/// Project provider history into visible chat rows.
///
/// # Arguments
/// * `messages` — Chronological messages; call names are remembered across them.
///
/// # Returns
/// Visible rows with original part ordering and retained payload limits.
///
/// # Examples
/// ```
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::User, content: vec![codetether_provider_core::ContentPart::Text { text: "hello".into() }] };
/// let rows = codetether_chat_transcript::provider_messages_to_chat_messages([&message]);
/// assert_eq!(rows[0].content, "hello");
/// ```
pub fn provider_messages_to_chat_messages<'a>(
    messages: impl IntoIterator<Item = &'a Message>,
) -> Vec<ChatMessage> {
    let mut chat_messages = Vec::new();
    let mut tool_call_names = HashMap::new();

    for message in messages {
        if is_hidden_context_marker(message) {
            continue;
        }
        chat_messages.extend(chat_messages_from_provider_message(
            message,
            &mut tool_call_names,
        ));
    }

    chat_messages
}
fn chat_messages_from_provider_message(
    message: &Message,
    tool_call_names: &mut HashMap<String, String>,
) -> Vec<ChatMessage> {
    let mut projection = PartProjection::new(message.role, tool_call_names);
    for part in &message.content {
        projection.push(part);
    }
    projection.finish()
}
