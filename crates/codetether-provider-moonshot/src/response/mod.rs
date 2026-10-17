//! Response metadata, content, finish reasons, and usage accounting.
use crate::*;
mod content;
mod finish;
mod usage;
pub(crate) mod wire;
pub(crate) mod wire_error;
pub(crate) fn parse(response: wire::MoonshotResponse) -> Result<CompletionResponse> {
    // Log response metadata for debugging
    tracing::debug!(
        response_id = %response.id,
        model = %response.model,
        "Received Moonshot response"
    );

    let choice = response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices"))?;

    // Log reasoning/thinking content if present (Kimi K2 models)
    if let Some(ref reasoning) = choice.message.reasoning_content
        && !reasoning.is_empty()
    {
        tracing::info!(
            reasoning_len = reasoning.len(),
            "Model reasoning/thinking content received"
        );
        tracing::debug!(
            reasoning = %reasoning,
            "Full model reasoning"
        );
    }

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
