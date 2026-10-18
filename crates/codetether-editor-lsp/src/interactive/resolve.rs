//! Apply shared definition lookup outcomes to application state.
use super::Controls;
use codetether_lsp_core::client::LspClient;
use codetether_lsp_core::manager::ManagedClient;
use std::path::Path;
pub(super) async fn resolve_definition(
    state: &mut Controls<'_, impl ManagedClient>,
    client: &LspClient,
    path: &Path,
    line: u32,
    col: u32,
) {
    match crate::resolve_definition(client, path, line, col).await {
        crate::DefinitionOutcome::Location(location) => super::navigate::navigate(state, &location),
        crate::DefinitionOutcome::Status(status) => *state.status = status,
    }
    *state.needs_redraw = true;
}
