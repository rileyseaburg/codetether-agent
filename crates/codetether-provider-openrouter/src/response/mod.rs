//! Response interpretation and side-channel diagnostics.
use crate::*;
mod content;
mod diagnostics;
mod finish;
mod tools;
mod usage;
pub(crate) mod wire;
pub(crate) mod wire_error;
pub(crate) fn parse(response: OpenRouterResponse) -> Result<CompletionResponse> {
    diagnostics::metadata(&response);
    let choice = response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices"))?;
    diagnostics::details(choice);
    let (content, has_tool_calls) = content::content(choice);
    let finish_reason = finish::finish(choice, has_tool_calls);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: usage::usage(&response),
        finish_reason,
    })
}
