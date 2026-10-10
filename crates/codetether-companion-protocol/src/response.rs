use serde::{Deserialize, Serialize};

/// Frame acknowledgement (HTTP 202), not a claim that analysis succeeded.
/// ```
/// use codetether_companion_protocol::Accepted;
/// assert!(Accepted { accepted: true }.accepted);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Accepted {
    /// Whether the relay accepted the frame.
    pub accepted: bool,
}

/// Device pause acknowledgement (HTTP 200).
/// ```
/// use codetether_companion_protocol::Paused;
/// assert!(Paused { paused: true }.paused);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Paused {
    /// Relay pause; local capture must stop independently as well.
    pub paused: bool,
}

/// Owner stop acknowledgement (HTTP 200).
/// ```
/// use codetether_companion_protocol::Stopped;
/// assert!(Stopped { stopped: true }.stopped);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Stopped {
    /// Whether the relay stopped the session and revoked capabilities.
    pub stopped: bool,
}

/// Device typed-reply acknowledgement (HTTP 200); the queued reply is cleared.
/// ```
/// use codetether_companion_protocol::Typed;
/// assert!(Typed { typed: true }.typed);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Typed {
    /// Whether the device consumed the reply, typed or refused.
    pub typed: bool,
}

/// Error body; HTTP status is carried separately by the transport.
/// ```
/// use codetether_companion_protocol::ErrorResponse;
/// let body = ErrorResponse { error: "Screen session ended".into() };
/// assert!(!body.error.is_empty());
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ErrorResponse {
    /// Redacted diagnostic, never credentials or screenshot contents.
    pub error: String,
}
