//! Tab / Shift+Tab dispatch for chat-view agent focus cycling.
//!
//! Keeps the keycode match in [`super::keybinds`] terse: Tab accepts a
//! slash suggestion or cycles agent focus forward; Shift+Tab (BackTab)
//! cycles backward. Both are no-ops outside the chat view.

use crossterm::event::KeyCode;

use crate::tui::app::{navigation as nav, state::App};
use crate::tui::models::ViewMode;

/// Route a Tab or BackTab keypress to the right focus action.
pub(super) fn dispatch(app: &mut App, code: KeyCode) {
    if app.state.view_mode != ViewMode::Chat {
        return;
    }
    match code {
        KeyCode::BackTab => nav::cycle_agent_focus_back(app),
        _ => nav::handle_tab(app),
    }
}
