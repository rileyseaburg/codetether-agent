//! Realtime connection snapshot model.

use super::kind::RealtimeKind;

/// Snapshot of one realtime connection known to a page runtime.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RealtimeConnection, RealtimeKind};
///
/// let connection = RealtimeConnection {
///     id: 1,
///     kind: RealtimeKind::WebSocket,
///     url: "ws://example.test".into(),
///     ready_state: 1,
///     retry_ms: None,
/// };
/// assert_eq!(connection.ready_state, 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeConnection {
    /// Stable page-local connection id.
    pub id: u64,
    /// Connection family.
    pub kind: RealtimeKind,
    /// URL supplied to the JS constructor.
    pub url: String,
    /// Browser-compatible numeric readyState.
    pub ready_state: i64,
    /// EventSource retry delay when known.
    pub retry_ms: Option<u64>,
}
