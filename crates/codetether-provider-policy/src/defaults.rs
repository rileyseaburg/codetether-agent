//! Default model selection per provider.
//!
//! When a session has no explicit model configured we fall back to a
//! sensible default for the chosen provider. These defaults only apply when
//! the caller has not set an explicit model.

/// Return the default model ID for a given provider name.
/// # Arguments
/// * `provider` - Selected provider identifier.
/// # Returns
/// The established fallback model, honoring local CUDA environment overrides.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_policy::default_model_for_provider("openai"), "gpt-4o");
/// ```
pub fn default_model_for_provider(provider: &str) -> String {
    match provider {
        "moonshotai" => "kimi-k2.5".to_string(),
        "anthropic" => "claude-sonnet-4-20250514".to_string(),
        "minimax" => "MiniMax-M3".to_string(),
        "openai" => "gpt-4o".to_string(),
        "openai-codex" => "gpt-5.5".to_string(),
        "google" => "gemini-2.5-pro".to_string(),
        "local_cuda" => std::env::var("LOCAL_CUDA_MODEL")
            .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
            .unwrap_or_else(|_| "qwen3.5-9b".to_string()),
        "zhipuai" | "zai" | "zai-api" => "glm-5".to_string(),
        // OpenRouter uses model IDs like "z-ai/glm-5".
        "openrouter" => "z-ai/glm-5".to_string(),
        "novita" => "qwen/qwen3.5-35b-a3b".to_string(),
        "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
        _ => "gpt-4o".to_string(),
    }
}
