//! Provider request for swarm plan generation and repair.

use crate::provider::{CompletionRequest, ContentPart, Message, Provider, Role};
use anyhow::Result;

pub(super) async fn complete(
    provider: &dyn Provider,
    model: &str,
    prompt: String,
    temperature: f32,
) -> Result<String> {
    let response = provider
        .complete(CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: prompt }],
            }],
            tools: Vec::new(),
            model: model.to_string(),
            temperature: Some(temperature),
            top_p: None,
            max_tokens: Some(8192),
            stop: Vec::new(),
        })
        .await?;
    Ok(response
        .message
        .content
        .into_iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n"))
}
