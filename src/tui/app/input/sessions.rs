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
use std::sync::Arc;

use crossterm::event::KeyModifiers;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "goal_autostart.rs"]
pub(crate) mod goal_autostart;

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
pub(super) async fn handle_enter_sessions(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    super::continue_command::select_session(app, cwd, slot, registry, worker_bridge, runtime).await;
}
