//! Resolve model aliases and lazily consult the configured reasoning effort.
use super::*;

/// Resolve effort and service-tier aliases with lazy configuration fallback.
///
/// # Arguments
/// * `model` - Requested model and any inline options.
/// * `configured` - Called only when the model has no explicit effort.
/// # Returns
/// The resolved model, supported effort if any, and optional service tier.
/// # Examples
/// ```
/// let (model, _, _) = codetether_provider_core::codex::model_policy::resolve_model_and_reasoning_effort_and_service_tier("gpt-5.5:high", || None);
/// assert_eq!(model, "gpt-5.5");
/// ```
pub fn resolve_model_and_reasoning_effort_and_service_tier(
    model: &str,
    configured: impl FnOnce() -> Option<ThinkingLevel>,
) -> (String, Option<ThinkingLevel>, Option<CodexServiceTier>) {
    let (base_model, level_from_model) = parse_model_thinking_level(model);
    let level = level_from_model.or_else(configured);
    let (base_model, service_tier) = parse_service_tier_model_alias(&base_model);
    if !model_supports_reasoning_effort(&base_model) {
        return (base_model, None, service_tier);
    }
    let supported = reasoning_catalog::supported_levels(&base_model);
    let level = level.filter(|effort| {
        supported.is_empty() || reasoning_catalog::supports(&base_model, effort.as_str())
    });
    (base_model, level, service_tier)
}
