//! Register the background go tool with an optional completion callback.
use crate::Host;
use std::sync::Arc;
/// Register the background go tool with an optional completion callback.
///
/// # Arguments
/// * `registry` — Registry receiving the background go tool.
/// * `completion_callback` — Optional callback passed to background execution.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_go_tool<H: Host>(
    registry: &mut H::Registry,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    H::register(registry, H::go(completion_callback));
}
