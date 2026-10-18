//! Bound the interactive wait without cancelling an initializing language server.

use anyhow::{Context, Result};
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use codetether_lsp_core::{LspActionResult, detect_language_from_path};
use std::{path::Path, sync::Arc};
use tokio::time::Instant;

/// Start diagnostics for one proposed file with a bounded interactive wait.
///
/// # Arguments
/// `manager` retains the client; `workspace` scopes backoff; `path` and `content`
/// identify the proposal; `deadline` bounds only the caller's wait.
///
/// # Returns
/// The diagnostic result if available before the deadline.
///
/// # Errors
/// Reports unknown languages, pending checks, elapsed deadlines or server failures.
/// Background diagnostics continue when only the interactive deadline elapses.
///
/// # Examples
/// ```no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # manager: &std::sync::Arc<codetether_lsp_core::manager::LspManager<C>>) {
/// let root = std::path::Path::new(".");
/// let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
/// let _result = codetether_lsp_preflight::check::run(
///     manager, root, &root.join("file.ts"), "export {};", deadline
/// ).await;
/// # }
/// ```
pub async fn run<C: ManagedClient>(
    manager: &Arc<LspManager<C>>,
    workspace: &Path,
    path: &Path,
    content: &str,
    deadline: Instant,
) -> Result<LspActionResult> {
    let language = detect_language_from_path(path.to_string_lossy().as_ref())
        .context("No language detected for automatic preflight")?;
    super::cooldown::begin(workspace, language)?;
    let job = tokio::spawn(super::background::run(
        manager.clone(),
        workspace.to_path_buf(),
        path.to_path_buf(),
        content.to_string(),
        language.to_string(),
    ));
    tokio::time::timeout_at(deadline, job)
        .await
        .map_err(|_| super::budget::Expired {
            path: path.to_path_buf(),
        })?
        .context("LSP background check task failed")?
}
