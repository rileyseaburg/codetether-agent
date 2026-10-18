//! Forward frontend actions to shared editor interaction policy.
use crate::tui::app::state::App;
use std::path::Path;
pub(crate) async fn handle_editor_lsp_key(
    app: &mut App,
    cwd: &Path,
    key: crossterm::event::KeyEvent,
) -> bool {
    codetether_editor_lsp::interactive::handle_editor_lsp_key(
        &mut super::editor_lsp::controls(app),
        cwd,
        key,
    )
    .await
}
