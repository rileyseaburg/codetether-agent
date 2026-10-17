//! Auto-chat preferred models for provider.
use super::*;

pub(super) fn preferred_models_for_provider(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M3", "MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" => &["glm-5.3", "glm-5.2", "glm-5.1", "glm-5"],
        "openai-codex" => codetether_provider_core::codex::model_catalog::current_models(),
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "z-ai/glm-4.7:free"],
        _ => &[],
    }
}
