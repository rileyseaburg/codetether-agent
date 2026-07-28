//! Page IndexedDB context-state access.

use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub(crate) fn indexed_db_state(&self) -> Result<&SharedContextState, String> {
        self.context_state
            .as_ref()
            .ok_or_else(|| "indexedDB requires a page attached to BrowserContext".into())
    }
}
