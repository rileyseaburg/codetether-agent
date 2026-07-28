//! Page APIs for deterministic realtime connections.

use super::convert::connection_from_js;
use super::{RealtimeConnection, RealtimeOutboundMessage};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return realtime connections created by page JavaScript.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    /// page.eval_js("WebSocket('ws://agent');").unwrap();
    /// let connections = page.realtime_connections().unwrap();
    /// assert_eq!(connections[0].url, "ws://agent");
    /// ```
    pub fn realtime_connections(&mut self) -> Result<Vec<RealtimeConnection>, String> {
        Ok(self
            .runtime_mut()?
            .realtime_connections()
            .into_iter()
            .map(connection_from_js)
            .collect())
    }

    /// Return outbound WebSocket messages recorded by the host.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    /// page.eval_js("let ws=WebSocket('ws://agent'); ws.onopen=function(){ ws.send('ping'); };").unwrap();
    /// let messages = page.realtime_outbound_messages().unwrap();
    /// assert_eq!(messages[0].data, "ping");
    /// ```
    pub fn realtime_outbound_messages(&mut self) -> Result<Vec<RealtimeOutboundMessage>, String> {
        Ok(self
            .runtime_mut()?
            .realtime_outbound()
            .into_iter()
            .map(|message| RealtimeOutboundMessage {
                connection_id: message.connection_id,
                url: message.url,
                data: message.data,
            })
            .collect())
    }

    /// Inject an inbound WebSocket `message` event.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<p id='out'></p>");
    /// page.eval_js("let ws=WebSocket('ws://agent'); ws.onmessage=function(e){ document.getElementById('out').textContent=e.data; };").unwrap();
    /// let id = page.realtime_connections().unwrap()[0].id;
    /// page.inject_websocket_message(id, "hello").unwrap();
    /// assert!(page.session.html.contains("hello"));
    /// ```
    pub fn inject_websocket_message(
        &mut self,
        connection_id: u64,
        data: impl AsRef<str>,
    ) -> Result<(), String> {
        self.apply_realtime_result("page.websocket_message", |page| {
            page.runtime_mut()?
                .inject_websocket_message(connection_id, data.as_ref())
        })
    }

    /// Inject an inbound EventSource `message` event.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://rt", "<p id='out'></p>");
    /// page.eval_js("let es=EventSource('/events'); es.onmessage=function(e){ document.getElementById('out').textContent=e.data; };").unwrap();
    /// let id = page.realtime_connections().unwrap()[0].id;
    /// page.inject_event_source_message(id, "stream").unwrap();
    /// assert!(page.session.html.contains("stream"));
    /// ```
    pub fn inject_event_source_message(
        &mut self,
        connection_id: u64,
        data: impl AsRef<str>,
    ) -> Result<(), String> {
        self.apply_realtime_result("page.event_source_message", |page| {
            page.runtime_mut()?
                .inject_event_source_message(connection_id, data.as_ref())
        })
    }
}
