//! Finish diagnostics after the interactive caller returns; recover only real failures.

use crate::lsp::{LspActionResult, LspManager};
use anyhow::{Context, Result};
use std::{path::PathBuf, sync::Arc, time::Duration};

pub(super) async fn run(
    manager: Arc<LspManager>,
    workspace: PathBuf,
    path: PathBuf,
    content: String,
    language: String,
) -> Result<LspActionResult> {
    let result = tokio::time::timeout(Duration::from_secs(60), async {
        let client = manager.get_client_for_file(&path).await?;
        client.diagnostics_for_content(&path, &content).await
    })
    .await
    .context("LSP background check exceeded its hard 60s deadline")
    .and_then(std::convert::identity);
    match &result {
        Ok(result) => {
            super::cooldown::clear(&workspace, &language);
            let error_count = match result {
                LspActionResult::Diagnostics { diagnostics } => diagnostics
                    .iter()
                    .filter(|item| item.severity.as_deref() == Some("error"))
                    .count(),
                _ => 0,
            };
            tracing::info!(path = %path.display(), error_count, "Pre-approval LSP background check finished; server retained");
        }
        Err(error) => {
            super::cooldown::record(
                &workspace,
                &language,
                &format!("{}: {error:#}", path.display()),
            );
            manager.invalidate_client(&language).await;
            tracing::warn!(path = %path.display(), %error, "Pre-approval LSP background check failed; server evicted");
        }
    }
    result
}
