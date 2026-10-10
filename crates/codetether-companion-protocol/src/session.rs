use serde::{Deserialize, Serialize};

/// Owner's setup; the relay validates provider/model, prompt, and interval.
///
/// ```
/// use codetether_companion_protocol::SessionInput;
/// let input = SessionInput {
///     model: "provider/vision".into(), prompt: "Describe the screen".into(),
///     interval_seconds: 30,
/// };
/// assert_eq!(input.interval_seconds, 30);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SessionInput {
    /// Explicit provider/model identifier for read-only analysis.
    pub model: String,
    /// Owner instructions; deliberately excluded from Debug output.
    pub prompt: String,
    /// Relay accepts 15 through 300 seconds.
    pub interval_seconds: u16,
}

/// Owner-only receipt containing a short-lived pairing capability.
///
/// ```
/// use codetether_companion_protocol::SessionReceipt;
/// let receipt = SessionReceipt {
///     id: "session-id".into(), code: "AABBCCDDEEFF".into(),
///     pair_expires_at: "2026-01-01T00:05:00.000Z".into(),
///     expires_at: "2026-01-01T01:00:00.000Z".into(), interval_seconds: 30,
/// };
/// assert_eq!(receipt.code.len(), 12);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SessionReceipt {
    /// Session UUID shared with the paired device.
    pub id: String,
    /// One-use pairing code. Do not log or persist this receipt.
    pub code: String,
    /// ISO-8601 pairing deadline, preserved verbatim on the wire.
    pub pair_expires_at: String,
    /// ISO-8601 session deadline.
    pub expires_at: String,
    /// Accepted periodic interval in seconds.
    pub interval_seconds: u16,
}
