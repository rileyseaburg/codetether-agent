//! Page-origin IndexedDB read and delete APIs.

use super::{indexed_db_origin, IndexedDbRecord};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Get a record for the page's current origin.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the page is not attached to a
    /// [`BrowserContext`](crate::browser_agent::BrowserContext).
    pub fn indexed_db_get(
        &self,
        database: &str,
        object_store: &str,
        key: &str,
    ) -> Result<Option<String>, String> {
        let origin = indexed_db_origin(&self.session.url);
        Ok(self
            .indexed_db_state()?
            .borrow()
            .indexed_db
            .get(&origin, database, object_store, key)
            .map(str::to_string))
    }

    /// Delete a record for the page's current origin.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the page is not attached to a
    /// [`BrowserContext`](crate::browser_agent::BrowserContext).
    pub fn indexed_db_delete(
        &mut self,
        database: &str,
        object_store: &str,
        key: &str,
    ) -> Result<bool, String> {
        let origin = indexed_db_origin(&self.session.url);
        Ok(self.indexed_db_state()?.borrow_mut().indexed_db.delete(
            &origin,
            database,
            object_store,
            key,
        ))
    }

    /// List records for the page's current origin.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the page is not attached to a
    /// [`BrowserContext`](crate::browser_agent::BrowserContext).
    pub fn indexed_db_records(&self) -> Result<Vec<IndexedDbRecord>, String> {
        let origin = indexed_db_origin(&self.session.url);
        Ok(self
            .indexed_db_state()?
            .borrow()
            .indexed_db
            .list_origin(&origin))
    }
}
