use super::Source;
use codetether_provider_core::{ContentPart, Message, Role};
/// Pick the best prompt to resubmit for `/continue`.
///
/// Prefers the in-flight prompt (the turn that stalled), then the most recent
/// user message in `session`, then a literal `"continue"` nudge.
pub fn resolve<H: Source>(host: &H) -> String {
    if let Some(prompt) = host.inflight() {
        if !prompt.trim().is_empty() {
            return prompt;
        }
    }
    if let Some(session) = host.session() {
        if let Some(prompt) = H::durable_prompt(session) {
            return prompt;
        }
        if let Some(text) = last_user_text(H::messages(session)) {
            return text;
        }
    }
    "continue".to_string()
}

/// Extract the text of the most recent [`Role::User`] message.
fn last_user_text(messages: &[Message]) -> Option<String> {
    messages
        .iter()
        .rev()
        .find(|m| m.role == Role::User)
        .and_then(|m| {
            m.content.iter().find_map(|part| match part {
                ContentPart::Text { text } if !text.trim().is_empty() => Some(text.clone()),
                _ => None,
            })
        })
}
