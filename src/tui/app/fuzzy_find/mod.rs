//! `/edit` fuzzy file finder overlay.
//!
//! Opening `/edit` with no argument scans the workspace and shows a fuzzy
//! finder; typing filters by subsequence match, arrows move the selection,
//! Enter opens the highlighted file in the editor, and Esc closes the overlay.

pub mod scan;
pub mod state;

use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::ui::editor::FileBuffer;

pub use state::{FuzzyEntry, FuzzyFindState};

/// Opens the fuzzy finder over the workspace rooted at `cwd`.
pub fn open_fuzzy_find(app: &mut App, cwd: &Path) {
    let candidates = scan::scan_workspace(cwd);
    let count = candidates.len();
    app.state.fuzzy_find = Some(FuzzyFindState::new(candidates));
    app.state.status =
        format!("Fuzzy open: {count} files — type to filter, Enter opens, Esc cancels");
}

/// Confirms the current selection, opening it in the editor.
pub fn confirm_fuzzy_find(app: &mut App, cwd: &Path) {
    let chosen = app
        .state
        .fuzzy_find
        .as_ref()
        .and_then(|f| f.current())
        .map(|e| e.path.clone());
    app.state.fuzzy_find = None;
    let Some(path) = chosen else { return };
    open_path_in_editor(app, cwd, &path);
}

/// Opens `path` in the in-TUI editor, switching to the Editor view.
pub fn open_path_in_editor(app: &mut App, cwd: &Path, path: &Path) {
    let label = path.strip_prefix(cwd).unwrap_or(path).display().to_string();
    match FileBuffer::open(path) {
        Ok(buf) => {
            app.state.editor = Some(buf);
            app.state.editor_scroll = 0;
            app.state.editor_hscroll = 0;
            app.state.set_view_mode(ViewMode::Editor);
            app.state.status = format!("Editing {label} — Ctrl+S save, Esc close");
        }
        Err(e) => app.state.status = format!("Cannot open {label}: {e}"),
    }
}
