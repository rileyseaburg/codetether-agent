//! Short-name aliases for Bedrock-hosted OpenAI GPT models.
//!
//! GPT-5.6 Sol/Terra/Luna were added to the Bedrock catalog in Codex
//! rust-v0.143.0 (openai/codex PR #30285). All three support `max`
//! reasoning effort.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::aliases_openai::resolve_openai_alias;
//!
//! assert_eq!(resolve_openai_alias("gpt-5.6-sol"), Some("openai.gpt-5.6-sol"));
//! assert_eq!(resolve_openai_alias("gpt-5.5"), Some("openai.gpt-5.5"));
//! assert_eq!(resolve_openai_alias("claude-opus-4-7"), None);
//! ```

/// Resolve a short OpenAI GPT alias to the full Bedrock model ID.
///
/// Returns `None` when the input is not a Bedrock-hosted OpenAI GPT alias
/// so the caller can fall through to other alias families.
pub fn resolve_openai_alias(model: &str) -> Option<&'static str> {
    match model {
        "gpt-5.6-sol" => Some("openai.gpt-5.6-sol"),
        "gpt-5.6-terra" => Some("openai.gpt-5.6-terra"),
        "gpt-5.6-luna" => Some("openai.gpt-5.6-luna"),
        "gpt-5.5" => Some("openai.gpt-5.5"),
        "gpt-5.4" => Some("openai.gpt-5.4"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_openai_alias;
    use crate::provider::bedrock::BedrockProvider;

    #[test]
    fn short_aliases_map_to_openai_prefix() {
        assert_eq!(
            resolve_openai_alias("gpt-5.6-sol"),
            Some("openai.gpt-5.6-sol")
        );
        assert_eq!(
            resolve_openai_alias("gpt-5.6-terra"),
            Some("openai.gpt-5.6-terra")
        );
        assert_eq!(
            resolve_openai_alias("gpt-5.6-luna"),
            Some("openai.gpt-5.6-luna")
        );
        assert_eq!(resolve_openai_alias("gpt-5.5"), Some("openai.gpt-5.5"));
        assert_eq!(resolve_openai_alias("gpt-5.4"), Some("openai.gpt-5.4"));
    }

    #[test]
    fn full_ids_pass_through_resolve_model_id() {
        assert_eq!(
            BedrockProvider::resolve_model_id("gpt-5.6-sol"),
            "openai.gpt-5.6-sol"
        );
        assert_eq!(
            BedrockProvider::resolve_model_id("openai.gpt-5.6-sol"),
            "openai.gpt-5.6-sol"
        );
    }

    #[test]
    fn non_openai_returns_none() {
        assert_eq!(resolve_openai_alias("claude-opus-4-7"), None);
        assert_eq!(resolve_openai_alias("nova-pro"), None);
    }
}
