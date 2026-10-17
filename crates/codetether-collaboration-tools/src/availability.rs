//! Result of ensuring a durable child is memory-resident.

use codetether_tool_core::ToolResult;

/// Durable residency outcome without transferring ownership of live agent state.
///
/// # Examples
/// ```
/// let outcome=codetether_collaboration_tools::EnsureOpen::Missing;
/// assert!(matches!(outcome,codetether_collaboration_tools::EnsureOpen::Missing));
/// ```
pub enum EnsureOpen {
    /// The child is resident; `resumed` reports whether this call loaded it.
    Ready {
        /// Canonical durable child session ID.
        agent_id: String,
        /// Human-readable child name.
        name: String,
        /// Whether this call restored a dormant child.
        resumed: bool,
    },
    /// No durable child matched the requested identity and owner.
    Missing,
    /// Residency capacity could not be made available safely.
    Rejected(ToolResult),
}
