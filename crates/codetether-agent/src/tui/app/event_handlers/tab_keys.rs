//! Tab / Shift+Tab dispatch for chat-view agent focus cycling.
//!
//! Keeps the keycode match in [`super::keybinds`] terse: Tab accepts a
//! slash suggestion or cycles agent focus forward; Shift+Tab (BackTab)
//! cycles backward. Both are no-ops outside the chat view.

use crossterm::event::KeyCode;

use crate::tui::app::{navigation::Navigation, state::App};
#[cfg(test)]
use crate::tui::models::ViewMode;

/// Route a Tab or BackTab keypress to the right focus action.
pub(super) fn dispatch(app: &mut App, code: KeyCode) {
    codetether_tui_navigation::tab::dispatch(&mut Navigation(app), code);
}

#[cfg(test)]
#[path = "tab_keys_tests.rs"]
mod tests;
