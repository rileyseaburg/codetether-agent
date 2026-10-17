//! Buffered response parsing and conversion.
use crate::*;
mod content;
mod finish;
mod usage;
pub(crate) fn parse(text: &str) -> Result<CompletionResponse> {
    let completion: ChatCompletion = serde_json::from_str(text).context(format!(
        "Failed to parse Vertex AI GLM response: {}",
        codetether_util::truncate_bytes_safe(text, 200)
    ))?;

    let choice = completion
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in Vertex AI GLM response"))?;

    let (content, has_tool_calls) = content::content(choice);
    let finish_reason = finish::finish(choice, has_tool_calls);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: usage::usage(&completion),
        finish_reason,
    })
}
