//! Response envelope and choice diagnostics in their original order.
use crate::*;
pub(super) fn metadata(response: &OpenRouterResponse) {
    // Log response metadata for debugging
    tracing::debug!(
        response_id = %response.id,
        provider = ?response.provider,
        model = ?response.model,
        "Received OpenRouter response"
    );
}
pub(super) fn details(choice: &OpenRouterChoice) {
    // Log native finish reason if present
    if let Some(ref native_reason) = choice.native_finish_reason {
        tracing::debug!(native_finish_reason = %native_reason, "OpenRouter native finish reason");
    }

    // Log reasoning content if present (e.g., Kimi K2 models)
    if let Some(ref reasoning) = choice.message.reasoning
        && !reasoning.is_empty()
    {
        tracing::info!(
            reasoning_len = reasoning.len(),
            "Model reasoning content received"
        );
        tracing::debug!(
            reasoning = %reasoning,
            "Full model reasoning"
        );
    }
    if let Some(ref details) = choice.message.reasoning_details
        && !details.is_empty()
    {
        tracing::debug!(
            reasoning_details = ?details,
            "Model reasoning details"
        );
    }
}
