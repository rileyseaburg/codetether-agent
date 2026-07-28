//! Realtime connection family model.

/// Realtime connection family exposed by the deterministic browser host.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RealtimeKind;
///
/// let kind = RealtimeKind::WebSocket;
/// assert!(matches!(kind, RealtimeKind::WebSocket));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealtimeKind {
    /// A `WebSocket(...)` host object.
    WebSocket,
    /// An `EventSource(...)` host object.
    EventSource,
}
