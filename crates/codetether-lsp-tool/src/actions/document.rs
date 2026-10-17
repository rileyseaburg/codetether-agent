//! document LSP request.
use anyhow::Result;
use codetether_lsp_core::{LspActionResult, manager::ManagedClient};
use std::path::Path;
pub(crate) async fn run<C: ManagedClient>(client: &C, path: &Path) -> Result<LspActionResult> {
    Ok(client.document_symbols(path).await?)
}
