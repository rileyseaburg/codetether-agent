//! Small model records used by the browser JS realtime host.

use super::BrowserJsRealtimeKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BrowserJsRealtimeEventKind {
    Connect,
    Open,
    Send,
    Receive,
    Close,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserJsRealtimeEvent {
    pub(crate) sequence: u64,
    pub(crate) connection_id: u64,
    pub(crate) event: BrowserJsRealtimeEventKind,
    pub(crate) connection_kind: BrowserJsRealtimeKind,
    pub(crate) url: String,
    pub(crate) ready_state: i64,
    pub(crate) data: Option<String>,
    pub(crate) code: Option<u16>,
    pub(crate) reason: Option<String>,
    pub(crate) retry_ms: Option<u64>,
}
