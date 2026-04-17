//! Context-window and max-output-token estimates for Bedrock models.
//!
//! Bedrock does not expose per-model context limits consistently via its
//! `ListFoundationModels` / `ListInferenceProfiles` APIs, so we maintain
//! heuristic estimates keyed off model family substrings.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::{estimate_context_window, estimate_max_output};
//!
//! assert_eq!(estimate_context_window("us.anthropic.claude-opus-4-7-v1:0"), 1_000_000);
//! assert_eq!(estimate_max_output("us.anthropic.claude-opus-4-7-v1:0"), 128_000);
//! assert_eq!(estimate_context_window("amazon.nova-lite-v1:0"), 300_000);
//! ```

/// Estimate context-window size (input+output combined) for a Bedrock model.
///
/// # Arguments
///
/// * `model_id` — Full or aliased Bedrock model identifier.
///
/// # Returns
///
/// Estimated token capacity; returns 32_000 for unknown families.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::estimate_context_window;
/// assert_eq!(estimate_context_window("anthropic.claude-3-haiku-20240307-v1:0"), 200_000);
/// assert_eq!(estimate_context_window("unknown.model"), 32_000);
/// ```
pub fn estimate_context_window(model_id: &str) -> usize {
    let id = model_id.to_lowercase();
    if id.contains("claude-opus-4-7") {
        1_000_000
    } else if id.contains("anthropic") || id.contains("claude") {
        200_000
    } else if id.contains("nova-pro") || id.contains("nova-lite") || id.contains("nova-premier") {
        300_000
    } else if id.contains("nova-micro") || id.contains("nova-2") {
        128_000
    } else if id.contains("llama4") || id.contains("jamba") {
        256_000
    } else if id.contains("mistral") && !id.contains("large") && !id.contains("magistral") {
        32_000
    } else if id.contains("deepseek")
        || id.contains("llama3")
        || id.contains("mistral-large-3")
        || id.contains("magistral")
        || id.contains("qwen")
        || id.contains("kimi")
        || id.contains("glm")
        || id.contains("minimax")
        || id.contains("gemma")
        || id.contains("cohere")
        || id.contains("amazon")
    {
        128_000
    } else {
        32_000
    }
}

/// Estimate max completion-output tokens for a Bedrock model.
///
/// # Arguments
///
/// * `model_id` — Full or aliased Bedrock model identifier.
///
/// # Returns
///
/// Estimated maximum completion size in tokens; 4_096 for unknown families.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::estimate_max_output;
/// assert_eq!(estimate_max_output("us.anthropic.claude-sonnet-4-6-v1:0"), 128_000);
/// assert_eq!(estimate_max_output("amazon.nova-pro-v1:0"), 5_000);
/// ```
pub fn estimate_max_output(model_id: &str) -> usize {
    let id = model_id.to_lowercase();
    if id.contains("claude-opus-4-7") {
        128_000
    } else if id.contains("claude-opus-4-6")
        || id.contains("claude-opus-4-5")
        || id.contains("claude-opus-4-1")
        || id.contains("claude-opus-4")
    {
        32_000
    } else if id.contains("claude-sonnet-4-6") {
        128_000
    } else if id.contains("claude-sonnet-4-5")
        || id.contains("claude-sonnet-4")
        || id.contains("claude-3-7")
    {
        64_000
    } else if id.contains("claude-haiku-4-5") {
        16_384
    } else if id.contains("claude") {
        8_192
    } else if id.contains("nova") {
        5_000
    } else if id.contains("deepseek") || id.contains("llama4") || id.contains("mistral-large-3") {
        16_384
    } else if id.contains("llama") {
        4_096
    } else if id.contains("mistral")
        || id.contains("mixtral")
        || id.contains("qwen")
        || id.contains("kimi")
        || id.contains("glm")
        || id.contains("minimax")
        || id.contains("gemma")
        || id.contains("cohere")
        || id.contains("amazon")
    {
        8_192
    } else {
        4_096
    }
}
