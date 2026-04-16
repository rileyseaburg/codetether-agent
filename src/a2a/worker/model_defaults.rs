//! Default model selection helpers for worker-run tasks.
//!
//! These defaults prefer GLM and MiniMax families, with provider-specific
//! fallbacks only when those providers are explicitly selected.
//!
//! # Examples
//!
//! ```ignore
//! let model = default_model_for_provider("zai", None);
//! assert_eq!(model, "glm-5.1");
//! ```

/// Returns the default model identifier for a provider and model tier.
///
/// The returned value is tuned toward GLM and MiniMax usage before broader
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
        ("zhipuai" | "zai", _) => "glm-5.1".to_string(),
        ("openrouter", _) => "z-ai/glm-5".to_string(),
        ("minimax-credits", _) => "MiniMax-M2.5-highspeed".to_string(),
        ("minimax", "fast" | "quick") => "MiniMax-M2.5-highspeed".to_string(),
        ("minimax", _) => "MiniMax-M2.5".to_string(),
        ("moonshotai", _) => "kimi-k2.5".to_string(),
        ("anthropic", _) => "claude-sonnet-4-20250514".to_string(),
        ("openai", _) => "o3".to_string(),
        ("google", "fast" | "quick") => "gemini-2.5-flash".to_string(),
        ("google", _) => "gemini-2.5-pro".to_string(),
        ("bedrock", "heavy" | "deep") => "us.anthropic.claude-opus-4-6-v1:0".to_string(),
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
