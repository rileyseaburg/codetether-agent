//! Primary Bedrock output-capacity families.

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
/// use codetether_bedrock_protocol::estimate_max_output;
/// assert_eq!(estimate_max_output("us.anthropic.claude-sonnet-4-6"), 128_000);
/// assert_eq!(estimate_max_output("global.anthropic.claude-fable-5"), 128_000);
/// assert_eq!(estimate_max_output("amazon.nova-pro-v1:0"), 5_000);
/// ```
pub fn estimate_max_output(model_id: &str) -> usize {
    let id = model_id.to_lowercase();
    if crate::output_budget::has_encrypted_reasoning(model_id) {
        128_000
    } else if id.contains("openai.gpt-") {
        // Bedrock-hosted OpenAI GPT families use the 128k output envelope.
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
    } else {
        super::remaining::estimate(&id)
    }
}
