//! Fallback model lists per provider.

/// Returns the ordered list of preferred fallback models for a given provider.
pub fn smart_switch_preferred_models(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" | "zai-api" => &["glm-5", "glm-4.7", "glm-4.7-flash"],
        "openai-codex" => &["gpt-5.5", "gpt-5-mini", "gpt-5"],
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "moonshotai/kimi-k2:free"],
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openai" => &["gpt-4o-mini", "gpt-4.1"],
        "anthropic" => &["claude-sonnet-4-20250514", "claude-3-5-sonnet-20241022"],
        "google" => &["gemini-2.5-flash", "gemini-2.5-pro"],
        "gemini-web" => &["gemini-web-pro"],
        _ => &[],
    }
}
