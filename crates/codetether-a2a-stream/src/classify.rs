//! Event-class classification for the resumable stream protocol.
//!
//! Maps an SSE `event:` type to its delivery class. See
//! `docs/transport-phase1-wire-contract.md` section 2.

/// Delivery class of a stream event.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_stream::classify::{classify, EventClass};
///
/// assert_eq!(classify("task_available"), EventClass::Advisory);
/// assert_eq!(classify("heartbeat"), EventClass::Control);
/// assert_eq!(classify("result"), EventClass::Sequenced);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventClass {
    /// Claim-gated, at-least-once, never sequenced (e.g. `task_available`).
    Advisory,
    /// Liveness/handshake, never sequenced (e.g. `heartbeat`, `connected`).
    Control,
    /// Exactly-once in-order state events; carry an `id:` and move the cursor.
    Sequenced,
}

/// Classify an event type string into its [`EventClass`].
///
/// Unknown types default to [`EventClass::Sequenced`] so new state events are
/// resumed conservatively rather than silently dropped from the cursor.
pub fn classify(event_type: &str) -> EventClass {
    match event_type {
        "task_available" => EventClass::Advisory,
        "heartbeat" | "connected" | "resync-required" => EventClass::Control,
        _ => EventClass::Sequenced,
    }
}
#[cfg(test)]
#[path = "classify_tests.rs"]
mod tests;
