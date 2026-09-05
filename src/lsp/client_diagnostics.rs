//! Fresh language-server diagnostics for disk and proposed document content.

use std::path::Path;

use anyhow::Result;

use super::{DiagnosticInfo, LspActionResult, client::LspClient, path_to_uri};

#[path = "client_diagnostics_wait.rs"]
mod publication;

impl LspClient {
    /// Requests fresh diagnostics for the current on-disk document.
    pub async fn diagnostics(&self, path: &Path) -> Result<LspActionResult> {
        let content = tokio::fs::read_to_string(path).await.unwrap_or_default();
        self.diagnostics_for_content(path, &content).await
    }

    /// Requests real LSP diagnostics for proposed, not-yet-written content.
    ///
    /// # Errors
    /// Returns an error if document synchronization fails or the server does not
    /// publish diagnostics for this document within its configured timeout.
    pub async fn diagnostics_for_content(
        &self,
        path: &Path,
        content: &str,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        let already_open = self.open_documents.read().await.contains_key(&uri);
        self.transport.invalidate_diagnostics(&uri).await;
        if already_open {
            self.change_document(path, content).await?;
        } else {
            self.open_document(path, content).await?;
        }
        let diagnostics = publication::wait(
            || self.transport.diagnostics_snapshot(),
            &uri,
            std::time::Duration::from_millis(self.config.timeout_ms),
        )
        .await?
        .into_iter()
        .map(|value| DiagnosticInfo::from((uri.clone(), value)))
        .collect();
        Ok(LspActionResult::Diagnostics { diagnostics })
    }
}
