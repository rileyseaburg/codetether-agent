//! Parse the server's `resync-required` control event and decide the action.
//!
//! See `docs/transport-phase1-wire-contract.md` section 5. On receipt the client
//! drops its cursor, performs a cold resync, and adopts the new epoch.

use serde::Deserialize;

/// Why the server refused to replay and demanded a cold resync.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResyncReason {
    /// The presented `Last-Event-ID` epoch no longer matches the server epoch.
    EpochMismatch,
    /// The presented seq fell below the replay window's lowest retained seq.
    WindowExceeded,
}

/// Parsed payload of a `resync-required` event.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_stream::resync::{parse_resync, ResyncReason};
///
/// let json = r#"{"reason":"window_exceeded","head_seq":120,"epoch":"e2"}"#;
/// let r = parse_resync(json).unwrap();
/// assert_eq!(r.reason, ResyncReason::WindowExceeded);
/// assert_eq!(r.epoch, "e2");
/// assert_eq!(r.head_seq, 120);
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Resync {
    /// Why replay was refused.
    pub reason: ResyncReason,
    /// Current head sequence the server is at.
    pub head_seq: u64,
    /// The epoch the client must adopt going forward.
    pub epoch: String,
}

/// Parse a `resync-required` data payload.
///
/// # Errors
///
/// Returns an error if the JSON is malformed or fields are missing.
pub fn parse_resync(data: &str) -> Result<Resync, serde_json::Error> {
    serde_json::from_str(data)
}
#[cfg(test)]
#[path = "resync_tests.rs"]
mod tests;
