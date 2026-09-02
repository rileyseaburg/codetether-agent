//! In-memory session-list refresh for the just-finished turn.
//!
//! A full [`refresh_sessions`](super::refresh_sessions) rescans every session
//! file on disk plus the Codex archive; on large workspaces that takes seconds
//! and used to run inline on the UI task after *every* reply. The active
//! session is the only entry that changed, so update it in place instead.

use crate::session::{Session, SessionSummary};
use crate::tui::app::state::App;

/// Move the finished session to the top of the cached list with fresh
/// metadata, inserting it when it was not listed yet.
pub fn upsert_active_session(app: &mut App, session: &Session) {
    let summary = summary_of(session);
    let sessions = &mut app.state.sessions;
    if let Some(index) = sessions.iter().position(|item| item.id == summary.id) {
        sessions.remove(index);
    }
    sessions.insert(0, summary);
    app.state.selected_session = 0;
}

fn summary_of(session: &Session) -> SessionSummary {
    SessionSummary {
        id: session.id.clone(),
        title: session.title.clone(),
        created_at: session.created_at,
        updated_at: session.updated_at,
        message_count: session.messages.len(),
        agent: session.agent.clone(),
        directory: session.metadata.directory.clone(),
    }
}

#[cfg(test)]
#[path = "upsert_tests.rs"]
mod tests;
