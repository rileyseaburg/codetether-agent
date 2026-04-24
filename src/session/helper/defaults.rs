//! Default model selection per provider.
//!
//! When a session has no explicit model configured we fall back to a
//! sensible default for the chosen provider. These defaults only apply when
//! the caller has not set [`SessionMetadata::model`](super::super::SessionMetadata::model).

/// Return the default model ID for a given provider name.
pub(crate) fn default_model_for_provider(provider: &str) -> String {
    match provider {
        "moonshotai" => "kimi-k2.5".to_string(),
        "anthropic" => "claude-sonnet-4-20250514".to_string(),
        "minimax" => "MiniMax-M2.5".to_string(),
        "openai" => "gpt-4o".to_string(),
        "openai-codex" => "gpt-5.4".to_string(),
        "google" => "gemini-2.5-pro".to_string(),
        "local_cuda" => std::env::var("LOCAL_CUDA_MODEL")
            .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
            .unwrap_or_else(|_| "qwen3.5-9b".to_string()),
        "zhipuai" | "zai" | "zai-api" => "glm-5".to_string(),
        // OpenRouter uses model IDs like "z-ai/glm-5".
        "openrouter" => "z-ai/glm-5".to_string(),
        "novita" => "Qwen/Qwen3.5-35B-A3B".to_string(),
        "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
        _ => "gpt-4o".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::default_model_for_provider;

    #[test]
    fn openai_defaults_to_gpt4o() {
        assert_eq!(default_model_for_provider("openai"), "gpt-4o");
    }

    #[test]
    fn openai_codex_defaults_to_gpt_5_4() {
        assert_eq!(default_model_for_provider("openai-codex"), "gpt-5.4");
    }

    #[test]
    fn unknown_provider_defaults_to_gpt4o() {
        assert_eq!(default_model_for_provider("does-not-exist"), "gpt-4o");
    }

    #[test]
    fn zhipuai_aliases_resolve_to_glm() {
        assert_eq!(default_model_for_provider("zai"), "glm-5");
        assert_eq!(default_model_for_provider("zhipuai"), "glm-5");
        assert_eq!(default_model_for_provider("zai-api"), "glm-5");
    }
}
