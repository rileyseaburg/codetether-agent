//! JSON payloads for text observations.
use super::*;

/// Build a `text_chunk` observer payload.
///
/// # Arguments
/// Supply text.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::text_chunk;
/// let payload = text_chunk("hello");
/// assert_eq!(payload["type"], "text_chunk");
/// ```
pub fn text_chunk(text: &str) -> Value {
    json!({ "type": "text_chunk", "text": text })
}

/// Build a `text_complete` observer payload.
///
/// # Arguments
/// Supply text.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::text_complete;
/// let payload = text_complete("hello");
/// assert_eq!(payload["type"], "text_complete");
/// ```
pub fn text_complete(text: &str) -> Value {
    json!({ "type": "text_complete", "text": text })
}

/// Build a `thinking_complete` observer payload.
///
/// # Arguments
/// Supply thought.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::thinking_complete;
/// let payload = thinking_complete("considering");
/// assert_eq!(payload["type"], "thinking_complete");
/// ```
pub fn thinking_complete(thought: &str) -> Value {
    json!({ "type": "thinking_complete", "thought": thought })
}

/// Build a `error` observer payload.
///
/// # Arguments
/// Supply error.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::error;
/// let payload = error("failed");
/// assert_eq!(payload["type"], "error");
/// ```
pub fn error(error: &str) -> Value {
    json!({ "type": "error", "error": error })
}
