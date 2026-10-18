//! Prefer non-empty diagnostics over symbol recovery.
use codetether_context_policy::transcript::truncate_with_ellipsis;
use codetether_tool_core::Tool;
use serde_json::json;
pub(super) async fn append(
    lsp_tool: &dyn Tool,
    path: &str,
    max_chars: usize,
    sections: &mut Vec<String>,
) -> bool {
    let diagnostics_args = json!({
        "action": "diagnostics",
        "file_path": path,
    });

    match lsp_tool.execute(diagnostics_args).await {
        Ok(result) if result.success => {
            let output = result.output.trim();
            if !output.is_empty() && output != "No diagnostics found" {
                sections.push(format!(
                    "File: {}\n{}",
                    path,
                    truncate_with_ellipsis(output, max_chars)
                ));
                return true;
            }
        }
        Ok(result) => {
            tracing::debug!(
                file = %path,
                output = %truncate_with_ellipsis(&result.output, 200),
                "Proactive LSP diagnostics skipped file due to unsuccessful result"
            );
        }
        Err(e) => {
            tracing::debug!(file = %path, error = %e, "Proactive LSP diagnostics prefetch failed");
        }
    }

    false
}
