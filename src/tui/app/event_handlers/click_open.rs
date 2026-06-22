//! Left-click-to-open dispatch for the chat panel.
//!
//! Maps a click to the chat line under the cursor, extracts a file path from
//! that line, and opens it in the in-TUI editor.

use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::click_path::path_at;

/// Open the file referenced by the chat line under the click, if any.
///
/// No-op outside the Chat view or when the clicked line has no resolvable
/// file path. Opens the resolved file in the in-TUI editor.
pub(super) fn click_open_path(app: &mut App, cwd: &Path, col: u16, row: u16) {
    if app.state.view_mode != ViewMode::Chat {
        return;
    }
    let hit = &app.state.chat_hit;
    let Some(line) = hit.line_at(col, row) else {
        return;
    };
    let rel = col.saturating_sub(hit.rect.x) as usize;
    let Some(path) = path_at(line, rel, cwd) else {
        return;
    };
    crate::tui::app::fuzzy_find::open_path_in_editor(app, cwd, &path);
}
