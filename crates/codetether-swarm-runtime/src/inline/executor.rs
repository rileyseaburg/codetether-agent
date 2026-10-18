//! Native executor construction used by inline worker execution.
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig, SwarmEvent, SwarmResult};
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc;
/// Native executor operations needed by the inline worker loop.
/// # Implementors
/// Adapters preserve existing builder and execution behavior.
pub trait Executor: Sized + Send + Sync {
    /// Native agent bus.
    type Bus: Send + Sync;
    /// Construct an executor from the worker's resolved configuration.
    fn new(config: SwarmConfig) -> Self;
    /// Attach shared cancellation and pause control.
    fn with_control(self, control: ControlHandle) -> Self;
    /// Attach the existing agent event bus.
    fn with_bus(self, bus: Arc<Self::Bus>) -> Self;
    /// Attach the worker event-forwarding channel.
    fn with_event_tx(self, tx: mpsc::Sender<SwarmEvent>) -> Self;
    /// Execute the swarm inline.
    /// # Errors
    /// Returns native planning or execution failures.
    fn execute(
        &self,
        prompt: &str,
        strategy: DecompositionStrategy,
    ) -> impl Future<Output = anyhow::Result<SwarmResult>> + Send;
}
