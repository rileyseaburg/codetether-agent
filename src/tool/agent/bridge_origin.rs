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

impl AgentOrigin {
    pub fn lan_peer() -> Self {
        Self::LanPeer {
            transport: "a2a-mdns",
        }
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Self::LanPeer { .. })
    }

    /// Parent session label for sorting and local rows.
    pub fn parent(&self) -> Option<&str> {
        match self {
            Self::Local { parent, .. } => parent.as_deref(),
            Self::LanPeer { .. } => None,
        }
    }

    /// Model selector for local children; peers have none.
    pub fn model_id(&self) -> Option<&str> {
        match self {
            Self::Local { model_id, .. } => model_id.as_deref(),
            Self::LanPeer { .. } => None,
        }
    }

    /// `"tool-agent"` or `"LAN peer"`.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Local { .. } => "tool-agent",
            Self::LanPeer { .. } => "LAN peer",
        }
    }

    /// `"← parent · model"` for local children, `"via transport"` for peers.
    pub fn lineage(&self) -> String {
        match self {
            Self::Local { parent, model_id } => format!(
                "← {} · {}",
                parent.as_deref().unwrap_or("main"),
                model_id.as_deref().unwrap_or("default model")
            ),
            Self::LanPeer { transport } => format!("via {transport}"),
        }
    }
}
