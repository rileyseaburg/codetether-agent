//! Repository run delete operations.

use super::OkrRepository;
use anyhow::Result;
use tokio::fs;
use uuid::Uuid;
impl OkrRepository {
    /// Delete a run
    ///
    /// # Errors
    /// Returns an error if an existing file cannot be removed.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.delete_run(uuid::Uuid::nil()).await.is_ok());
    /// # });
    /// ```
    pub async fn delete_run(&self, id: Uuid) -> Result<bool> {
        let path = self.run_path(id);

        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).await?;

        // Remove from cache
        let mut cache = self.cache.write().await;
        cache.runs.retain(|r| r.id != id);

        tracing::info!(run_id = %id, "OKR run deleted");
        Ok(true)
    }
}
