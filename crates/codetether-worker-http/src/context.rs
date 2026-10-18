//! Attach initialized worker resources to their HTTP-facing state.
use crate::WorkerServerState;
use codetether_a2a_worker_core::initialization::WorkerContext;
use codetether_bus_core::AgentBus;
use std::sync::Arc;

impl WorkerServerState {
    /// Publish the worker identity, heartbeat state and bus to HTTP handlers.
    /// # Arguments
    /// * `context` - Initialized worker resources retained by the worker loop.
    /// # Returns
    /// Unit after attaching all three resources in startup order.
    /// # Examples
    /// ```no_run
    /// # async fn example<A, C, P>(
    /// # context: &codetether_a2a_worker_core::initialization::WorkerContext<A, C, codetether_bus_core::AgentBus, P>) {
    /// let state = codetether_worker_http::WorkerServerState::new();
    /// state.attach_context(context).await;
    /// # }
    /// ```
    pub async fn attach_context<A, C, P>(&self, context: &WorkerContext<A, C, AgentBus, P>) {
        self.set_worker_id(context.worker_id.clone()).await;
        self.set_heartbeat_state(Arc::new(context.heartbeat_state.clone()))
            .await;
        self.set_bus(context.bus.clone()).await;
    }
}
