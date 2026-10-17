//! Shared Codex model capabilities and request-alias parsing.

pub mod model_catalog;
pub mod model_discovery;
mod parse_model_thinking_level;
mod parse_service_tier_model_alias;
pub mod reasoning_catalog;
mod service_tier;
pub mod service_tier_catalog;
pub mod thinking_level;
pub use parse_model_thinking_level::parse_model_thinking_level;
pub use parse_service_tier_model_alias::parse_service_tier_model_alias;
pub use service_tier::CodexServiceTier;
pub use thinking_level::ThinkingLevel;

/// Return the upstream model ID after removing supported request options.
///
/// # Arguments
/// * `model` - A model ID, optionally with an effort or service-tier alias.
/// # Returns
/// The model ID submitted to the provider.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_core::codex::resolved_model_identity("gpt-5.4-fast:high"), "gpt-5.4");
/// ```
pub fn resolved_model_identity(model: &str) -> String {
    let (base, _) = parse_model_thinking_level(model);
    parse_service_tier_model_alias(&base).0
}

pub mod model_policy;
