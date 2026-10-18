//! Local and peer origins and their relevant identity fields.
//! Where a snapshotted agent came from, and the identity that origin implies.

/// The origin of an [`AgentSnapshot`](super::AgentSnapshot).
///
/// Local children have a parent and a model; LAN peers have a transport.
/// Rendering code matches on this instead of reading faked-in fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentOrigin {
    /// Spawned in this process through the `agent` tool.
    Local {
        parent: Option<String>,
        model_id: Option<String>,
    },
    /// A discovered, authenticated A2A peer on the LAN.
    LanPeer { transport: &'static str },
}
