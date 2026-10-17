//! manager documents operations for language server clients.

use super::*;

impl<C: ManagedClient> LspManager<C> {
    /// Close a document across all relevant clients.
    #[allow(dead_code)]
    pub async fn close_document(&self, path: &Path) -> Result<()> {
        if let Ok(client) = self.get_client_for_file(path).await {
            client.close_document(path).await?;
        }
        Ok(())
    }

    /// Notify clients of a document change.
    #[allow(dead_code)]
    pub async fn change_document(&self, path: &Path, content: &str) -> Result<()> {
        if let Ok(client) = self.get_client_for_file(path).await {
            client.change_document(path, content).await?;
        }
        Ok(())
    }
}
