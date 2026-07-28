//! Page-origin IndexedDB write APIs.

use super::indexed_db_origin;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Insert or replace a record for the page's current origin.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the page is not attached to a [`BrowserContext`].
    ///
    /// [`BrowserContext`]: crate::browser_agent::BrowserContext
    pub fn indexed_db_put(
        &mut self,
        database: &str,
        object_store: &str,
        key: &str,
        value: &str,
    ) -> Result<(), String> {
        let origin = indexed_db_origin(&self.session.url);
        let state = self.indexed_db_state()?;
        state
            .borrow_mut()
            .indexed_db
            .put(&origin, database, object_store, key, value);
        Ok(())
    }
}
