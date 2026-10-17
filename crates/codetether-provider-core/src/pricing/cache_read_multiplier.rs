//! Shared model pricing policy.

/// Multiplier applied to `input_price` for cache-read tokens. Varies by
/// provider family:
///
/// * Anthropic / Bedrock Claude — 10% (0.1×)
/// * OpenAI (GPT-4o, GPT-5, Codex) — 50% (0.5×)
/// * Google Gemini — 25% (0.25×)
/// * Z.AI GLM / Moonshot — 20% (0.2×) *(best-effort default)*
///
/// Returns `0.1` as a conservative default so pricing never silently
/// over-counts for an unknown model.
///
/// # Arguments
/// * `model` - Model identifier matched using the existing lowercase rules.
/// # Returns
/// The stored multiplier for this model family.
pub fn cache_read_multiplier(model: &str) -> f64 {
    let m = model.to_ascii_lowercase();
    if m.contains("claude") || m.contains("anthropic") {
        0.10
    } else if m.contains("gpt-") || m.contains("codex") || m.contains("o1") || m.contains("o3") {
        0.50
    } else if m.contains("gemini") {
        0.25
    } else if m.contains("glm") || m.contains("kimi") || m.contains("k1.") {
        0.20
    } else {
        0.10
    }
}
