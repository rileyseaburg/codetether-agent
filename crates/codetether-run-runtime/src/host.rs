//! Native state and service bindings for ordinary CLI runs.
use codetether_bus_core::AgentBus;
use codetether_provider_core::Usage;
use codetether_run_policy::SessionPolicy;
use codetether_run_resume::{RunSession, SessionSource};
use std::{path::PathBuf, sync::Arc};
/// Additional session fields needed after selection and before execution.
/// # Implementors
/// Native sessions retain their existing metadata and bus storage.
pub trait Session: RunSession + SessionSource + SessionPolicy + 'static {
    /// Set the workspace knowledge snapshot path.
    fn set_knowledge_snapshot(&mut self, path: Option<PathBuf>);
    /// Attach the process bus to this session.
    fn attach_bus(&mut self, bus: Arc<AgentBus>);
    /// Read accumulated provider usage for final output.
    fn usage(&self) -> &Usage;
}
/// Service initialization owned by the native application.
/// # Implementors
/// Install the process bus and start the configured event sink in order.
pub trait Host {
    /// Native session satisfying selection, policy and prompt contracts.
    type Session: Session;
    /// Create the bus, install it globally, then start its event sink.
    fn create_bus() -> Arc<AgentBus>;
}
