//! Inspect proposed files within one shared interactive preflight budget.

use codetether_lsp_core::manager::{LspManager, ManagedClient};
use codetether_tool_core::ToolResult;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::time::{Duration, Instant};

/// Inspect proposed files within one shared five-second interactive budget.
///
/// # Arguments
/// `workspace` scopes server backoff; `tool` labels failures; `files` contains
/// proposed contents; `manager` owns the configured clients.
///
/// # Returns
/// An optional diagnostic rejection and all availability warnings.
///
/// # Examples
/// ```no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # manager: std::sync::Arc<codetether_lsp_core::manager::LspManager<C>>) {
/// let result = codetether_lsp_preflight::scan::run(
///     std::path::Path::new("."), "write", vec![], manager
/// ).await;
/// assert!(result.0.is_none() && result.1.is_empty());
/// # }
/// ```
pub async fn run<C: ManagedClient>(
    workspace: &Path,
    tool: &str,
    files: Vec<(PathBuf, String)>,
    manager: Arc<LspManager<C>>,
) -> (Option<ToolResult>, Vec<String>) {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    for (path, content) in files {
        let (errors, warning) =
            super::scan_file::inspect(workspace, &path, &content, &manager, deadline).await;
        issues.extend(errors);
        if let Some(warning) = warning {
            warnings.push(warning);
        }
    }
    (
        (!issues.is_empty()).then(|| super::report::diagnostics(tool, issues)),
        warnings,
    )
}
