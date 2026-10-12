//! Regression coverage for the extracted provider's wire conversions.
mod messages;
mod requests;
mod responses;

pub(super) fn request(text: &str, max_tokens: usize) -> crate::CompletionRequest {
    crate::CompletionRequest {
        messages: vec![crate::Message {
            role: crate::Role::User,
            content: vec![crate::ContentPart::Text { text: text.into() }],
        }],
        tools: vec![],
        model: "glm-5.1".into(),
        temperature: None,
        top_p: None,
        max_tokens: Some(max_tokens),
        stop: vec![],
    }
}
