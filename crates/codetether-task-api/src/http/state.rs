//! Request-state access to caller-owned task services.
use codetether_bus_core::AgentBus;
use codetether_task_queue::KnativeTaskQueue;
use std::sync::Arc;
/// Shared queue and bus needed by task HTTP handlers.
/// # Implementors
/// Applications expose the services already held by their request state.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Process-local task queue.
    fn queue(&self) -> &KnativeTaskQueue;
    /// Shared agent event bus.
    fn bus(&self) -> &Arc<AgentBus>;
}
