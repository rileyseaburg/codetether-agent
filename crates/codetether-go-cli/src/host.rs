//! Native provider and bus startup on the existing Ralph host.
use codetether_bus_core::AgentBus;
use std::sync::Arc;
/// Additional CLI services supplied by an existing Ralph execution host.
/// # Implementors
/// Reuse the application Ralph host; its registry must also implement the
/// autochat ModelRegistry contract when calling the CLI executor.
#[async_trait::async_trait]
pub trait Host: codetether_go_ralph::Host {
    /// Load the existing provider registry.
    /// # Errors
    /// Returns provider initialization errors.
    async fn load_registry() -> anyhow::Result<Arc<Self::Registry>>;
    /// Create the training bus and start its configured sink.
    fn create_bus() -> Arc<AgentBus>;
}
