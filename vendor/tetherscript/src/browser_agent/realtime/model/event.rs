//! Realtime lifecycle event model.

use super::kind::RealtimeKind;

/// Deterministic realtime lifecycle event category.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RealtimeEventKind;
///
/// let event = RealtimeEventKind::Open;
/// assert!(matches!(event, RealtimeEventKind::Open));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealtimeEventKind {
    /// Constructor-created connection entry.
    Connect,
    /// Connection opened.
    Open,
    /// WebSocket outbound message.
    Send,
    /// Host-injected inbound message.
    Receive,
    /// Connection closed.
    Close,
    /// Connection failed.
    Error,
}

/// Metadata for one deterministic realtime lifecycle event.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RealtimeEvent, RealtimeEventKind, RealtimeKind};
///
/// let event = RealtimeEvent {
///     sequence: 0,
///     connection_id: 1,
///     event: RealtimeEventKind::Open,
///     connection_kind: RealtimeKind::WebSocket,
///     url: "ws://example.test".into(),
///     ready_state: 1,
///     data: None,
///     code: None,
///     reason: None,
///     retry_ms: None,
/// };
/// assert_eq!(event.ready_state, 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeEvent {
    /// Monotonic page-local realtime sequence number.
    pub sequence: u64,
    /// Page-local connection id.
    pub connection_id: u64,
    /// Lifecycle event category.
    pub event: RealtimeEventKind,
    /// Connection family.
    pub connection_kind: RealtimeKind,
    /// Connection URL.
    pub url: String,
    /// Ready state observed when the event was logged.
    pub ready_state: i64,
    /// Payload for send/receive events.
    pub data: Option<String>,
    /// Close code when supplied.
    pub code: Option<u16>,
    /// Failure or close reason when supplied.
    pub reason: Option<String>,
    /// EventSource retry delay when known.
    pub retry_ms: Option<u64>,
}
