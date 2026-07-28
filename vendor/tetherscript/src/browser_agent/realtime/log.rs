//! Page APIs for realtime lifecycle logs.

use super::convert::event_from_js;
use super::RealtimeEvent;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return deterministic realtime lifecycle events in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::{BrowserPage, RealtimeEventKind};
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    /// page.eval_js("WebSocket('ws://agent');").unwrap();
    /// let events = page.realtime_events().unwrap();
    /// assert_eq!(events[0].event, RealtimeEventKind::Connect);
    /// ```
    pub fn realtime_events(&mut self) -> Result<Vec<RealtimeEvent>, String> {
        Ok(self
            .runtime_mut()?
            .realtime_events()
            .into_iter()
            .map(event_from_js)
            .collect())
    }
}
