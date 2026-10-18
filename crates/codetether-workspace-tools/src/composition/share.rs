use crate::Registry;
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_share::SwarmShareTool;
use std::sync::Arc;
/// Add a result-sharing tool and share ownership of the finished registry.
/// # Arguments
/// * `registry` — Catalog to extend.
/// * `store`, `producer_id` — Shared result store and publishing identity.
/// # Returns
/// An Arc owning the registry and its new result-sharing tool.
/// # Examples
/// ```no_run
/// # fn extend<R: codetether_workspace_tools::Registry>(registry: R) {
/// let _registry = codetether_workspace_tools::composition::with_shared(registry, codetether_swarm_core::result_store::ResultStore::new_arc(), "worker".into());
/// # }
/// ```
pub fn with_shared<R: Registry>(
    mut registry: R,
    store: Arc<ResultStore>,
    producer_id: String,
) -> Arc<R> {
    registry.register(Arc::new(SwarmShareTool::new(store, producer_id)));
    Arc::new(registry)
}
