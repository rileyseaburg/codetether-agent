//! Realtime outbound message model.

/// Outbound WebSocket message recorded by the deterministic host.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RealtimeOutboundMessage;
///
/// let message = RealtimeOutboundMessage {
///     connection_id: 1,
///     url: "ws://example.test".into(),
///     data: "ping".into(),
/// };
/// assert_eq!(message.data, "ping");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeOutboundMessage {
    /// Page-local connection id that sent the message.
    pub connection_id: u64,
    /// Connection URL.
    pub url: String,
    /// Stringified payload passed to `send`.
    pub data: String,
}
