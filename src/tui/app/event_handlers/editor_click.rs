//! Forward frontend actions to shared editor interaction policy.
use crate::tui::app::state::App;
use std::path::Path;
pub(crate) async fn editor_click(app: &mut App, cwd: &Path, col: u16, row: u16) -> bool {
    codetether_editor_lsp::interactive::editor_click(
        &mut super::editor_lsp::controls(app),
        cwd,
        col,
        row,
    )
    .await
}
