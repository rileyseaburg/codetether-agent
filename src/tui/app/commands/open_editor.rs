//! Opens a file in the in-TUI editor view.
//!
//! Resolves the requested path against the workspace cwd, loads it into a
//! [`FileBuffer`](crate::tui::ui::editor::FileBuffer), and switches the app to
//! [`ViewMode::Editor`](crate::tui::models::ViewMode::Editor).

use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::ui::editor::FileBuffer;

/// Opens `arg` (relative to `cwd`) in the editor, or reports an error.
pub(super) fn open_editor(app: &mut App, cwd: &Path, arg: &str) {
    if arg.is_empty() {
        app.state.status = "Usage: /edit <path>".to_string();
        return;
    }
    let path = cwd.join(arg);
    match FileBuffer::open(&path) {
        Ok(buf) => {
            app.state.editor = Some(buf);
            app.state.editor_scroll = 0;
            app.state.editor_hscroll = 0;
            app.state.set_view_mode(ViewMode::Editor);
            app.state.status = format!("Editing {arg} — Ctrl+S save, Esc close");
        }
        Err(e) => app.state.status = format!("Cannot open {arg}: {e}"),
    }
}
