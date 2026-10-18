//! Estimate serialized tool-definition token cost.
use codetether_provider_core::ToolDefinition;
use codetether_rlm::RlmChunker;

/// Estimate tokens from the serialized tool definitions.
/// # Arguments
/// * `tools` — Tool definitions included in the request.
/// # Returns
/// Serialized-definition token cost, or zero if serialization fails.
/// # Examples
/// ```
/// let definitions = Vec::new();
/// let estimate = codetether_request_tokens::estimate_tokens_for_tools(&definitions);
/// assert_eq!(estimate, codetether_rlm::RlmChunker::estimate_tokens("[]"));
/// ```
pub fn estimate_tokens_for_tools(tools: &[ToolDefinition]) -> usize {
    serde_json::to_string(tools)
        .ok()
        .map(|s| RlmChunker::estimate_tokens(&s))
        .unwrap_or(0)
}
