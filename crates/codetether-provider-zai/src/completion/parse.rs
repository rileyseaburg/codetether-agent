//! Application buffered response decoding.
use crate::{CompletionResponse, Message, Role, response_wire::ZaiResponse};
use anyhow::{Context, Result};
pub(super) fn parse(text: &str) -> Result<CompletionResponse> {
    let response: ZaiResponse = serde_json::from_str(text).context(format!(
        "Failed to parse Z.AI response: {}",
        crate::preview::text(text, 200)
    ))?;

    let choice = response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in Z.AI response"))?;

    let (content, has_tool_calls) = super::content::convert(choice);
    let finish_reason = super::finish::finish(choice, has_tool_calls);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: super::usage::usage(&response),
        finish_reason,
    })
}
