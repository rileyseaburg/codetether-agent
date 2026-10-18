//! Native service boundaries used by the relay runtime.
use anyhow::Result;
use codetether_autochat_core::model_rotation::ModelRegistry;
use codetether_bus_core::AgentBus;
use codetether_session_events::SessionResult;
use std::sync::Arc;
/// Native provider registry and external event sink setup.
/// # Implementors
/// Applications retain credential loading and S3 configuration.
#[async_trait::async_trait(?Send)]
pub trait Host {
    /// Initialized model registry implementation.
    type Registry: ModelRegistry;
    /// Application session retained for each relay specialist.
    type Session: Session;
    /// Start the configured bus sink, if available.
    fn spawn_bus_sink(bus: Arc<AgentBus>);
    /// Load providers from the application's existing source.
    /// # Errors
    /// Returns provider initialization errors.
    async fn load_registry() -> Result<Arc<Self::Registry>>;
}
/// Session operations required by a relay specialist.
/// # Implementors
/// Native sessions preserve their own policy and persistence behavior.
#[async_trait::async_trait(?Send)]
pub trait Session: Sized {
    /// Create a session with the application defaults.
    /// # Errors
    /// Returns native session creation errors.
    async fn new() -> Result<Self>;
    /// Set the model, identity, bus and initial system instructions in order.
    fn configure(&mut self, model: String, name: String, bus: Arc<AgentBus>, instructions: String);
    /// Execute a correlated handoff prompt.
    /// # Errors
    /// Returns native prompt errors.
    async fn prompt(&mut self, prompt: &str) -> Result<SessionResult>;
}
