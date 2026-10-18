//! Recover document symbols when diagnostics did not provide context.
use codetether_context_policy::transcript::truncate_with_ellipsis;
use codetether_tool_core::Tool;
use serde_json::json;
pub(super) async fn append(
    lsp_tool: &dyn Tool,
    path: &str,
    max_chars: usize,
    sections: &mut Vec<String>,
) {
    let symbol_args = json!({
        "action": "documentSymbol",
        "file_path": path,
    });
    match lsp_tool.execute(symbol_args).await {
        Ok(result) if result.success => {
            sections.push(format!(
                "File: {}\n{}",
                path,
                truncate_with_ellipsis(&result.output, max_chars / 2)
            ));
        }
        Ok(result) => {
            tracing::debug!(
                file = %path,
                output = %truncate_with_ellipsis(&result.output, 200),
                "Proactive LSP symbol recovery skipped file due to unsuccessful result"
            );
        }
        Err(e) => {
            tracing::debug!(file = %path, error = %e, "Proactive LSP symbol recovery failed");
        }
    }
}
