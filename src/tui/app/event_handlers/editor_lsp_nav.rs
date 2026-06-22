//! Navigates the editor to an LSP [`LocationInfo`] (go-to-definition target).
//!
//! If the location is in the currently open file, only the cursor moves.
//! Otherwise the target file is opened into a fresh [`FileBuffer`] first.
//! Pure state mutation; the async request lives in [`super::editor_lsp`].

use crate::lsp::{LocationInfo, uri_to_path};
use crate::tui::app::state::App;
use crate::tui::ui::editor::FileBuffer;

/// Moves the editor cursor (and file, if needed) to `loc`'s start position.
pub(super) fn navigate(app: &mut App, loc: &LocationInfo) {
    let target = uri_to_path(&loc.uri);
    let line = loc.range.start.line as usize;
    let col = loc.range.start.character as usize;

    let same_file = app
        .state
        .editor
        .as_ref()
        .is_some_and(|b| b.path() == target);

    if !same_file {
        match FileBuffer::open(&target) {
            Ok(buf) => {
                app.state.editor = Some(buf);
                app.state.editor_scroll = 0;
                app.state.editor_hscroll = 0;
            }
            Err(e) => {
                app.state.status = format!("Cannot open {}: {e}", target.display());
                return;
            }
        }
    }

    if let Some(buf) = app.state.editor.as_mut() {
        buf.set_cursor(line, col);
        app.state.status = format!("→ {}:{}", target.display(), line + 1);
    }
}
