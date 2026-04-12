//! Session-view character input handler.
//!
//! Appends printable characters to the session filter
//! string and loads the selected session on Enter.
//!
//! # Examples
//!
//! ```ignore
//! handle_sessions_char(&mut app, modifiers, 'a');
//! ```

use std::path::Path;

use crossterm::event::KeyModifiers;

use crate::session::Session;
use crate::tui::app::state::App;

/// Append a character to the session filter string.
///
/// Ignores Ctrl/Alt modified keys so they can be handled
/// by other keybind logic.
///
/// # Examples
///
/// ```ignore
/// handle_sessions_char(&mut app, modifiers, 'a');
/// ```
pub fn handle_sessions_char(app: &mut App, modifiers: KeyModifiers, c: char) {
    if !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT) {
        app.state.session_filter_push(c);
    }
}

/// Load the currently selected session on Enter.
///
/// Reads the filtered session list, finds the original
/// index, and loads the session by ID.
///
/// # Examples
///
/// ```ignore
/// handle_enter_sessions(&mut app, cwd, &mut session).await;
/// ```
pub(super) async fn handle_enter_sessions(app: &mut App, cwd: &Path, session: &mut Session) {
    let session_id = app
        .state
        .filtered_sessions()
        .get(app.state.selected_session)
        .map(|(orig_idx, _)| app.state.sessions[*orig_idx].id.clone());
    if let Some(session_id) = session_id {
        crate::tui::app::codex_sessions::load_selected_session(app, cwd, session, &session_id)
            .await;
    }
}
