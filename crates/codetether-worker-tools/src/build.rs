//! Build a workspace-scoped registry under the worker approval policy.
use crate::Host;
use crate::{register_mutating_tools, register_safe_tools};
use codetether_a2a_worker_core::execution_policy::AutoApprove;
use codetether_provider_core::Provider;
use std::path::Path;
use std::sync::Arc;
/// Build a workspace-scoped registry under the worker approval policy.
///
/// # Arguments
/// * `provider` — Provider for model-backed tools.
/// * `model` — Resolved model identifier.
/// * `auto_approve` — Only `All` includes mutating tools.
/// * `workspace_dir` — Worker workspace root.
/// * `completion_callback` — Optional background completion notification.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// A registry containing safe tools, permitted mutating tools and fallback.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn create_filtered_registry<H: Host>(
    provider: Arc<dyn Provider>,
    model: String,
    auto_approve: AutoApprove,
    workspace_dir: &Path,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> H::Registry {
    let mut registry = H::new_registry();
    let root_path = workspace_dir.to_path_buf();
    register_safe_tools::<H>(&mut registry, workspace_dir, &root_path);
    if matches!(auto_approve, AutoApprove::All) {
        register_mutating_tools::<H>(
            &mut registry,
            provider,
            model,
            root_path,
            completion_callback,
        );
    }
    H::register(
        &mut registry,
        Arc::new(codetether_tool_core::InvalidTool::new()),
    );
    registry
}
