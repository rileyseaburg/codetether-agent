//! Application bus access for the HTTP event stream.
use codetether_bus_core::AgentBus;
use std::sync::Arc;

/// Application state providing the bus used for event subscriptions.
/// # Implementors
/// Implement this trait on the server's shared request state.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the existing shared agent bus.
    fn bus(&self) -> &Arc<AgentBus>;
}
