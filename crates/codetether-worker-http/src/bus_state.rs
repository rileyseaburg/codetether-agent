//! Worker state bus state operations.
use super::*;
impl WorkerServerState {
    /// Attach the agent bus (can be called after construction from the worker task)
    ///
    /// # Arguments
    /// bus is the shared in-process agent bus.
    ///
    /// # Returns
    /// Completes after attaching the bus.
    ///
    /// # Examples
    /// ```
    /// async fn attach(state: &codetether_worker_http::WorkerServerState, bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
    /// state.set_bus(bus).await;
    /// }
    /// ```
    pub async fn set_bus(&self, bus: Arc<AgentBus>) {
        *self.bus.lock().await = Some(bus);
    }
}
