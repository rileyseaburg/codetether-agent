//! Click-to-go-to-definition inside the in-TUI editor.
//!
//! Maps a left-click cell to the editor's logical `(line, col)` via
//! [`cell_to_cursor`], moves the cursor there, then issues an async
//! go-to-definition. Returns `true` when the click was handled in editor mode.

use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::ui::editor::cursor_pos::cell_to_cursor;

/// Handles a left-click in the editor: place cursor and go to definition.
pub(crate) async fn editor_click(app: &mut App, cwd: &Path, col: u16, row: u16) -> bool {
    if app.state.view_mode != ViewMode::Editor {
        return false;
    }
    let area = app.state.editor_lsp.area;
    let (scroll, hscroll) = (app.state.editor_scroll, app.state.editor_hscroll);
    let Some(buf) = app.state.editor.as_ref() else {
        return false;
    };
    let Some((line, column)) = cell_to_cursor(buf.backend(), area, scroll, hscroll, col, row)
    else {
        return false;
    };
    if let Some(buf) = app.state.editor.as_mut() {
        buf.set_cursor(line, column);
    }
    super::editor_lsp::goto_definition(app, cwd).await;
    true
}
