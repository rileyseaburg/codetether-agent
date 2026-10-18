//! Workspace-symbol request and textual response decoding.
use codetether_lsp_tool::{Host, LspTool};
use codetether_symbol_search::{SymbolEntry, parse};
use codetether_tool_core::Tool;
use std::path::Path;

pub(crate) async fn query_language<H: Host>(
    file: &Path,
    query: &str,
) -> anyhow::Result<Vec<SymbolEntry>> {
    let output = LspTool::<H>::new()
        .execute(serde_json::json!({
            "action": "workspaceSymbol", "query": query, "file_path": file,
        }))
        .await?;
    if !output.success {
        anyhow::bail!(output.output);
    }
    Ok(parse::output(&output.output))
}
