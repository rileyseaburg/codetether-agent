//! Effective output-token budget for Bedrock Converse requests.
//!
//! Models with always-on **encrypted reasoning** (Claude Fable 5, Opus 4.7)
//! burn output tokens on thinking that is invisible to the caller (Bedrock
//! returns only a `reasoningContent` signature). With a small `maxTokens`
//! the entire budget can be consumed before any visible text or tool call
//! is emitted, yielding an empty assistant message with `max_tokens` stop.
//! This module floors the budget for those models so thinking plus a real
//! answer fit.

/// Minimum output budget applied to encrypted-reasoning models.
pub const THINKING_OUTPUT_FLOOR: usize = 32_000;

/// Whether the model spends output tokens on reasoning the caller never
/// sees (signature-only `reasoningContent` blocks).
///
/// # Arguments
/// * `model_id` - Resolved model identifier.
///
/// # Returns
/// Whether to reserve output capacity for encrypted reasoning.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::output_budget::has_encrypted_reasoning("claude-opus-5"));
/// ```
pub fn has_encrypted_reasoning(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains("claude-fable-5")
        || id.contains("claude-opus-5")
        || id.contains("claude-sonnet-5")
        || id.contains("claude-opus-4-7")
        || id.contains("claude-opus-4.7")
        || id.contains("claude-opus-4-8")
        || id.contains("claude-opus-4.8")
}

/// Resolve the `inferenceConfig.maxTokens` value for a request.
///
/// Encrypted-reasoning models get a floor of [`THINKING_OUTPUT_FLOOR`]
/// (capped by the model's max output); other models keep the requested
/// value or the legacy 8 192 default.
///
/// # Arguments
/// * `requested` - Optional requested output budget.
/// * `model_id` - Resolved model identifier.
///
/// # Returns
/// The output budget after model-specific limits are applied.
///
/// # Examples
/// ```
/// assert_eq!(codetether_bedrock_protocol::output_budget::effective_max_tokens(Some(1024), "amazon.nova-pro"), 1024);
/// ```
pub fn effective_max_tokens(requested: Option<usize>, model_id: &str) -> usize {
    let requested = requested.unwrap_or(8_192);
    if has_encrypted_reasoning(model_id) {
        requested
            .max(THINKING_OUTPUT_FLOOR)
            .min(super::estimate_max_output(model_id))
    } else {
        requested
    }
}
#[cfg(test)]
#[path = "output_budget/tests.rs"]
mod tests;
