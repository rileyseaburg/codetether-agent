//! Browser context IndexedDB write APIs.

use super::indexed_db_origin;
use crate::browser_agent::context::BrowserContext;

impl BrowserContext {
    /// Insert or replace an IndexedDB-like record for an origin.
    pub fn indexed_db_put(
        &mut self,
        origin_or_url: &str,
        database: &str,
        object_store: &str,
        key: &str,
        value: &str,
    ) {
        let origin = indexed_db_origin(origin_or_url);
        self.state
            .borrow_mut()
            .indexed_db
            .put(&origin, database, object_store, key, value);
    }

    /// Delete an IndexedDB-like record for an origin.
    pub fn indexed_db_delete(
        &mut self,
        origin_or_url: &str,
        database: &str,
        object_store: &str,
        key: &str,
    ) -> bool {
        let origin = indexed_db_origin(origin_or_url);
        self.state
            .borrow_mut()
            .indexed_db
            .delete(&origin, database, object_store, key)
    }
}
