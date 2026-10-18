//! Click-to-go-to-definition inside the in-TUI editor.
//!
//! Maps a left-click cell to the editor's logical `(line, col)` via
//! [`cell_to_cursor`], moves the cursor there, then issues an async
//! go-to-definition. Returns `true` when the click was handled in editor mode.

use std::path::Path;

use super::Controls;
use codetether_lsp_core::manager::ManagedClient;

use codetether_editor::cursor_pos::cell_to_cursor;

/// Handles a left-click in the editor: place cursor and go to definition.
///
/// # Arguments
/// * `state` — Borrowed frontend editor state and mode guards.
/// * `cwd` — Workspace root used when creating the language-server manager.
/// * `col` and `row` — Terminal cell coordinates.
/// # Returns
/// Whether the click was handled by the editor.
/// # Examples
/// ```rust,no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # state: &mut codetether_editor_lsp::interactive::Controls<'_, C>,
/// # ) {
/// let _ = codetether_editor_lsp::interactive::editor_click(state, std::path::Path::new("."), 1, 1).await;
/// # }
/// ```
pub async fn editor_click(
    state: &mut Controls<'_, impl ManagedClient>,
    cwd: &Path,
    col: u16,
    row: u16,
) -> bool {
    if !state.in_editor {
        return false;
    }
    let area = state.editor_lsp.area;
    let (scroll, hscroll) = (*state.editor_scroll, *state.editor_hscroll);
    let Some(buf) = state.editor.as_ref() else {
        return false;
    };
    let Some((line, column)) = cell_to_cursor(buf.backend(), area, scroll, hscroll, col, row)
    else {
        return false;
    };
    if let Some(buf) = state.editor.as_mut() {
        buf.set_cursor(line, column);
    }
    if !state.goal_editing {
        super::definition::goto_definition(state, cwd).await;
    }
    true
}
