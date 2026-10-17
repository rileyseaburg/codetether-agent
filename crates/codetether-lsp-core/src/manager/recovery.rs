//! Evict unavailable language servers so the next diagnostic attempt starts fresh.

use super::{LspManager, ManagedClient};

impl<C: ManagedClient> LspManager<C> {
    /// Remove the cached primary server after an automatic diagnostic failure.
    ///
    /// Try graceful shutdown first so servers can stop their worker processes.
    /// Other in-flight operations may fail and retry against a fresh client.
    pub async fn invalidate_client(&self, language: &str) {
        let client = self.clients.write().await.remove(language);
        if let Some(client) = client {
            let _ = tokio::time::timeout(std::time::Duration::from_millis(250), client.shutdown())
                .await;
        }
    }
}
