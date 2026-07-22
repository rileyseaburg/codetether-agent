//! Per-provider failover candidate ladders for the session router.
//!
//! Same-provider candidates are tried before the cross-provider tail in
//! [`known_good_router_candidates`](super::router::known_good_router_candidates),
//! so a failing model first retries a sibling model on its own provider
//! (e.g. Bedrock `fable` → Bedrock Sonnet) instead of jumping straight
//! to another vendor such as `zai`.

/// Same-provider failover model refs for `provider`, in preference order.
pub(in crate::session::helper) fn provider_failover_candidates(provider: &str) -> Vec<String> {
    match provider {
        "openrouter" => vec![
            "openrouter/qwen/qwen3-coder:free".to_string(),
            "openrouter/openai/gpt-oss-120b:free".to_string(),
            "openrouter/google/gemma-3-27b-it:free".to_string(),
            "openrouter/meta-llama/llama-3.3-70b-instruct:free".to_string(),
        ],
        "minimax" => vec!["minimax/MiniMax-M3".to_string()],
        "zai" => vec!["zai/glm-5.1".to_string(), "zai/glm-5".to_string()],
        "glm5" => vec!["glm5/glm-5".to_string()],
        "bedrock" => vec![
            "bedrock/us.anthropic.claude-sonnet-4-6".to_string(),
            "bedrock/us.anthropic.claude-opus-4-6-v1".to_string(),
            "bedrock/us.anthropic.claude-haiku-4-5-20251001-v1:0".to_string(),
        ],
        "github-copilot" | "github-copilot-enterprise" => {
            vec![format!("{provider}/gpt-5-mini")]
        }
        "openai-codex" => crate::provider::openai_codex::model_catalog::chatgpt_models()
            .iter()
            .map(|model| format!("openai-codex/{model}"))
            .collect(),
        "gemini-web" => vec!["gemini-web/gemini-2.5-flash".to_string()],
        "local_cuda" => vec!["local_cuda/qwen3.5-9b".to_string()],
        "google" => vec!["google/gemini-2.5-flash".to_string()],
        "anthropic" => vec!["anthropic/claude-3-5-haiku-latest".to_string()],
        _ => Vec::new(),
    }
}

#[cfg(test)]
#[path = "router_candidates_tests.rs"]
mod tests;
