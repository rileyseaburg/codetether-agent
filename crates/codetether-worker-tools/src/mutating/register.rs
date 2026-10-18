//! Register the complete state-changing worker tool set.
use super::{register_edit_tools, register_go_tool, register_model_tools, register_task_tools};
use crate::Host;
use codetether_provider_core::Provider;
use std::path::PathBuf;
use std::sync::Arc;
/// Register the complete state-changing worker tool set.
///
/// # Arguments
/// * `registry` — Registry receiving mutating tool groups in order.
/// * `provider` — Provider for RLM and Ralph tools.
/// * `model` — Resolved model identifier.
/// * `root_path` — Workspace root for editing tools.
/// * `completion_callback` — Optional background completion notification.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_mutating_tools<H: Host>(
    registry: &mut H::Registry,
    provider: Arc<dyn Provider>,
    model: String,
    root_path: PathBuf,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    register_edit_tools::<H>(registry, root_path);
    register_task_tools::<H>(registry);
    register_model_tools::<H>(registry, provider, model);
    register_go_tool::<H>(registry, completion_callback);
}
