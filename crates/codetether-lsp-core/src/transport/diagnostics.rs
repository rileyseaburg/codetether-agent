//! diagnostics responsibility for the stdio LSP transport.

use super::*;

impl LspTransport {
    /// Return the last diagnostics published by the language server.
    pub async fn diagnostics_snapshot(&self) -> HashMap<String, Vec<lsp_types::Diagnostic>> {
        self.diagnostics.read().await.clone()
    }

    /// Current publish sequence counter. Increments every time the language
    /// server publishes diagnostics for any URI. Callers can read this before
    /// a `textDocument/didChange` and then wait via
    /// [`Self::wait_for_publish_after`] for the server to republish.
    pub fn diagnostics_publish_seq(&self) -> u64 {
        self.diag_publish_seq.load(Ordering::SeqCst)
    }

    /// Remove any cached diagnostics for `uri`. Useful before a didChange so
    /// stale entries can't be returned while the server recomputes.
    pub async fn invalidate_diagnostics(&self, uri: &str) {
        self.diagnostics.write().await.remove(uri);
    }

    /// Wait until the server publishes diagnostics with a sequence greater
    /// than `baseline`, or the timeout elapses. Returns `true` if a new
    /// publication arrived in time.
    pub async fn wait_for_publish_after(
        &self,
        baseline: u64,
        timeout: std::time::Duration,
    ) -> bool {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            if self.diag_publish_seq.load(Ordering::SeqCst) > baseline {
                return true;
            }
            if std::time::Instant::now() >= deadline {
                return false;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}
