//! Result of ensuring a durable child is memory-resident.

use crate::tool::ToolResult;

pub(crate) enum EnsureOpen {
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
