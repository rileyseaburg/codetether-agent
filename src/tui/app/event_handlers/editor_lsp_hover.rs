//! Forward frontend actions to shared editor interaction policy.
use crate::tui::app::state::App;
use std::path::Path;
pub(crate) async fn hover(app: &mut App, cwd: &Path) {
    codetether_editor_lsp::interactive::hover(&mut super::editor_lsp::controls(app), cwd).await
}
