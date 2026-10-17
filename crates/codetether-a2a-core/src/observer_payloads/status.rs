//! JSON payloads for status observations.
use super::*;

/// Build a `thinking` observer payload.
///
/// # Arguments
/// This marker takes no arguments.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::thinking;
/// let payload = thinking();
/// assert_eq!(payload["type"], "thinking");
/// ```
pub fn thinking() -> Value {
    json!({ "type": "thinking" })
}

/// Build a `done` observer payload.
///
/// # Arguments
/// This marker takes no arguments.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::done;
/// let payload = done();
/// assert_eq!(payload["type"], "done");
/// ```
pub fn done() -> Value {
    json!({ "type": "done" })
}

/// Build a `ignored` observer payload.
///
/// # Arguments
/// This marker takes no arguments.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::ignored;
/// let payload = ignored();
/// assert_eq!(payload["type"], "ignored");
/// ```
pub fn ignored() -> Value {
    json!({ "type": "ignored" })
}
