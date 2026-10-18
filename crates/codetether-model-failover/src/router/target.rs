//! Rule-based provider availability selection.
use super::known_good_router_candidates;

/// Choose the first available model in the failover ladder.
/// # Arguments
/// * `registry` — Active provider availability.
/// * `selected_provider`, `current_model` — Provider and failed model identity.
/// # Returns
/// A provider/model pair, or None when no fallback provider is registered.
/// # Examples
/// ```
/// struct Empty;
/// impl codetether_model_failover::router::Registry for Empty {
///     fn available(&self, _: &str) -> bool { false }
/// }
/// assert!(codetether_model_failover::router::choose_router_target(&Empty, "openai", "failed").is_none());
/// ```
pub fn choose_router_target(
    registry: &impl super::Registry,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    let current_provider = selected_provider.to_ascii_lowercase();

    for candidate in known_good_router_candidates(selected_provider, current_model) {
        let (provider_name, model_name) =
            codetether_provider_core::routing::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.available(provider_name) {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    if current_provider != "zai" && registry.available("zai") {
        return Some(("zai".to_string(), "glm-5".to_string()));
    }
    if current_provider != "glm5" && registry.available("glm5") {
        return Some(("glm5".to_string(), "glm-5".to_string()));
    }
    if current_provider != "openrouter" && registry.available("openrouter") {
        return Some((
            "openrouter".to_string(),
            "openai/gpt-oss-120b:free".to_string(),
        ));
    }

    None
}
