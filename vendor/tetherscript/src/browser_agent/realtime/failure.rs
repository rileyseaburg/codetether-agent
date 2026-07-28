//! Page APIs for deterministic realtime failures.

use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Fail a WebSocket connection and dispatch `error` then `close`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::{BrowserPage, RealtimeEventKind};
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    /// page.eval_js("WebSocket('ws://agent');").unwrap();
    /// let id = page.realtime_connections().unwrap()[0].id;
    /// page.fail_websocket_connection(id, "refused").unwrap();
    /// assert_eq!(page.realtime_events().unwrap().last().unwrap().event, RealtimeEventKind::Close);
    /// ```
    pub fn fail_websocket_connection(
        &mut self,
        connection_id: u64,
        reason: impl AsRef<str>,
    ) -> Result<(), String> {
        self.apply_realtime_result("page.websocket_error", |page| {
            page.runtime_mut()?
                .fail_websocket_connection(connection_id, reason.as_ref())
        })
    }

    /// Fail an EventSource connection and record retry metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    /// page.eval_js("EventSource('/events');").unwrap();
    /// let id = page.realtime_connections().unwrap()[0].id;
    /// page.fail_event_source_connection(id, "offline", Some(2500)).unwrap();
    /// assert_eq!(page.realtime_connections().unwrap()[0].retry_ms, Some(2500));
    /// ```
    pub fn fail_event_source_connection(
        &mut self,
        connection_id: u64,
        reason: impl AsRef<str>,
        retry_ms: Option<u64>,
    ) -> Result<(), String> {
        self.apply_realtime_result("page.event_source_error", |page| {
            page.runtime_mut()?.fail_event_source_connection(
                connection_id,
                reason.as_ref(),
                retry_ms,
            )
        })
    }
}
