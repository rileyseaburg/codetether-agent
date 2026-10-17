//! Repository okr list operations.

use super::OkrRepository;
use crate::Okr;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    /// List all OKRs
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.list_okrs().await.is_ok());
    /// # });
    /// ```
    pub async fn list_okrs(&self) -> Result<Vec<Okr>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if !cache.okrs.is_empty() {
                return Ok(cache.okrs.clone());
            }
        }

        // Load from disk
        let dir = self.okrs_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut okrs = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json")
                && let Ok(data) = fs::read_to_string(&path).await
                && let Ok(okr) = serde_json::from_str::<Okr>(&data)
            {
                okrs.push(okr);
            }
        }

        // Update cache
        let mut cache = self.cache.write().await;
        cache.okrs = okrs.clone();

        Ok(okrs)
    }
}
