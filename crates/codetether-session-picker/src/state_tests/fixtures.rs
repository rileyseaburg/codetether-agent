//! Shared test fixtures for session picker tests.

use crate::SessionPickerState;
use chrono::Utc;
use codetether_session_metadata::SessionSummary;

/// Build a minimal [`SessionSummary`] for tests.
pub fn summary(id: &str, title: &str) -> SessionSummary {
    SessionSummary {
        id: id.to_string(),
        title: Some(title.to_string()),
        message_count: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        agent: "default".to_string(),
        directory: None,
    }
}

/// Build an [`SessionPickerState`] preloaded with `sessions` and `filter`.
pub fn state_with(sessions: Vec<SessionSummary>, filter: &str) -> SessionPickerState {
    let mut state = SessionPickerState::default();
    state.sessions = sessions;
    state.session_filter = filter.to_string();
    state
}
