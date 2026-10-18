//! Existing provider runtime configuration bindings for settings cycles.
use crate::provider::{bedrock, openai_codex, openrouter};
use codetether_settings_policy::cycles::{Cycle, Runtime};
pub(super) struct ProviderRuntime;
impl Runtime for ProviderRuntime {
    fn current(setting: Cycle) -> Option<String> {
        match setting {
            Cycle::BedrockTier => bedrock::runtime_config::service_tier(),
            Cycle::BedrockEffort => bedrock::runtime_config::thinking_effort(),
            Cycle::CodexEffort => openai_codex::runtime_config::thinking_level(),
            Cycle::OpenRouterEffort => openrouter::runtime_config::thinking_level(),
        }
    }
    fn set(setting: Cycle, value: Option<String>) {
        match setting {
            Cycle::BedrockTier => bedrock::runtime_config::set_service_tier(value),
            Cycle::BedrockEffort => bedrock::runtime_config::set_thinking_effort(value),
            Cycle::CodexEffort => openai_codex::runtime_config::set_thinking_level(value),
            Cycle::OpenRouterEffort => openrouter::runtime_config::set_thinking_level(value),
        }
    }
    fn levels(setting: Cycle, model: &str) -> Vec<&'static str> {
        match setting {
            Cycle::CodexEffort => openai_codex::reasoning_catalog::supported_levels(model),
            Cycle::OpenRouterEffort => openrouter::reasoning_levels::LEVELS.to_vec(),
            Cycle::BedrockTier | Cycle::BedrockEffort => Vec::new(),
        }
    }
}
