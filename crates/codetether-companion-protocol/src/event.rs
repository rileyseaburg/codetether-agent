use serde::{Deserialize, Serialize};

/// SSE kinds; unknown kinds fail closed, matching the Swift decoder.
///
/// ```
/// use codetether_companion_protocol::EventKind;
/// assert!(matches!(EventKind::Delta, EventKind::Delta));
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// Full current state on subscribe/reconnect, not replayed deltas.
    Snapshot,
    /// A newly accepted frame starts analysis.
    Capture,
    /// Incremental analysis text.
    Delta,
    /// Final current analysis.
    Done,
    /// Bounded, redacted analysis error.
    Error,
    /// Terminal session stop.
    Stopped,
}

/// JSON after the SSE `data:` prefix; contains neither pixels nor tokens.
///
/// ```
/// use codetether_companion_protocol::{EventKind, ScreenEvent};
/// let event = ScreenEvent { kind: EventKind::Snapshot, seq: 0,
///     text: Some(String::new()), status: Some("waiting".into()), captured_at: None };
/// assert_eq!(event.seq, 0);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ScreenEvent {
    /// Event discriminator, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: EventKind,
    /// Nonnegative sequence; ordering/reconnect is the consumer's duty.
    pub seq: u64,
    /// Analysis text; omitted when not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// String retained for forward-compatible status display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Capture time, absent before the first capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
}
