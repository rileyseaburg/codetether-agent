//! Native bindings for shared workspace finder and embedded editor opening.
mod editor_host;
pub mod scan;
pub mod state;
use crate::tui::app::state::App;
pub use state::{FuzzyEntry, FuzzyFindState};
use std::path::Path;
/// Open the shared workspace finder using native view bindings.
pub fn open_fuzzy_find(app: &mut App, cwd: &Path) {
    codetether_editor::interactive::open_finder(&mut editor_host::EditorView(app), cwd);
}
/// Open the selected finder entry in the embedded editor.
pub fn confirm_fuzzy_find(app: &mut App, cwd: &Path) {
    codetether_editor::interactive::confirm_finder(&mut editor_host::EditorView(app), cwd);
}
/// Open a path using the workspace-relative editor label.
pub fn open_path_in_editor(app: &mut App, cwd: &Path, path: &Path) {
    codetether_editor::interactive::open_path(&mut editor_host::EditorView(app), cwd, path);
}
pub(crate) fn open_argument(app: &mut App, cwd: &Path, arg: &str) {
    codetether_editor::interactive::open_argument(&mut editor_host::EditorView(app), cwd, arg);
}

/// Handle a finder key using the shared editor workflow.
pub(crate) fn handle_key(app: &mut App, cwd: &Path, key: crossterm::event::KeyEvent) -> bool {
    codetether_editor::interactive::handle_finder_key(&mut editor_host::EditorView(app), cwd, key)
}
