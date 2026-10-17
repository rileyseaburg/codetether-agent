//! Query workspace symbols using the path language or Rust fallback.
use super::args::get_file_path_arg;
use anyhow::Result;
use codetether_lsp_core::{
    detect_language_from_path,
    manager::{LspManager, ManagedClient},
};
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn execute<C: ManagedClient>(
    manager: &LspManager<C>,
    args: &Value,
) -> Result<ToolResult> {
    let query = args["query"].as_str().unwrap_or("");
    let language = get_file_path_arg(&args).and_then(detect_language_from_path);

    let client = if let Some(lang) = language {
        manager.get_client(lang).await?
    } else {
        manager.get_client("rust").await?
    };

    let result = client.workspace_symbols(query).await?;
    super::format::format_result(result)
}
