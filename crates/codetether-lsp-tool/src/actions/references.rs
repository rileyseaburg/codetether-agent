//! references LSP request.
use anyhow::Result;
use codetether_lsp_core::{LspActionResult, manager::ManagedClient};
use serde_json::Value;
use std::path::Path;
pub(crate) async fn run<C: ManagedClient>(
    client: &C,
    path: &Path,
    args: &Value,
    line: Option<u32>,
    column: Option<u32>,
) -> Result<LspActionResult> {
    let include_decl = args["include_declaration"].as_bool().unwrap_or(true);
    Ok(client
        .find_references(
            path,
            line.expect("line required"),
            column.expect("column required"),
            include_decl,
        )
        .await?)
}
