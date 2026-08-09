//! Fresh language-server diagnostics for disk and proposed document content.

use std::{path::Path, time::Duration};

use anyhow::Result;
use tracing::debug;

use super::{DiagnosticInfo, LspActionResult, client::LspClient, path_to_uri};

impl LspClient {
    /// Requests fresh diagnostics for the current on-disk document.
    pub async fn diagnostics(&self, path: &Path) -> Result<LspActionResult> {
        let content = tokio::fs::read_to_string(path).await.unwrap_or_default();
        self.diagnostics_for_content(path, &content).await
    }

    /// Requests real LSP diagnostics for proposed, not-yet-written content.
    pub async fn diagnostics_for_content(
        &self,
        path: &Path,
        content: &str,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        let already_open = self.open_documents.read().await.contains_key(&uri);
        let baseline = self.transport.diagnostics_publish_seq();
        self.transport.invalidate_diagnostics(&uri).await;
        let synced = if already_open {
            self.change_document(path, content).await
        } else {
            self.open_document(path, content).await
        };
        if let Err(error) = synced {
            debug!(path = %path.display(), %error, "LSP document sync failed");
        }
        let _ = self
            .transport
            .wait_for_publish_after(baseline, Duration::from_millis(1500))
            .await;
        let snapshot = self.transport.diagnostics_snapshot().await;
        let diagnostics = snapshot
            .get(&uri)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|value| DiagnosticInfo::from((uri.clone(), value)))
            .collect();
        Ok(LspActionResult::Diagnostics { diagnostics })
    }
}
