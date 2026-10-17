//! Construct an empty in-memory run map.
use super::*;

impl InMemoryStore {
    /// Create an empty map of runs protected by an asynchronous lock.
    ///
    /// # Returns
    /// A store containing no runs.
    /// # Examples
    /// ```
    /// let store = codetether_ralph_core::store_memory::InMemoryStore::new();
    /// let _ = store;
    /// ```
    pub fn new() -> Self {
        Self {
            runs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}
