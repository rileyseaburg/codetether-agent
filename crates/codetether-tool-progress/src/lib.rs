//! Scoped delivery of live tool-output chunks to registered sinks.
//!
//! [register] installs a callback until its [Guard] is dropped. [emit] delivers
//! output to that callback outside the registry lock. Callers choose the event
//! transport; the registry owns only tool identity and callback lifetime.
mod emission;
mod registration;
mod registry;
pub use emission::emit;
pub use registration::{Guard, register};

/// One tool-output fragment, ready for the caller's event transport.
///
/// # Examples
/// ```rust
/// let chunk = codetether_tool_progress::Chunk {
///     tool_call_id: "call-1".into(), name: "bash".into(),
///     stream: "stdout".into(), chunk: "hello".into(),
/// };
/// assert_eq!(chunk.chunk, "hello");
/// ```
pub struct Chunk {
    /// Registered invocation identifier.
    pub tool_call_id: String,
    /// Tool name supplied at registration.
    pub name: String,
    /// Output stream name, such as stdout or stderr.
    pub stream: String,
    /// Output text for this fragment.
    pub chunk: String,
}
