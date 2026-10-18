//! Historical swarm model defaults, independent of session defaults.

/// Choose the historical default model for direct swarm workers.
/// # Arguments
/// * `provider` — Provider identifier.
/// # Returns
/// A model identifier; unknown providers use MiniMax-M3.
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_tool::model_defaults::default_model_for_provider("zai"), "glm-5.3");
/// ```

pub fn default_model_for_provider(provider: &str) -> String {
    match provider {
        "moonshotai" => "kimi-k2.5".to_string(),
        "anthropic" => "claude-sonnet-4-20250514".to_string(),
        "bedrock" => "us.anthropic.claude-sonnet-4-20250514-v1:0".to_string(),
        "openai" => "gpt-4o".to_string(),
        "google" => "gemini-2.5-pro".to_string(),
        "zhipuai" | "zai" => "glm-5.3".to_string(),
        "openrouter" => "z-ai/glm-5".to_string(),
        "novita" => "qwen/qwen3.5-35b-a3b".to_string(),
        "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
        "openai-codex" => "gpt-5.5".to_string(),
        _ => "MiniMax-M3".to_string(),
    }
}
