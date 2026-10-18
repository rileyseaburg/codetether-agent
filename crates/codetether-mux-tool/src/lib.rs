//! Manager tool dispatch and safety policy for mux sessions.
//!
//! [execute] parses tool input and routes observation, steering and lifecycle
//! actions. Lifecycle changes retain their running/unknown-state checks.
//! [Host] supplies the native durable-session output reader.
mod args;
mod execute;
mod lifecycle;
mod lifecycle_safety;
mod operations;
mod schema;
pub use execute::execute;
pub use schema::parameters;
/// Stable tool identifier.
pub const ID: &str = "mux_control";
/// Human-readable tool name.
pub const NAME: &str = "Mux Control";
/// Tool description retained by the native registry.
pub const DESCRIPTION: &str = "Operate CodeTether mux sessions directly. Use watch instead of wait_agent. Steer and interact return server-owned delivery acceptance; call watch or status for subsequent execution state.";
/// Native session projection reader used by the read action.
/// # Implementors
/// Bind the existing application session store without changing output selection.
#[async_trait::async_trait]
pub trait Host: Send + Sync {
    /// Read retained agent output for the supplied mux route.
    /// # Errors
    /// Returns native projection/storage errors.
    async fn read_agent_output(name: &str) -> anyhow::Result<Option<String>>;
}
