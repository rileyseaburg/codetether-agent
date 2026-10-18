//! Accumulated identity and argument fragments for one tool call.

/// Accumulates one tool call's identity and argument deltas during streaming.
/// # Examples
/// ```
/// let tool = codetether_stream_outcome::ToolAccumulator { id: "id".into(), name: "read".into(), arguments: "{}".into() };
/// assert_eq!(tool.name, "read");
/// ```
#[derive(Default)]
pub struct ToolAccumulator {
    /// Provider tool-call identifier.
    pub id: String,
    /// Resolved tool name or the provisional placeholder.
    pub name: String,
    /// Accumulated argument JSON fragments.
    pub arguments: String,
}
