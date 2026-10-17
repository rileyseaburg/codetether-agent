//! Shared model pricing policy.

/// Multiplier applied to `input_price` for cache-write tokens. Only
/// Anthropic/Bedrock bill a surcharge for cache writes (1.25×). For
/// providers that cache implicitly (OpenAI, Gemini, GLM) this is `0.0`
/// because the write is bundled into the regular input price.
///
/// # Arguments
/// * `model` - Model identifier matched using the existing lowercase rules.
/// # Returns
/// The stored multiplier for this model family.
pub fn cache_write_multiplier(model: &str) -> f64 {
    let m = model.to_ascii_lowercase();
    if m.contains("claude") || m.contains("anthropic") {
        1.25
    } else {
        0.0
    }
}
