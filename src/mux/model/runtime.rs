//! Semantic state reported by the TUI that owns a mux session.

use serde::{Deserialize, Serialize};

/// Durable identity and live turn state for the active mux TUI.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct MuxRuntimeStatus {
    /// Durable session identifier used only for internal routing.
    pub session_id: String,
    /// Human-readable session title used in agent-facing output.
    pub session_title: String,
    /// Whether the session currently accepts steering input.
    pub processing: bool,
    /// Number of durable transcript messages observed by the owning TUI.
    #[serde(default)]
    pub message_count: usize,
    /// Tool currently executing, when known.
    pub current_tool: Option<String>,
    /// Whether text is waiting for an explicit interaction.
    #[serde(default)]
    pub needs_interaction: bool,
    /// Whether the owning TUI has detected a stalled turn.
    #[serde(default)]
    pub lagging: bool,
}
