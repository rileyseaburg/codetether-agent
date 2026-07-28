//! Runtime-result application for injected realtime events.

use crate::browser_agent::page::BrowserPage;
use crate::browser_js::BrowserJsResult;

impl BrowserPage {
    pub(crate) fn apply_realtime_result(
        &mut self,
        action: &str,
        dispatch: impl FnOnce(&mut BrowserPage) -> Result<BrowserJsResult, String>,
    ) -> Result<(), String> {
        self.enforce_resource_limits(action)?;
        self.sync_context_state_into_session();
        let checkpoint = self.event_checkpoint();
        let result = dispatch(self);
        self.apply_runtime_result(checkpoint, action, result)?;
        self.sync_context_state_from_session();
        Ok(())
    }
}
