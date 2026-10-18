//! Direct-worker registry composition without recursive coordination tools.
use super::{Factory, with_provider, with_shared};
use crate::Capability;
use codetether_provider_core::Provider;
use codetether_swarm_core::result_store::ResultStore;
use std::{path::Path, sync::Arc};
/// Construct a direct-worker catalog with shared result publishing.
/// # Arguments
/// * `read_only`, `verification`, `expects_changes` — Task capability flags.
/// * `workspace` — Assigned working directory.
/// * `store`, `producer_id` — Shared results and publisher identity.
/// * `provider`, `model` — Completion backend and model.
/// # Returns
/// Shared ownership of the catalog with result publishing installed.
/// # Examples
/// ```no_run
/// # fn make<H: codetether_workspace_tools::composition::Factory>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _registry = codetether_workspace_tools::composition::direct::shared::<H>(true, false, false, std::path::Path::new("."), codetether_swarm_core::result_store::ResultStore::new_arc(), "worker".into(), provider, "model".into());
/// # }
/// ```
pub fn shared<H: Factory>(
    read_only: bool,
    verification: bool,
    expects_changes: bool,
    workspace: &Path,
    store: Arc<ResultStore>,
    producer_id: String,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<H::Registry> {
    let capability = Capability::from_task_flags(read_only, verification, expects_changes);
    let registry = with_provider::<H>(provider, model, workspace, capability);
    with_shared(registry, store, producer_id)
}
/// Construct a direct-worker catalog from its capability flags.
/// # Arguments
/// * `read_only`, `verification` — Worker capability flags.
/// * `workspace` — Assigned working directory.
/// * `provider`, `model` — Completion backend and model.
/// # Returns
/// Shared ownership of the scoped catalog.
/// # Examples
/// ```no_run
/// # fn make<H: codetether_workspace_tools::composition::Factory>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _registry = codetether_workspace_tools::composition::direct::standard::<H>(true, false, std::path::Path::new("."), provider, "model".into());
/// # }
/// ```
pub fn standard<H: Factory>(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<H::Registry> {
    let capability = Capability::from_flags(read_only, verification);
    Arc::new(with_provider::<H>(provider, model, workspace, capability))
}
