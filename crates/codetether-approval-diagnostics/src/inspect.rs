//! Run diagnostics for each reconstructed proposed file.
use crate::report::unavailable;
use codetether_approval_queue::ApprovalReport;
use codetether_lsp_core::{
    LspActionResult,
    manager::{LspManager, ManagedClient},
};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
/// Inspect a proposed mutation using the caller's language-server manager.
/// # Arguments
/// * `manager` - Manager retaining caller-specific client initialization.
/// * `root` - Workspace root for proposed file reconstruction.
/// * `tool` - File-mutation tool name.
/// * `arguments` - Original tool arguments.
/// # Returns
/// Clean, issues or unavailable, retaining up to three diagnostics.
/// # Examples
/// ```no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(manager: std::sync::Arc<codetether_lsp_core::manager::LspManager<C>>) {
/// let _ = codetether_approval_diagnostics::inspect(manager, &std::path::PathBuf::from("."), "write", &serde_json::json!({"path":"note", "content":"hello"})).await;
/// # }
/// ```

pub async fn inspect<C: ManagedClient>(
    manager: Arc<LspManager<C>>,
    root: &PathBuf,
    tool: &str,
    arguments: &Value,
) -> ApprovalReport {
    let files = match codetether_proposed_content::contents(root, tool, arguments) {
        Ok(files) => files,
        Err(error) => return unavailable(error.to_string()),
    };
    let mut messages = Vec::new();
    for (path, content) in files {
        let client = match manager.get_client_for_file(&path).await {
            Ok(client) => client,
            Err(error) => return unavailable(error.to_string()),
        };
        match client.diagnostics_for_content(&path, &content).await {
            Ok(LspActionResult::Diagnostics { diagnostics }) => {
                messages.extend(
                    diagnostics
                        .into_iter()
                        .map(|item| crate::report::diagnostic(&path, item)),
                );
            }
            Ok(_) => return unavailable("language server returned an unexpected result".into()),
            Err(error) => return unavailable(error.to_string()),
        }
    }
    crate::report::finish(messages)
}
