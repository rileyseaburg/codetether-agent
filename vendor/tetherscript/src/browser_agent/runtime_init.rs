//! Runtime initialization helpers for browser pages.

use crate::browser_agent::page::BrowserPage;
use crate::browser_js::BrowserJsRuntime;

impl BrowserPage {
    pub(crate) fn runtime_mut(&mut self) -> Result<&mut BrowserJsRuntime, String> {
        let refresh = match self.runtime.as_ref() {
            Some(runtime) => runtime.html() != self.session.html,
            None => true,
        };
        let state = self.session.browser_js_state();
        if refresh {
            self.runtime = Some(BrowserJsRuntime::new(&self.session.html, state)?);
        } else if let Some(runtime) = self.runtime.as_mut() {
            runtime.apply_state(state);
        }
        self.runtime
            .as_mut()
            .ok_or_else(|| "browser page runtime was not initialized".into())
    }
}
