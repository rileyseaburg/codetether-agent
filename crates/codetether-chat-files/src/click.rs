//! Left-click-to-open dispatch for the chat panel.
//!
//! Maps a click to the chat line under the cursor, extracts a file path from
//! that line, and opens it in the in-TUI editor.

use std::path::Path;

use crate::Host;

use super::path::path_at;

/// Open the file referenced by the chat line under the click, if any.
///
/// No-op outside the Chat view or when the clicked line has no resolvable
/// file path. Opens the resolved file in the in-TUI editor.
///
/// # Arguments
/// * `host` — Frontend state and editor operations.
/// * `cwd` — Base directory for relative paths.
/// * `col` and `row` — Terminal coordinates of the click.
/// # Returns
/// Updates the host only when the click resolves to a file in chat.
/// # Examples
/// ```rust,no_run
/// fn open(host: &mut impl codetether_chat_files::Host) {
///     codetether_chat_files::click_open_path(host, std::path::Path::new("."), 1, 1);
/// }
/// ```
pub fn click_open_path(host: &mut impl Host, cwd: &Path, col: u16, row: u16) {
    if !host.in_chat() {
        return;
    }
    let hit = host.chat_hit();
    let Some(line) = hit.line_at(col, row) else {
        return;
    };
    let rel = col.saturating_sub(hit.rect.x) as usize;
    let Some(path) = path_at(line, rel, cwd) else {
        return;
    };
    host.open_editor(cwd, &path);
}
