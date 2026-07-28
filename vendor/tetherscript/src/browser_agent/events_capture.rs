//! Internal page event capture helpers.

use crate::browser_agent::events::{PageErrorEvent, PageEventCheckpoint, PageEventKind};
use crate::browser_agent::page::BrowserPage;
use crate::browser_js::BrowserJsResult;
use crate::js::JsValue;

impl BrowserPage {
    pub(crate) fn event_checkpoint(&self) -> PageEventCheckpoint {
        PageEventCheckpoint::new(self.session.console.len(), self.session.network.len())
    }

    pub(crate) fn apply_runtime_result(
        &mut self,
        checkpoint: PageEventCheckpoint,
        action: &str,
        result: Result<BrowserJsResult, String>,
    ) -> Result<JsValue, String> {
        let value = result.and_then(|result| self.session.apply_browser_js_result(result, action));
        match value {
            Ok(value) => {
                self.record_event_deltas(checkpoint);
                Ok(value)
            }
            Err(message) => Err(self.record_page_error(action, message)),
        }
    }

    fn record_event_deltas(&mut self, checkpoint: PageEventCheckpoint) {
        let console = self.session.console[checkpoint.console_len..].to_vec();
        let network = self.session.network[checkpoint.network_len..].to_vec();
        for event in console {
            self.push_console_event(&event);
        }
        for event in network {
            self.push_network_event(&event);
        }
    }

    fn record_page_error(&mut self, action: &str, message: String) -> String {
        let error = PageErrorEvent {
            action: action.into(),
            message,
        };
        self.push_event(PageEventKind::PageError, &error.action, &error.message);
        error.message
    }
}
