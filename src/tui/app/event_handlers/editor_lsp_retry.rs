//! Apply shared definition lookup outcomes to application state.
use crate::{lsp::client::LspClient, tui::app::state::App};
use std::path::Path;
pub(super) async fn resolve_definition(
    app: &mut App,
    client: &LspClient,
    path: &Path,
    line: u32,
    col: u32,
) {
    match codetether_editor_lsp::resolve_definition(client, path, line, col).await {
        codetether_editor_lsp::DefinitionOutcome::Location(location) => {
            super::editor_lsp_nav::navigate(app, &location)
        }
        codetether_editor_lsp::DefinitionOutcome::Status(status) => app.state.status = status,
    }
    app.state.needs_redraw = true;
}
