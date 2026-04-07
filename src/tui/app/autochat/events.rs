//! Autochat UI event types for the TUI event loop.

use crate::session::SessionEvent;

/// Events emitted by the autochat relay worker thread.
#[derive(Debug)]
pub enum AutochatUiEvent {
    /// Incremental progress update (shown in status bar).
    Progress(String),
    /// System-level message to display in chat.
    SystemMessage(String),
    /// An agent in the relay produced a session event.
    AgentEvent {
        agent_name: String,
        event: Box<SessionEvent>,
    },
    /// Relay completed with a summary and optional OKR linkage.
    Completed {
        summary: String,
        okr_id: Option<String>,
        okr_run_id: Option<String>,
        relay_id: Option<String>,
    },
}
