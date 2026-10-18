//! Provider and model resolution for swarm workers.

use super::model_resolution::{resolve_default, resolve_requested};
use anyhow::Result;

/// Original and resolved model identities for worker result diagnostics.
/// # Examples
/// ```
/// let model = codetether_swarm_tool::model_selection::resolve(Some("openai/gpt-4o"), &["openai"]).unwrap();
/// assert_eq!(model.resolved_provider, "openai");
/// ```
#[derive(Clone, Debug, serde::Serialize)]
pub struct ModelSelection {
    /// Original reference, if explicitly set or inherited.
    pub requested_model: Option<String>,
    /// Available provider selected for execution.
    pub resolved_provider: String,
    /// Model identifier passed to the provider.
    pub resolved_model: String,
}

/// Resolve a model reference against the available providers.
/// # Arguments
/// * `requested` — Explicit or inherited model reference.
/// * `providers` — Available provider identifiers.
/// # Returns
/// Original and resolved model diagnostics.
/// # Errors
/// Fails when there is no available provider or an explicit provider is unavailable.
/// # Examples
/// ```
/// assert!(codetether_swarm_tool::model_selection::resolve(None, &[]).is_err());
/// ```
pub fn resolve(requested: Option<&str>, providers: &[&str]) -> Result<ModelSelection> {
    let (provider, model) = match requested {
        Some(model_ref) => resolve_requested(model_ref, providers)?,
        None => resolve_default(providers)?,
    };
    Ok(ModelSelection {
        requested_model: requested.map(str::to_string),
        resolved_provider: provider,
        resolved_model: model,
    })
}
