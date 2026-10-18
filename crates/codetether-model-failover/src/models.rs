//! Fallback model lists per provider.

/// Returns the ordered list of preferred fallback models for a given provider.
///
/// # Arguments
/// * `provider_name` — provider name input.
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert!(!codetether_model_failover::smart_switch_preferred_models("minimax").is_empty());
/// ```
pub fn smart_switch_preferred_models(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M3", "MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" | "zai-api" => &["glm-5", "glm-4.7", "glm-4.7-flash"],
        "openai-codex" => codetether_provider_core::codex::model_catalog::current_models(),
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "moonshotai/kimi-k2:free"],
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openai" => &["gpt-4o-mini", "gpt-4.1"],
        "anthropic" => &["claude-sonnet-4-20250514", "claude-3-5-sonnet-20241022"],
        "google" => &["gemini-2.5-flash", "gemini-2.5-pro"],
        "gemini-web" => &["gemini-web-pro"],
        _ => &[],
    }
}
pub use super::overload_fallback::codex_overload_fallback;
