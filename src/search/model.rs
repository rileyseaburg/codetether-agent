//! Resolve a router `provider/model` string against the live `ProviderRegistry`.
//!
//! Lets the search command use `codetether models` discovery rather than a
//! hardcoded provider binding.

use std::sync::Arc;

use anyhow::{Result, anyhow};

use crate::provider::{Provider, ProviderRegistry};

/// Default router model. Override with `--router-model` on the CLI or the
/// `CODETETHER_SEARCH_ROUTER_MODEL` env var.
pub const DEFAULT_ROUTER_MODEL: &str = "zai/glm-5.1";

/// Resolve a `provider/model` reference into `(provider, model_id)`.
///
/// Falls back to the first registered provider when the reference has no
/// `/`, which keeps the command usable when a user just types `glm-5.1`.
///
/// # Errors
///
/// Returns an error when the registry is empty or the named provider is
/// missing.
pub fn resolve_router_model(
    registry: &ProviderRegistry,
    model_ref: &str,
) -> Result<(Arc<dyn Provider>, String)> {
    if let Some((prov_name, model_name)) = model_ref.split_once('/') {
        let prov = registry
            .get(prov_name)
            .ok_or_else(|| anyhow!("router provider '{prov_name}' not available"))?;
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
