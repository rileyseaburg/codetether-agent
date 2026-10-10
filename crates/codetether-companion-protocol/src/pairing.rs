use serde::{Deserialize, Serialize};

/// Device pairing input; pairing does not grant local capture consent.
///
/// ```
/// use codetether_companion_protocol::PairRequest;
/// let request = PairRequest { code: "AABBCCDDEEFF".into() };
/// assert_eq!(request.code.len(), 12);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct PairRequest {
    /// One-use capability supplied locally by the user; never log it.
    pub code: String,
}

/// Memory-only device capability, not an owner credential.
///
/// ```
/// use codetether_companion_protocol::PairReceipt;
/// let receipt = PairReceipt {
///     id: "session-id".into(), device_token: "test-only-token".into(),
///     interval_seconds: 30, expires_at: "2026-01-01T01:00:00Z".into(),
/// };
/// assert_eq!(receipt.interval_seconds, 30);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct PairReceipt {
    /// Paired session UUID.
    pub id: String,
    /// Device bearer capability; no Debug implementation to prevent logging.
    pub device_token: String,
    /// Minimum periodic interval accepted by the relay.
    pub interval_seconds: u16,
    /// ISO-8601 session deadline, with or without fractional seconds.
    pub expires_at: String,
}
