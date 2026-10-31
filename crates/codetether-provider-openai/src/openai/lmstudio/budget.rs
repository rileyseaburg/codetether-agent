//! Conservative text budget at the final provider request boundary.
//!
//! Estimate ASCII at three bytes per token and non-ASCII at one byte per token,
//! plus template/framing headroom and a 20% safety reserve. This is not a tokenizer.
//! Include system/developer messages, tools, tool results and output. Reject
//! media rather than pretending a short image URL implies a small token cost.

use super::*;
use anyhow::ensure;
use serde_json::json;

pub(super) fn output_ceiling(capacity: usize) -> usize {
    (capacity / 4).clamp(1, 1024)
}

pub(super) fn cap_output(request: &mut CompletionRequest, capacity: usize) -> Result<()> {
    ensure!(
        !request
            .messages
            .iter()
            .flat_map(|message| &message.content)
            .any(|part| matches!(part, ContentPart::Image { .. } | ContentPart::File { .. })),
        "LM Studio context guard cannot safely size image/file inputs; use text-only input"
    );
    let messages = OpenAIProvider::convert_messages(&request.messages)?;
    let tools = OpenAIProvider::convert_tools(&request.tools)?;
    let framing = 256usize
        .saturating_add(messages.len().saturating_mul(32))
        .saturating_add(tools.len().saturating_mul(64));
    let serialized = serde_json::to_vec(&json!({"messages": messages, "tools": tools}))?;
    let ascii = serialized.iter().filter(|byte| byte.is_ascii()).count();
    let prompt = ascii
        .div_ceil(3)
        .saturating_add(serialized.len() - ascii)
        .saturating_add(framing);
    let available = (capacity.saturating_mul(4) / 5).saturating_sub(prompt);
    ensure!(
        available > 0,
        "LM Studio context budget exceeded for '{}': estimated input {prompt} tokens, loaded capacity {capacity} with 20% reserved; shorten history/system prompt or reduce tools before retrying",
        request.model
    );
    let requested = request
        .max_tokens
        .unwrap_or_else(|| output_ceiling(capacity));
    ensure!(requested > 0, "LM Studio max_tokens must be positive");
    request.max_tokens = Some(requested.min(output_ceiling(capacity)).min(available));
    Ok(())
}
