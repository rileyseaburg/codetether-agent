//! new task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Create a new task store with the given agent card.
    ///
    /// Initializes an empty, shareable [`GrpcTaskStore`] for a single local
    /// agent. The returned [`Arc`] is intended to be cloned into gRPC handlers,
    /// JSON-RPC adapters, streaming tasks, and other transport glue that needs
    /// access to the same in-process task state.
    ///
    /// # Parameters
    ///
    /// * `card` - Agent metadata served by this store and returned to clients
    ///   that request the local agent card.
    ///
    /// # Returns
    ///
    /// A reference-counted store with no tasks, no push-notification
    /// configuration, no attached [`AgentBus`], and a task-update broadcast
    /// channel sized for 256 queued status updates per receiver.
    ///
    /// # Side effects
    ///
    /// Allocates the concurrent maps and creates a Tokio broadcast channel. It
    /// does not publish network services, spawn background tasks, or emit any
    /// task updates.
    pub fn new(card: local::AgentCard) -> Arc<Self> {
        let (update_tx, _) = broadcast::channel(256);
        Arc::new(Self {
            tasks: DashMap::new(),
            push_configs: DashMap::new(),
            card,
            bus: None,
            update_tx,
        })
    }
}
