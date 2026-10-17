//! diagnostics LSP request.
use anyhow::Result;
use codetether_lsp_core::{
    LspActionResult,
    manager::{LspManager, ManagedClient},
};
use std::path::Path;
pub(crate) async fn run<C: ManagedClient>(
    client: &C,
    manager: &LspManager<C>,
    path: &Path,
) -> Result<LspActionResult> {
    let mut result = client.diagnostics(path).await?;
    // Merge diagnostics from any applicable linter servers
    let linter_diags = manager.linter_diagnostics(path).await;
    if !linter_diags.is_empty() {
        if let LspActionResult::Diagnostics {
            ref mut diagnostics,
        } = result
        {
            diagnostics.extend(linter_diags);
        }
    }
    Ok(result)
}
