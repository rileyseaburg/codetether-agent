//! Agent identity profile lookup.
//!
//! Maps agent names to codenames, collaboration styles, and signature moves
//! using keyword matching with FNV-1a hash fallback.

use crate::session::Session;

/// A spawned sub-agent with its own independent LLM session.
#[allow(dead_code)]
pub struct SpawnedAgent {
    /// User-facing name (e.g. "planner", "coder")
    pub name: String,
    /// System instructions for this agent
    pub instructions: String,
    /// Name of the agent that spawned this one; `None` means the main chat
    /// (a top-level sibling). Forms the agent tree used by the header rail.
    pub parent: Option<String>,
    /// Nesting depth: 0 for children of main, up to
    /// [`crate::tui::app::state::agent_tree::MAX_AGENT_DEPTH`].
    pub depth: u8,
    /// Independent conversation session
    pub session: Session,
    /// Model selected or last used when this agent was created.
    pub model_id: Option<String>,
    /// Whether this agent is currently processing a message
    pub is_processing: bool,
}
pub use codetether_agent_identity::agent_profile;
