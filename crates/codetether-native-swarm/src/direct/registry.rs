//! Base direct-swarm registry construction.

use super::{Application, ToolRegistry};
use codetether_provider_core::Provider;
use codetether_swarm_core::result_store::ResultStore;
use std::{path::Path, sync::Arc};

/// Build an autonomous registry with the requested read/verification policy.
/// # Arguments
/// Policy flags and workspace bound the supplied provider/model's tools.
/// # Returns
/// The native registry using the application's existing tool catalog.
/// # Examples
/// ```no_run
/// # fn example<T: codetether_application_registry::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let registry = codetether_native_swarm::direct::registry::standard::<T>(
///     true, false, std::path::Path::new("."), provider, "model".into(),
/// );
/// # }
/// ```
pub fn standard<T: Application>(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry<T>> {
    codetether_workspace_tools::composition::direct::standard::<crate::workspace::WorkspaceHost<T>>(
        read_only,
        verification,
        workspace,
        provider,
        model,
    )
}

/// Build an autonomous registry sharing one result store across workers.
/// # Arguments
/// Policy flags, workspace and provider/model configure execution; store and
/// producer identity bind shared findings to the current worker.
/// # Returns
/// A native registry with the same collaboration and restriction policy.
/// # Examples
/// ```no_run
/// # fn example<T: codetether_application_registry::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let registry = codetether_native_swarm::direct::registry::shared::<T>(
///     true, false, false, std::path::Path::new("."),
///     codetether_swarm_core::result_store::ResultStore::new_arc(),
///     "worker".into(), provider, "model".into(),
/// );
/// # }
/// ```
#[allow(clippy::too_many_arguments)]
pub fn shared<T: Application>(
    read_only: bool,
    verification: bool,
    expects_changes: bool,
    workspace: &Path,
    store: Arc<ResultStore>,
    producer_id: String,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry<T>> {
    codetether_workspace_tools::composition::direct::shared::<crate::workspace::WorkspaceHost<T>>(
        read_only,
        verification,
        expects_changes,
        workspace,
        store,
        producer_id,
        provider,
        model,
    )
}
