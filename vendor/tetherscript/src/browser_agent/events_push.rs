//! Internal helpers that append page event summaries.

use crate::browser_agent::events::{PageEventKind, PageEventSummary};
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::{ConsoleEvent, NetworkEvent};

impl BrowserPage {
    pub(crate) fn push_console_event(&mut self, event: &ConsoleEvent) {
        self.push_event(PageEventKind::Console, &event.level, &event.message);
    }

    pub(crate) fn push_network_event(&mut self, event: &NetworkEvent) {
        let status = event
            .status
            .map_or_else(|| "none".into(), |status| status.to_string());
        let mut message = format!("{} {}", event.url, status);
        if let Some(route) = event.route_result.as_deref() {
            message.push(' ');
            message.push_str(route);
        }
        self.push_event(PageEventKind::Network, &event.method, &message);
    }

    pub(crate) fn push_event(&mut self, kind: PageEventKind, action: &str, message: &str) {
        self.event_log.push(PageEventSummary {
            sequence: self.event_log.len() as u64,
            kind,
            action: action.into(),
            message: message.into(),
        });
    }
}
