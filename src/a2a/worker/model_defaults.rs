//! Default model selection helpers for worker-run tasks.
//!
//! These defaults prefer Codex GPT-5.5 when the Codex provider is available,
//! with provider-specific fallbacks for explicitly selected providers.
//!
//! # Examples
//!
//! ```ignore
//! let model = default_model_for_provider("openai-codex", None);
//! assert_eq!(model, "gpt-5.5");
//! ```

/// Returns the default model identifier for a provider and model tier.
///
/// The returned value is tuned toward Codex GPT-5.5 before broader
/// provider-specific fallbacks.
///
/// # Examples
///
/// ```ignore
/// let model = default_model_for_provider("minimax", Some("fast"));
/// assert!(model.starts_with("MiniMax-"));
/// ```
pub(super) fn default_model_for_provider(provider: &str, model_tier: Option<&str>) -> String {
    match (provider, model_tier.unwrap_or("balanced")) {
        ("openai-codex" | "codex" | "chatgpt", _) => "gpt-5.5".to_string(),
        ("zhipuai" | "zai" | "zai-api", _) => "glm-5.1".to_string(),
        ("openrouter", _) => "z-ai/glm-5".to_string(),
        ("minimax-credits", _) => "MiniMax-M2.5-highspeed".to_string(),
        ("minimax", "fast" | "quick") => "MiniMax-M2.5-highspeed".to_string(),
        ("minimax", _) => "MiniMax-M2.5".to_string(),
        ("moonshotai", _) => "kimi-k2.5".to_string(),
        ("anthropic", _) => "claude-sonnet-4-20250514".to_string(),
        ("openai", _) => "o3".to_string(),
        ("google", "fast" | "quick") => "gemini-2.5-flash".to_string(),
        ("google", _) => "gemini-2.5-pro".to_string(),
        ("bedrock", "heavy" | "deep") => "us.anthropic.claude-opus-4-6-v1".to_string(),
        ("bedrock", _) => "amazon.nova-lite-v1:0".to_string(),
        _ => "glm-5.1".to_string(),
    }
}

/// Reports whether the selected model should run at `temperature = 1.0`.
///
/// Kimi, GLM, and MiniMax model families work best with the fixed temperature
/// policy used elsewhere in the agent stack.
///
/// # Examples
///
/// ```ignore
/// assert!(prefers_temperature_one("zai/glm-5.1"));
/// ```
pub(super) fn prefers_temperature_one(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("kimi-k2") || normalized.contains("glm-") || normalized.contains("minimax")
}

#[cfg(test)]
mod tests {
    use super::default_model_for_provider;

    #[test]
    fn openai_codex_defaults_to_gpt_5_5() {
        assert_eq!(default_model_for_provider("openai-codex", None), "gpt-5.5");
    }
}
