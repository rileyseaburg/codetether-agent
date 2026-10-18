use super::Factory;
use crate::{Capability, finish};
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc};
/// Construct and scope a provider-aware application catalog.
/// # Arguments
/// * `provider`, `model` — Completion provider and selected model.
/// * `root`, `capability` — Workspace and allowed filesystem operations.
/// # Returns
/// The populated, workspace-scoped registry.
/// # Examples
/// ```no_run
/// # fn make<H: codetether_workspace_tools::composition::Factory>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _registry = codetether_workspace_tools::composition::with_provider::<H>(provider, "model".into(), std::path::Path::new("."), codetether_workspace_tools::Capability::ReadOnly);
/// # }
/// ```
pub fn with_provider<H: Factory>(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
) -> H::Registry {
    finish::<H>(H::provider(provider, model), root, capability)
}
/// Construct and scope the application's default catalog.
/// # Arguments
/// * `root`, `capability` — Workspace and allowed filesystem operations.
/// # Returns
/// The populated, workspace-scoped registry.
/// # Examples
/// ```no_run
/// # fn make<H: codetether_workspace_tools::composition::Factory>() {
/// let _registry = codetether_workspace_tools::composition::with_defaults::<H>(std::path::Path::new("."), codetether_workspace_tools::Capability::ReadOnly);
/// # }
/// ```
pub fn with_defaults<H: Factory>(root: &Path, capability: Capability) -> H::Registry {
    finish::<H>(H::defaults(), root, capability)
}
