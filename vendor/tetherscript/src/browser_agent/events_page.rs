//! Public page event views.

use crate::browser_agent::events::{PageErrorEvent, PageEventKind, PageEventSummary};
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::{ConsoleEvent, NetworkEvent};

impl BrowserPage {
    /// Return console events captured from page JavaScript.
    pub fn console_events(&self) -> &[ConsoleEvent] {
        &self.session.console
    }

    /// Return network events captured from page JavaScript.
    pub fn network_events(&self) -> &[NetworkEvent] {
        &self.session.network
    }

    /// Return JavaScript page errors recorded at page action boundaries.
    pub fn page_errors(&self) -> Vec<PageErrorEvent> {
        self.event_log
            .iter()
            .filter(|event| event.kind == PageEventKind::PageError)
            .map(|event| PageErrorEvent {
                action: event.action.clone(),
                message: event.message.clone(),
            })
            .collect()
    }

    /// Return deterministic summaries for console, pageerror, and network events.
    pub fn event_log(&self) -> &[PageEventSummary] {
        &self.event_log
    }
}
