//! Resolve requested model identities through the caller-owned registry.
use crate::Registry;
use anyhow::{Result, anyhow};
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Default router model. Override with `--router-model` on the CLI or the
/// `CODETETHER_SEARCH_ROUTER_MODEL` env var.
pub const DEFAULT_ROUTER_MODEL: &str = "zai/glm-5.3";

/// Resolve a `provider/model` reference into `(provider, model_id)`.
///
/// Falls back to the first registered provider when the reference has no
/// `/`, which keeps the command usable when a user just types `glm-5.3`.
///
/// # Errors
///
/// Returns an error when the registry is empty or the named provider is
/// missing.
/// # Arguments
/// * `registry` - Existing provider registry; its order determines fallback selection.
/// * `model_ref` - Provider/model pair or a provider-local model name.
/// # Returns
/// The chosen shared provider and its local model identifier.
/// # Examples
/// ```rust,no_run
/// # fn example(registry: &impl codetether_search_router::Registry)->anyhow::Result<()> {
/// let (_, model)=codetether_search_router::model::resolve_router_model(registry,"zai/glm-5.3")?;
/// assert_eq!(model,"glm-5.3");
/// # Ok(()) }
/// ```
pub fn resolve_router_model(
    registry: &impl Registry,
    model_ref: &str,
) -> Result<(Arc<dyn Provider>, String)> {
    if let Some((prov_name, model_name)) = model_ref.split_once('/') {
        if let Some(prov) = registry.get(prov_name) {
            return Ok((prov, model_name.to_string()));
        }
        // Requested provider is not configured. Rather than hard-fail — which
        // makes the `search` tool unusable for any user who doesn't have the
        // default `zai` provider set up — fall back to the first registered
        // provider and log a warning so the caller can see what happened.
        let first = registry.list().first().copied().ok_or_else(|| {
            anyhow!("router provider '{prov_name}' not available and no providers configured")
        })?;
        let prov = registry
            .get(first)
            .ok_or_else(|| anyhow!("provider '{first}' missing"))?;
        tracing::warn!(target: "codetether_agent::search::model",
            requested = prov_name,
            requested_model = model_name,
            fallback_provider = first,
            "router provider unavailable; falling back to first registered provider"
        );
        return Ok((prov, model_name.to_string()));
    }
    let first = registry
        .list()
        .first()
        .copied()
        .ok_or_else(|| anyhow!("no providers configured"))?;
    let prov = registry
        .get(first)
        .ok_or_else(|| anyhow!("provider '{first}' missing"))?;
    Ok((prov, model_ref.to_string()))
}
