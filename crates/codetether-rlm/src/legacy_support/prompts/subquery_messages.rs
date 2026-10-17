//! Build the paired messages used by both legacy sub-query paths.
use codetether_provider_core::{ContentPart, Message, Role};
pub(super) fn subquery_messages(context: &str, query: &str) -> Vec<Message> {
    let truncated_context = super::subquery_context::subquery_context(context);
    vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "You are a focused analysis assistant. Answer the question based on the provided context. Be concise.".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("Context:\n{}\n\nQuestion: {}", truncated_context, query),
                }],
            },
        ]
}
