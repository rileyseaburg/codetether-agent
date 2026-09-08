//! Bound the interactive wait without cancelling an initializing language server.

use crate::lsp::{LspActionResult, LspManager, detect_language_from_path};
use anyhow::{Context, Result};
use std::{path::Path, sync::Arc};
use tokio::time::Instant;

pub(super) async fn run(
    manager: &Arc<LspManager>,
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
