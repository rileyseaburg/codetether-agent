//! Forward finder keys through the existing native editor view adapter.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
use std::path::Path;
pub(super) fn handle_fuzzy_find_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    crate::tui::app::fuzzy_find::handle_key(app, cwd, key)
}
