//! with bus task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Create with an agent bus attached.
    ///
    /// Constructs a new store using [`Self::new`] and attaches an [`AgentBus`]
    /// before the store is shared. The bus allows gRPC task activity to be
    /// mirrored into the broader agent-event system while preserving the same
    /// task storage and update-stream behavior as [`Self::new`].
    ///
    /// # Parameters
    ///
    /// * `card` - Agent metadata served by this store.
    /// * `bus` - Shared event bus used to publish or coordinate task activity
    ///   outside the gRPC transport.
    ///
    /// # Returns
    ///
    /// A reference-counted store with the supplied bus installed.
    ///
    /// # Panics
    ///
    /// Panics only if the freshly created [`Arc`] is not uniquely owned before
    /// installing the bus. Under normal control flow this cannot happen because
    /// the [`Arc`] has just been returned by [`Self::new`] and has not yet been
    /// cloned.
    pub fn with_bus(card: local::AgentCard, bus: Arc<AgentBus>) -> Arc<Self> {
        let mut store = Self::new(card);
        Arc::get_mut(&mut store)
            .expect("fresh Arc must be uniquely owned")
            .bus = Some(bus);
        store
    }
}
