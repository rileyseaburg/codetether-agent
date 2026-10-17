//! Default and environment-supplied worker registration capabilities.

//! Worker capability and interface helpers.

/// Default A2A control-plane URL used by worker callers.
pub const DEFAULT_A2A_SERVER_URL: &str = "https://api.codetether.run";

const BASE_WORKER_CAPABILITIES: &[&str] = &[
    "forage", "ralph", "swarm", "rlm", "a2a", "mcp", "grpc", "grpc-web", "jsonrpc",
];
mod environment;
mod interfaces;
use environment::{append_env_capabilities, is_knative, push_capability};
pub use interfaces::advertised_interfaces;

/// Returns the capability list advertised during worker registration.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{worker_capabilities};
/// assert!(worker_capabilities().iter().any(|value|value=="forage"));
/// ```
pub fn worker_capabilities() -> Vec<String> {
    let mut capabilities = BASE_WORKER_CAPABILITIES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if is_knative() {
        push_capability(&mut capabilities, "knative");
    }
    append_env_capabilities(&mut capabilities, "CODETETHER_WORKER_CAPABILITIES");
    append_env_capabilities(&mut capabilities, "A2A_WORKER_CAPABILITIES");
    capabilities
}
#[cfg(test)]
mod tests;
