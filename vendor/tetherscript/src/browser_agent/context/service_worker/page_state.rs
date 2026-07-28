//! Page access to context-scoped service-worker state.

use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub(crate) fn service_worker_state(&self) -> Result<&SharedContextState, String> {
        self.context_state
            .as_ref()
            .ok_or_else(|| "service worker API requires a page attached to BrowserContext".into())
    }
}
