//! Application state contract for registry routes.
use crate::ToolRegistry;
/// Read-only access to the shared dynamic tool registry.
/// # Implementors
/// Applications return the registry already held in their request state.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Registry used for discovery, registration and heartbeat.
    fn tool_registry(&self) -> &ToolRegistry;
}
