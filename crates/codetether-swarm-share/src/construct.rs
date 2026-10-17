//! construct for shared swarm results.
use super::*;

impl SwarmShareTool {
    /// Construct a tool bound to a shared store and producer identity.
    ///
    /// # Arguments
    /// * `store` - Result store shared by participating agents.
    /// * `producer_id` - Identity recorded on published results.
    ///
    /// # Returns
    /// The configured tool.
    ///
    /// # Examples
    /// ```rust
    /// let store = codetether_swarm_core::result_store::ResultStore::new_arc();
    /// let _tool = codetether_swarm_share::SwarmShareTool::new(store, "worker".into());
    /// ```
    pub fn new(store: Arc<ResultStore>, producer_id: String) -> Self {
        Self { store, producer_id }
    }

    /// Create with default ResultStore and empty producer ID (for registry registration)
    ///
    /// # Returns
    /// A tool with an independent empty store.
    ///
    /// # Examples
    /// ```rust
    /// let _tool = codetether_swarm_share::SwarmShareTool::with_defaults();
    /// ```
    pub fn with_defaults() -> Self {
        Self {
            store: ResultStore::new_arc(),
            producer_id: String::new(),
        }
    }
}

impl Default for SwarmShareTool {
    fn default() -> Self {
        Self::with_defaults()
    }
}
