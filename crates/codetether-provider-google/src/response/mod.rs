//! Response selection and completion assembly.
use crate::*;
mod content;
mod finish;
mod usage;
pub(crate) mod wire;
pub(crate) mod wire_error;
pub(crate) fn parse(completion: wire::ChatCompletion) -> Result<CompletionResponse> {
    let choice = completion
        .choices
        .into_iter()
        .next()
        .context("No choices in Google Gemini response")?;

    let (content_parts, has_tool_calls) = content::convert(choice.message);
    let finish_reason = finish::finish(choice.finish_reason.as_deref(), has_tool_calls);
    let usage = completion.usage.as_ref();

    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: content_parts,
        },
        usage: usage::convert(usage),
        finish_reason,
    })
}
