//! Swarm model resolution helpers.
//!
//! This module picks an execution model for swarm tasks when the queue payload
//! does not explicitly provide one.
//!
//! # Examples
//!
//! ```ignore
//! let model = resolve_swarm_model(None, Some("fast")).await;
//! ```

use crate::provider::ProviderRegistry;

use super::model_defaults::default_model_for_provider;
use super::model_preferences::choose_provider_for_tier;

/// Resolves the effective swarm model from explicit or inferred settings.
///
/// Explicit models win. Otherwise the worker loads the provider registry and
/// chooses a preferred provider for the requested tier.
///
/// # Examples
///
/// ```ignore
/// let model = resolve_swarm_model(Some("zai/glm-5.1".to_string()), None).await;
/// assert_eq!(model.as_deref(), Some("zai/glm-5.1"));
/// ```
pub(super) async fn resolve_swarm_model(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> Option<String> {
    if let Some(model) = explicit_model.filter(|value| !value.trim().is_empty()) {
        return Some(model);
    }
    let registry = ProviderRegistry::shared_from_vault().await.ok()?;
    let providers = registry.list();
    if providers.is_empty() {
        return None;
    }
    let provider = choose_provider_for_tier(providers.as_slice(), model_tier);
    Some(format!(
        "{provider}/{}",
        default_model_for_provider(provider, model_tier)
    ))
}
