//! Lazy registry loading and explicit-model precedence for swarm tasks.
use crate::execution_policy::{Registry, choose_provider_for_tier, default_model_for_provider};
use std::{future::Future, sync::Arc};

/// Resolve a swarm model using explicit input or configured provider defaults.
///
/// # Arguments
/// * `explicit_model` — Caller-provided model, preserved when nonblank.
/// * `model_tier` — Optional provider/default preference tier.
/// * `load` — Lazy registry loader, called only when no explicit model is usable.
/// # Returns
/// A model reference, or `None` when registry loading fails or has no providers.
/// # Examples
/// ```no_run
/// # async fn example<R, F, L>(load: F)
/// # where R: codetether_a2a_worker_core::execution_policy::Registry,
/// # F: FnOnce() -> L, L: std::future::Future<Output = anyhow::Result<std::sync::Arc<R>>> {
/// let model = codetether_a2a_worker_core::execution_policy::model_selection::resolve_swarm_model(
///     Some("provider/model".into()), None, load,
/// ).await;
/// assert_eq!(model.as_deref(), Some("provider/model"));
/// # }
/// ```
pub async fn resolve_swarm_model<R, F, L>(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
    load: F,
) -> Option<String>
where
    R: Registry,
    F: FnOnce() -> L,
    L: Future<Output = anyhow::Result<Arc<R>>>,
{
    if let Some(model) = explicit_model.filter(|value| !value.trim().is_empty()) {
        return Some(model);
    }
    let registry = load().await.ok()?;
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
