//! Shared bus access for voice HTTP operations.
use codetether_bus_core::AgentBus;
use std::sync::Arc;

/// Application state providing the bus used for voice session notifications.
/// # Implementors
/// Implement this trait on the state passed to the application's HTTP router.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the existing shared agent bus.
    fn bus(&self) -> &Arc<AgentBus>;
}
