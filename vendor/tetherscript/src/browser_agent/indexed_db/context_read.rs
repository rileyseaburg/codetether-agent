//! Browser context IndexedDB read APIs.

use super::{indexed_db_origin, IndexedDbRecord};
use crate::browser_agent::context::BrowserContext;

impl BrowserContext {
    /// Return a cloned IndexedDB-like value for an origin.
    pub fn indexed_db_get(
        &self,
        origin_or_url: &str,
        database: &str,
        object_store: &str,
        key: &str,
    ) -> Option<String> {
        let origin = indexed_db_origin(origin_or_url);
        self.state
            .borrow()
            .indexed_db
            .get(&origin, database, object_store, key)
            .map(str::to_string)
    }

    /// List IndexedDB-like records for one origin in deterministic order.
    pub fn indexed_db_records(&self, origin_or_url: &str) -> Vec<IndexedDbRecord> {
        self.state
            .borrow()
            .indexed_db
            .list_origin(&indexed_db_origin(origin_or_url))
    }
}
