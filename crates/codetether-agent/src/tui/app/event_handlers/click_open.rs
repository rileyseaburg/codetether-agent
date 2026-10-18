//! Chat click binding to shared file-reference actions.
use crate::tui::app::state::App;
use std::path::Path;
pub(super) fn click_open_path(app: &mut App, cwd: &Path, col: u16, row: u16) {
    codetether_chat_files::click_open_path(
        &mut super::shared_file_open::host::Native(app),
        cwd,
        col,
        row,
    );
}
