//! implementation LSP request.
use anyhow::Result;
use codetether_lsp_core::{LspActionResult, manager::ManagedClient};
use std::path::Path;
pub(crate) async fn run<C: ManagedClient>(
    client: &C,
    path: &Path,
    line: Option<u32>,
    column: Option<u32>,
) -> Result<LspActionResult> {
    Ok(client
        .go_to_implementation(
            path,
            line.expect("line required"),
            column.expect("column required"),
        )
        .await?)
}
