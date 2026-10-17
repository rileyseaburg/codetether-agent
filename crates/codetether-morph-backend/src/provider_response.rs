//! Extract text parts and reject empty provider output.
use anyhow::{Result, anyhow};
use codetether_provider_core::{CompletionResponse, ContentPart};
pub(super) fn content(response: CompletionResponse) -> Result<String> {
    let content = response
        .message
        .content
        .into_iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(anyhow!("Morph backend returned empty content"));
    }
    Ok(content)
}
